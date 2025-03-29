import streamlit as st
from streamlit_option_menu import option_menu
from hypergraph import create_hypergraph, draw_hypergraph
from layered_hypergraph import draw_layered_hypergraph
from dual_hypergraph import create_dual_hypergraph, draw_dual_hypergraph
import hypernetx as hnx
import pandas as pd
import toml
import subprocess
import os 
from pathlib import Path

# ---------------------------------------------- Basic Configuration Section ------------------------------------------------------------------

def main():
    if "hyperedges" not in st.session_state:
        H, hyperedges = create_hypergraph()
        st.session_state.hyperedges = hyperedges if hyperedges else {}

    # Load db_path for uploaded_files directory
    db_path = load_db_path()

    # Initialize a file database in session state
    if "file_db" not in st.session_state:
        st.session_state.file_db = {}  # Key: filename, Value: file_path

    with st.sidebar:
        choose = option_menu("HG-DB Project", ["Introduction", "Interactive Hypergraph", "Dual", "Files/Analysis"],
                            icons=['house', 'bar-chart', 'kanban', 'cloud-upload'],
                            menu_icon="github", default_index=0,
                            styles={
            "container": {"padding": "5!important", "background-color": "#fafafa"},
            "icon": {"color": "orange", "font-size": "25px"}, 
            "nav-link": {"font-size": "16px", "text-align": "left", "margin":"0px", "--hover-color": "#eee"},
            "nav-link-selected": {"background-color": "#7f827f"},
        }
    )

# ---------------------------------------------- Introduction Tab ------------------------------------------------------------------

    if choose == "Introduction":
        st.title("Introduction")
        st.write("Welcome to the HG-DB Project!📌  Configure your settings below:")
                
        st.header("Configuration")

        # Step 1: DB Path Configuration
        st.subheader("1. Database Path Configuration ")
        current_db_path = load_db_path()
        new_db_path = st.text_input(
            "Enter new database path:",
            value=current_db_path,
            key="db_path_input"
        )
        
        if st.button("Update DB Path 🔧"):
            if update_db_path(new_db_path):
                st.success("Database path updated successfully! ✅")
            else:
                st.error("Failed to update database path ❌")
        

        # Python Configuration Section
        st.subheader("Python Configuration")
        if st.button("Auto-configure Python Bindings 🔧"):
            python_lib_path = get_python_library_path()
            python_version = get_python_version()
            
            if python_lib_path and python_version:
                st.success(f"Detected Python library path: {python_lib_path} ⚙️")
                st.success(f"Detected Python version: {python_version} ⚙️")
                
                if update_build_rs(python_lib_path, python_version):
                    st.success("""
                    build.rs updated successfully with correct paths! ✅
                    - Python library linking (python{python_version})
                    - Library search path (without python version suffix)
                    - Rebuild trigger on Python changes
                    """)
                else:
                    st.error("Failed to update build.rs ❌")
            else:
                st.error("Could not detect Python configuration properly ❌")

# ---------------------------------------------- Hypergraph Tab ------------------------------------------------------------------

    if choose == "Interactive Hypergraph":
        st.title("Interactive Hypergraph")
        tabs = st.tabs([
            "HyperGraph Visualization", 
            "Layered HyperGraph Visualization",
        ])
        with tabs[0]:
            st.write("### HyperGraph Visualization")
            H = hnx.Hypergraph({k: v["nodes"] for k, v in st.session_state.hyperedges.items()})
            visualize_mode = st.radio(
                "Visualize based on traversable:",
                options=["edges", "nodes"],
                format_func=lambda x: f"{'Edges' if x == 'edges' else 'Nodes'} (traversable: {x == 'edges'})"
            )
            if st.button("Visualize HyperGraph✨"):
                fig = draw_hypergraph(H, st.session_state.hyperedges, visualize_mode)
                if fig:
                    st.pyplot(fig)
        with tabs[1]:
            st.write("### Layered HyperGraph Visualization")
            if st.button("Visualize Layered HyperGraph✨"):
                fig = draw_layered_hypergraph(st.session_state.hyperedges)  # Pass the full hyperedges dictionary
                if fig:
                    st.pyplot(fig)

