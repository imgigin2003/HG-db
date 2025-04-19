import os
import toml
import subprocess
import streamlit as st
from pathlib import Path
import pandas as pd
import json
from collections import Counter

def get_project_root():
    """Helper function to get the project root relative to main.py's location"""
    return Path(__file__).parent.parent.parent # Goes up from py_scripts to hgdb_core

def load_db_path():
    """Load db_path from Config.toml with correct path"""
    config_file = get_project_root() / "Config.toml"
    default_path = get_project_root() / "mydbs" / "rocksdb"
    if config_file.exists():
        try:
            with open(config_file, "r") as f:
                config_data = toml.load(f)
            return config_data.get("db_path", default_path)
        except Exception as e:
            st.error(f"Failed to load '{config_file}': {str(e)}. Using default path.")
    return ""

def update_db_path(new_path):
    """Update the db_path in Config.toml with correct path"""
    config_file = get_project_root() / "Config.toml"
    try:
        with open(config_file, "r") as f:
            config_data = toml.load(f)
        
        config_data["db_path"] = new_path
        
        with open(config_file, "w") as f:
            toml.dump(config_data, f)
        return True
    except Exception as e:
        st.error(f"Error updating Config.toml: {e}")
        return False

def get_python_version():
    """Get Python version (e.g., '3.13') using subprocess"""
    try:
        result = subprocess.run(
            ['python3', '--version'],
            capture_output=True,
            text=True,
            check=True
        )
        version = result.stdout.strip().split()[1]  # Gets "3.13.0" from "Python 3.13.0"
        return '.'.join(version.split('.')[:2])  # Returns "3.13"
    except Exception as e:
        st.error(f"Error detecting Python version: {e}")
        return None


def get_python_library_path():
    """Get the correct Python library path without appending the version"""
    try:
        # First try to get the lib directory path
        result = subprocess.run(
            ['python3', '-c', "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))"],
            capture_output=True,
            text=True,
            check=True
        )
        lib_path = result.stdout.strip()
        
        # Verify this is the correct path by checking for Python library files
        if lib_path and os.path.exists(lib_path):
            # Check for common Python library file patterns
            lib_files = os.listdir(lib_path)
            if any(f.startswith('libpython3') for f in lib_files):
                return lib_path
        
        # Fallback to stdlib path if LIBDIR doesn't work
        result = subprocess.run(
            ['python3', '-c', "import sysconfig; print(sysconfig.get_path('stdlib'))"],
            capture_output=True,
            text=True,
            check=True
        )
        stdlib_path = result.stdout.strip()
        if stdlib_path:
            # Go up one level from site-packages to lib directory
            return os.path.dirname(stdlib_path)
            
        return None
    except subprocess.CalledProcessError as e:
        st.error(f"Error detecting Python library path: {e}")
        return None

def update_build_rs(python_path, python_version):
    """Update build.rs with correct paths without appending python version"""
    build_rs_path = get_project_root() / "hg_app" / "build.rs"
    
    try:
        with open(build_rs_path, "w") as f:
            f.write(f"""fn main() {{
    // Link the Python {python_version} library
    println!("cargo:rustc-link-lib=python{python_version}");

    // Specify the search path for Python libraries (without pythonX.Y suffix)
    println!("cargo:rustc-link-search=native={python_path}");

    // Ensure Rust rebuilds when Python version changes
    println!("cargo:rerun-if-env-changed=PYTHON_SYS_EXECUTABLE");
}}""")
        return True
    except Exception as e:
        st.error(f"Error updating build.rs: {e}")
        return False
    
    
def get_upload_dir():
    """Get the upload directory path based on configured db_path"""
    db_path = load_db_path()
    if not db_path:
        return None
    return os.path.join(db_path, "hg_uploads")

def ensure_upload_dir():
    """Ensure upload directory exists"""
    upload_dir = get_upload_dir()
    if upload_dir and not os.path.exists(upload_dir):
        os.makedirs(upload_dir, exist_ok=True)
    return upload_dir

def display_properties(H, graph_type):
    st.write(f"### {graph_type} Properties")
    nodes_list = list(H.nodes())
    edges_list = list(H.edges())
    st.write(f"Nodes: {nodes_list}")
    st.write(f"Number of nodes: {len(nodes_list)}")
    st.write(f"Edges: {edges_list}")
    st.write(f"Number of edges: {len(edges_list)}")

def display_table(graph, graph_type):
    st.write(f"### {graph_type} Table")
    edge_data = []
    if graph_type == "HyperGraph":
        edge_data = [
            {"Edge": edge, "Nodes": ", ".join(sorted(data["nodes"])), 
            "Traversable": data["traversable"]}
            for edge, data in graph.items()
        ]
    else:  # Dual HyperGraph
        edge_data = [
            {"Node (Original Edge)": node, "Hyperedges (Original Nodes)": ", ".join(sorted(edges))}
            for node, edges in graph.items()
        ]
    df = pd.DataFrame(edge_data)
    st.dataframe(df)