# ---------------------------------------------- Dual Hypergraph Tab ------------------------------------------------------------------

    if choose == "Dual Hypergraph":
        st.title("Dual HyperGraph")
        tabs = st.tabs(["Dual Hypergraph Visualization"])
        with tabs[0]:
            st.write("### Dual HyperGraph Visualization")
            H_dual, _ = create_dual_hypergraph(st.session_state.hyperedges)
            if st.button("Visualize Dual HyperGraph✨"):
                fig = draw_dual_hypergraph(H_dual)
                if fig:
                    st.pyplot(fig)

# ---------------------------------------------- Analistics Tab ------------------------------------------------------------------

    if choose == "Files/Analysis":
        st.title("Files/Analysis")
        tabs = st.tabs([
            "Graph Properties",
            "Graph tables",
            "Graph Edit",
            "Upload Files",
        ])

# ---------------------------------------------- Graph Information Tab ------------------------------------------------------------------
        with tabs[0]:
            st.write("### Graphs Properties")
            H = hnx.Hypergraph({k: v["nodes"] for k, v in st.session_state.hyperedges.items()})
            display_properties(H, "HyperGraph")
            H_dual, _ = create_dual_hypergraph(st.session_state.hyperedges)
            display_properties(H_dual, "Dual HyperGraph")

        with tabs[1]:
            st.write("### Graphs Tables")
            display_table(st.session_state.hyperedges, "HyperGraph")
            H_dual, dual_hyperedges = create_dual_hypergraph(st.session_state.hyperedges)
            display_table(dual_hyperedges, "Dual HyperGraph")

# ---------------------------------------------- Graph Edit Tab ------------------------------------------------------------------

        with tabs[2]:
            st.write("### Graph Edit")
            edit_sub_tabs = st.tabs(["Add Hyperedge", "Edit Hyperedge", "Delete Hyperedge"])

            with edit_sub_tabs[0]:
                st.write("#### Add Hyperedge")
                new_edge_id = st.text_input("Enter new edge ID (e.g., e8):", key="new_edge_id")
                new_nodes = st.text_input("Enter nodes for the new edge (comma-separated):", key="new_nodes")
                new_layer = st.selectbox("Select layer for the new edge:", options=["Top", "Middle", "Lower"], key="new_layer")
                is_traverse = st.checkbox("Traversable", value=False, key="traverse")
                is_directed = st.checkbox("Directed", value=False, key="directed")  # New Directed checkbox

                # Collect all nodes in the hypergraph for the dropdown
                all_nodes = set()
                for edge_data in st.session_state.hyperedges.values():
                    all_nodes.update(edge_data["nodes"])
                target_node = None
                if is_directed:
                    st.write("Select a target node for the directed edge:")
                    target_node = st.selectbox(
                        "Target Node:",
                        options=list(all_nodes),
                        key="target_node"
                    )

                if st.button("Add Hyperedge"):
                    if new_edge_id and new_nodes:
                        nodes_set = set(new_nodes.split(","))
                        if new_edge_id in st.session_state.hyperedges:
                            st.warning("Edge ID already exists!")
                        else:
                            layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                            st.session_state.hyperedges[new_edge_id] = {
                                "nodes": nodes_set,
                                "head": nodes_set,
                                "tail": set(),
                                "traversable": is_traverse,
                                "type": "linked",
                                "layer": layer_map[new_layer],
                                "directed": is_directed,  # Store the directed field
                                "target_node": target_node if is_directed else None  # Store the target node if directed
                            }
                            st.success(f"Hyperedge '{new_edge_id}' added with nodes {nodes_set} in {new_layer} layer! Directed: {is_directed}, Target Node: {target_node if is_directed else 'None'}")

            with edit_sub_tabs[1]:
                st.write("### Edit Hyperedge")
                edge_to_edit = st.selectbox("Select an edge to edit:", options=list(st.session_state.hyperedges.keys()))
                edited_nodes = st.text_input("Enter new nodes for the selected edge (comma-separated):", key="edit_nodes")
                edited_layer = st.selectbox("Select new layer for the selected edge:", options=["Top", "Middle", "Lower"], key="edit_layer")
                if st.button("Edit Hyperedge"):
                    if edge_to_edit and edited_nodes:
                        new_nodes_set = set(edited_nodes.split(","))
                        layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                        st.session_state.hyperedges[edge_to_edit]["nodes"] = new_nodes_set
                        st.session_state.hyperedges[edge_to_edit]["head"] = new_nodes_set
                        st.session_state.hyperedges[edge_to_edit]["tail"] = set()
                        st.session_state.hyperedges[edge_to_edit]["layer"] = layer_map[edited_layer]  # Update layer
                        st.success(f"Hyperedge '{edge_to_edit}' updated with nodes '{new_nodes_set}' in {edited_layer} layer! ✅")

            with edit_sub_tabs[2]:
                st.write("### Delete Hyperedge")
                edges_to_delete = st.selectbox("Select an edge to delete:", options=list(st.session_state.hyperedges.keys()))
                if st.button("Delete Hyperedge"):
                    if edges_to_delete:
                        del st.session_state.hyperedges[edges_to_delete]
                        st.success(f"Hyperedge '{edges_to_delete}' deleted successfully. ✅")

# ---------------------------------------------- Manage Files Tab ------------------------------------------------------------------

        with tabs[3]:
            st.write("### Upload Files")
            edit_sub_tabs = st.tabs(["Add File", "Delete File", "Files List"])

            # Ensure upload directory exists
            upload_dir = ensure_upload_dir()
            if not upload_dir:
                st.error("Database path not configured! Please set DB path in Introduction tab.")
                st.stop()

            with edit_sub_tabs[0]:
                st.write("#### Add File")
                uploaded_file = st.file_uploader("Choose a file to upload", type=None, key="file_uploader")
                if st.button("Upload File"):
                    if uploaded_file:
                        file_path = os.path.join(upload_dir, uploaded_file.name)
                        try:
                            with open(file_path, "wb") as f:
                                f.write(uploaded_file.getbuffer())
                            st.session_state.file_db[uploaded_file.name] = file_path
                            st.success(f"File '{uploaded_file.name}' uploaded to database directory! ✅")
                        except Exception as e:
                            st.error(f"Upload failed: {str(e)} ❌")

            with edit_sub_tabs[1]:
                st.write("#### Delete File")
                if st.session_state.file_db:
                    file_to_delete = st.selectbox(
                        "Select a file to delete:",
                        options=list(st.session_state.file_db.keys()),
                        key="delete_file"
                    )
                    if st.button("Delete File"):
                        if file_to_delete in st.session_state.file_db:
                            file_path = st.session_state.file_db[file_to_delete]
                            try:
                                os.remove(file_path)
                                del st.session_state.file_db[file_to_delete]
                                st.success(f"File deleted from database directory! ✅")
                            except Exception as e:
                                st.error(f"Deletion failed: {str(e)} ❌")

            with edit_sub_tabs[2]:
                st.write("#### Files List")
                if st.session_state.file_db:
                    file_data = []
                    for name, path in st.session_state.file_db.items():
                        try:
                            size = os.path.getsize(path)
                            file_data.append({
                                "File Name": name,
                                "Path": path,
                                "Size (bytes)": size
                            })
                        except:
                            continue
                    df = pd.DataFrame(file_data)
                    st.dataframe(df)
                else:
                    st.write("No files in database directory yet ☁️")

# ---------------------------------------------- Functions Section ------------------------------------------------------------------

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

def load_db_path():
    """Load db_path from Config.toml in hgdb_core directory."""
    config_file = os.path.join("hgdb_core", "Config.toml")
    if os.path.exists(config_file):
        try:
            with open(config_file, "r") as f:
                config_data = toml.load(f)
            return config_data.get("db_path", "/Users/gigin/Documents/mydbs/rocksdb/")
        except Exception as e:
            st.error(f"Failed to load '{config_file}': {str(e)}. Using default path.")
    return ""


def get_project_root():
    """Helper function to get the project root relative to main.py's location"""
    return Path(__file__).parent.parent.parent # Goes up from py_scripts to hgdb_core

def load_db_path():
    """Load db_path from Config.toml with correct path"""
    config_file = get_project_root() / "Config.toml"
    if config_file.exists():
        try:
            with open(config_file, "r") as f:
                config_data = toml.load(f)
            return config_data.get("db_path", "/Users/gigin/Documents/mydbs/rocksdb/")
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


if __name__ == "__main__":
    main()