def load_atoms_data():
    FILE_PATH = get_project_root() / "resources" / "Atoms.json"
    try:
        with open(FILE_PATH, "r") as f:
            atoms_data = json.load(f)
        
        hyperedges = {}
        for atom in atoms_data:
            symbol = atom['properties'][0]['symbol']
            hyperedges[symbol] = {  # Using symbol as both key and node
                "nodes": {symbol},
                "layer": int(atom['properties'][0].get('layer', 0)),
                "traversable": False,  # Atoms are non-traversable
                "directed": False,
                "head": {symbol},
                "tail": set(),
                "type": "atom"
            }
        return hyperedges
    except Exception as e:
        st.error(f"Error loading Atoms data: {e}")
        return None
    
def parse_formula(formula):
    # Parse molecular formula (e.g., 'C6H12O6' -> {'C': 6, 'H': 12, 'O': 6})
    elements = {}
    current_element = ''
    current_count = ''
    for char in formula:
        if char.isupper():
            if current_element:
                count = int(current_count) if current_count else 1
                elements[current_element] = elements.get(current_element, 0) + count
            current_element = char
            current_count = ''
        elif char.islower():
            current_element += char
        elif char.isdigit():
            current_count += char
    if current_element:
        count = int(current_count) if current_count else 1
        elements[current_element] = elements.get(current_element, 0) + count
    return elements

def load_mols_data():
    FILE_PATH = get_project_root() / "resources" / "Mols.json"
    try:
        with open(FILE_PATH, "r") as f:
            molecules = json.load(f)
        
        hyperedges = {}
        for mol in molecules:
            formula = mol['properties'][0]['molecular_formula']
            atoms = parse_formula(formula)
            nodes = set(atoms.keys())  # Use individual elements as nodes
            # Assign layer based on roles
            # In load_mols_data
            layer = int(mol['properties'][0].get('layer', 0))  # Use layer from JSON, default to 0
            hyperedges[mol['id']] = {
                "nodes": nodes,
                "layer": layer,
                "traversable": mol['traversable'].lower() == "true",
                "directed": mol['directed'].lower() == "true",
                "head": nodes,  # All atoms as head nodes
                "tail": set(),
                "type": "molecule"
            }
        return hyperedges
    except Exception as e:
        st.error(f"Error loading Molecules data: {e}")
        return None

def display_atom_properties():
    try:
        FILE_PATH = get_project_root() / "resources" / "Atoms.json"
        with open(FILE_PATH, "r") as f:
            atoms = json.load(f)
        
        properties = []
        for atom in atoms:
            props = atom['properties'][0]
            properties.append({
                'Name': atom['name'],
                'Symbol': props['symbol'],
                'Atomic Number': props['atomic_number'],
                'Weight': props['atomic_weight'],
                'Electron Config': props['electron_configuration'],
                'Layer': props.get('layer', 0)
            })
        
        df = pd.DataFrame(properties)
        st.dataframe(df)
        
        # Additional metrics display
        col1, col2, col3 = st.columns(3)
        with col1:
            st.metric("Total Atoms", len(df))
        with col2:
            st.metric("Average Weight", f"{df['Weight'].mean():.2f}")
        with col3:
            st.metric("Unique Layers", df['Layer'].nunique())
            
    except Exception as e:
        st.error(f"Error loading atom properties: {str(e)}")

def display_molecule_properties():
    """Display molecule properties from JSON file"""
    FILE_PATH = get_project_root() / "resources" / "Mols.json"
    try:
        with open(FILE_PATH, "r") as f:
            molecules_data = json.load(f)
        
        # Prepare properties data
        properties = []
        for mol in molecules_data:
            props = mol['properties'][0]
            properties.append({
                'Name': mol['name'],
                'Formula': props['molecular_formula'],
                'Weight': props['molecular_weight'],
                'IUPAC Name': props['iupac_name'],
                'Roles': ', '.join(props['roles']),
                'Layer': props.get('layer', 0)
            })
        
        # Display dataframe
        df = pd.DataFrame(properties)
        st.dataframe(df)

        # Display metrics
        col1, col2, col3 = st.columns(3)
        with col1:
            st.metric("Total Molecules", len(properties))
        with col2:
            avg_weight = sum(float(mol['properties'][0]['molecular_weight']) for mol in molecules_data)/len(molecules_data)
            st.metric("Avg Weight", f"{avg_weight:.2f}")
        with col3:
            st.metric("Layers Used", len({mol['properties'][0].get('layer', 0) for mol in molecules_data}))
        
    except Exception as e:
        st.error(f"Error loading molecule properties: {str(e)}")

def display_db_config_propeties():
    """Display DB-Config Properties from JSON file"""
    FILE_PATH = get_project_root() / "resources" / "db-config.json"
    
    try:
        with open(FILE_PATH, "r") as f:
            db_data = json.load(f)

        # Column Families table
        properties = []
        for data in db_data["ColumnFamilies"]:
            properties.append({
                "Name": data["name"],
                "Type": data["properties"]["type"],
                "Rows": data["properties"]["numer_of_rows"],
                "Config": data["properties"]["config"] if data["properties"]["config"] != "None" else "Default"
            })
        # Display dataframe
        df = pd.DataFrame(properties)
        st.dataframe(df)

    except Exception as e:
        st.error(f"Error loading DB-Config properties: {str(e)}")