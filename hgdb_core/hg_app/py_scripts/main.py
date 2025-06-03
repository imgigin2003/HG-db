import streamlit as st
from streamlit_option_menu import option_menu
import streamlit.components.v1 as components
from hypergraph import create_hypergraph, draw_hypergraph
from layered_hypergraph import draw_layered_hypergraph
from dual_hypergraph import create_dual_hypergraph, draw_dual_hypergraph
from setting import (
    load_db_path, update_db_path, get_python_version, get_python_library_path,
    update_build_rs, load_atoms_data, load_mols_data, load_genes_data, load_paths_data,
    display_data_properties, display_table, ensure_upload_dir, display_db_config_properties
)
import hypernetx as hnx
import pandas as pd
import os
import json
from pathlib import Path

def display_hypergraph_visualization(hyperedges, graph_type, visualize_mode=None, is_layered=False, is_dual=False, tab_key="", highlighted_path=None, display_mode="default"):
    try:
        if is_dual:
            H, _ = create_dual_hypergraph(hyperedges)
        else:
            H = hnx.Hypergraph({k: v["nodes"] for k, v in hyperedges.items()})
        
        if st.button(f"Visualize {graph_type} ✨", key=f"viz_{tab_key}_{graph_type}"):
            # Debug: Print the display_mode before visualization
            if is_layered:
                fig = draw_layered_hypergraph(hyperedges)
            elif is_dual:
                fig = draw_dual_hypergraph(H)
            else:
                fig = draw_hypergraph(H, hyperedges, visualize_mode, highlighted_path=highlighted_path, display_mode=display_mode)
            if fig:
                st.pyplot(fig)
            else:
                st.error(f"Failed to visualize {graph_type}")
    except Exception as e:
        st.error(f"Visualization error: {str(e)}")

def main():
    # Initialize session state
    if "hyperedges" not in st.session_state:
        try:
            H, hyperedges = create_hypergraph()
            st.session_state.hyperedges = hyperedges if hyperedges else {}
        except Exception as e:
            st.error(f"Error initializing hypergraph: {str(e)}")
            st.session_state.hyperedges = {}
    
    if "active_menu" not in st.session_state:
        st.session_state.active_menu = "main"
    
    if "file_db" not in st.session_state:
        st.session_state.file_db = {}

        if "selected_node" not in st.session_state:
            st.session_state.selected_node = None

    # Sidebar configuration
    with st.sidebar:
        col1, col2 = st.columns(2)
        with col1:
            if st.button("HG-DB Menu", key="main_menu_btn"):
                st.session_state.active_menu = "main"
        with col2:
            if st.button("Bio Menu", key="bio_menu_btn"):
                st.session_state.active_menu = "bio"
        
        menu_options = (
            ["Introduction", "Interactive Hypergraph", "Hyperpath", "Dual Hypergraph", "Layered Hypergraph", "Files/Analysis", "Setting"]
            if st.session_state.active_menu == "main"
            else ["Atoms", "Molecules", "Genes"]
        )
        menu_icons = (
            ['house', 'graph-up','sign-turn-slight-right', 'kanban', 'graph-down', 'cloud-upload', 'gear']
            if st.session_state.active_menu == "main"
            else ['capsule', 'virus', 'gender-ambiguous']
        )
        selected = option_menu(
            "HG-DB Project" if st.session_state.active_menu == "main" else "Bio",
            menu_options,
            icons=menu_icons,
            menu_icon="bezier" if st.session_state.active_menu == "main" else "file-medical",
            default_index=0,
            styles={
                "container": {"padding": "5!important"},
                "icon": {"color": "orange", "font-size": "25px"},
                "nav-link": {"font-size": "16px", "text-align": "left", "margin": "0px", "--hover-color": "#A594F9"},
                "nav-link-selected": {"background-color": "#7E60BF"},
            },
            key="main_menu" if st.session_state.active_menu == "main" else "bio_menu"
        )

    # Main content
    if st.session_state.active_menu == "main":
        if selected == "Introduction":
            st.title("Introduction")
            st.write("Welcome to the HG-DB Project! 📌")
            st.write("This project provides a comprehensive framework for working with hypergraphs.")
            st.write("Features: Interactive Hypergraph, Dual Hypergraph, Layered Hypergraph, File Management")
            st.write("Ensure correct database and Python configuration to avoid issues.")
            st.write("Requires Python 3.8 or higher. Contact the development team for issues.")

        elif selected == "Interactive Hypergraph":
            st.title("Interactive Hypergraph")
            tabs = st.tabs(["Visualization", "Properties", "Tables", "Edit"])
            
            with tabs[0]:
                st.write("### Hypergraph Visualization")
                sub_tabs = st.tabs(["Static Visualize", "Dynamic Visualize"])
                with sub_tabs[0]:
                    col1, col2 = st.columns(2)
                    with col1:
                        visualize_mode = st.radio(
                            "Visualize based on traversable:",
                            options=["edges", "nodes"],
                            format_func=lambda x: f"{'Edges' if x == 'edges' else 'Nodes'} (traversable: {x == 'edges'})",
                            key="hypergraph_viz_mode"
                        )
                    with col2:
                        display_mode = st.radio(
                            "Display mode:",
                            options=["default", "simple", "minimal"],
                            format_func=lambda x: {
                                "default": "Default (filled ellipses)",
                                "simple": "Simple (outline ellipses)",
                                "minimal": "Minimal (lines only)"
                            }[x],
                            key="hypergraph_display_mode"
                        )
                    
                    display_hypergraph_visualization(
                        st.session_state.hyperedges, 
                        "Hypergraph", 
                        visualize_mode, 
                        tab_key="hypergraph",
                        highlighted_path=None,
                        display_mode=display_mode
                    )
                with sub_tabs[1]:
                    st.write("### Interactive Hypergraph Visualization")

                    html_path = Path(__file__).parent.parent / "statics" / "interactive.html"
                    json_path = Path(__file__).parent.parent / "statics" / "test_edge.json"
                    
                    try:
                        # Load HTML template
                        with open(html_path, "r", encoding="utf-8") as f:
                            html_content = f.read()

                        # Load JSON data
                        with open(json_path, "r", encoding="utf-8") as f:
                            json_data = json.load(f)

                        # Inject JSON directly into the HTML as a JS variable
                        injected_script = f"<script>const injectedData = {json.dumps(json_data)};</script>"
                        
                        # Replace the fetch line in HTML with use of the JS variable
                        html_content = html_content.replace(
                        'fetch("test_edge.json")',
                        'Promise.resolve({ ok: true, status: 200, json: () => Promise.resolve(injectedData) })'
)
                        
                        # Insert the injected script before </head> or </body>
                        html_content = html_content.replace("</head>", f"{injected_script}\n</head>")

                        # Show in Streamlit
                        components.html(html_content, height=700, scrolling=True)
                        
                    except Exception as e:
                        st.error(f"Error loading interactive visualization: {str(e)}")

            with tabs[1]:
                st.write("### Properties")
                H = hnx.Hypergraph({k: v["nodes"] for k, v in st.session_state.hyperedges.items()})
                display_data_properties(H, "Hypergraph")
            
            with tabs[2]:
                st.write("### Tables")
                display_table(st.session_state.hyperedges, "Hypergraph")
            
            with tabs[3]:
                st.write("### Edit Hypergraph")
                edit_sub_tabs = st.tabs(["Add", "Edit", "Delete"])
                
                with edit_sub_tabs[0]:
                    st.write("#### Add Hyperedge")
                    new_edge_id = st.text_input("Edge ID:", key="add_edge_id")
                    new_nodes = st.text_input("Nodes (comma-separated):", key="add_nodes")
                    new_layer = st.selectbox(
                        "Layer:", ["Top", "Middle", "Lower"],
                        key="add_layer_select"
                    )
                    is_traverse = st.checkbox("Traversable", key="add_traverse")
                    is_directed = st.checkbox("Directed", key="add_directed")
                    
                    all_nodes = set().union(*[data["nodes"] for data in st.session_state.hyperedges.values()])
                    target_node = None
                    if is_directed:
                        target_node = st.selectbox(
                            "Target Node:", list(all_nodes),
                            key="add_target_node_select"
                        )
                    
                    if st.button("Add Hyperedge", key="add_hyperedge_btn"):
                        if new_edge_id and new_nodes:
                            if new_edge_id in st.session_state.hyperedges:
                                st.warning("Edge ID already exists!")
                            else:
                                nodes_set = set(new_nodes.split(","))
                                layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                                st.session_state.hyperedges[new_edge_id] = {
                                    "nodes": nodes_set,
                                    "head": nodes_set,
                                    "tail": set(),
                                    "traversable": is_traverse,
                                    "type": "linked",
                                    "layer": layer_map[new_layer],
                                    "directed": is_directed,
                                    "target_node": target_node
                                }
                                st.success(f"Added hyperedge '{new_edge_id}' in {new_layer} layer!")

                with edit_sub_tabs[1]:
                    st.write("#### Edit Hyperedge")
                    edge_to_edit = st.selectbox(
                        "Edge:", list(st.session_state.hyperedges.keys()),
                        key="edit_edge_select"
                    )
                    edited_nodes = st.text_input("New nodes (comma-separated):", key="edit_nodes_input")
                    edited_layer = st.selectbox(
                        "New layer:", ["Top", "Middle", "Lower"],
                        key="edit_layer_select"
                    )
                    if st.button("Edit Hyperedge", key="edit_hyperedge_btn"):
                        if edge_to_edit and edited_nodes:
                            new_nodes_set = set(edited_nodes.split(","))
                            layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                            st.session_state.hyperedges[edge_to_edit].update({
                                "nodes": new_nodes_set,
                                "head": new_nodes_set,
                                "tail": set(),
                                "layer": layer_map[edited_layer]
                            })
                            st.success(f"Updated hyperedge '{edge_to_edit}'!")

                with edit_sub_tabs[2]:
                    st.write("#### Delete Hyperedge")
                    edge_to_delete = st.selectbox(
                        "Edge:", list(st.session_state.hyperedges.keys()),
                        key="delete_edge_select"
                    )
                    if st.button("Delete Hyperedge", key="delete_hyperedge_btn"):
                        del st.session_state.hyperedges[edge_to_delete]
                        st.success(f"Deleted hyperedge '{edge_to_delete}'!")

        elif selected == "Hyperpath":
            st.write("### Hyperpath")
            paths = load_paths_data()
            if paths:
                path_ids = [path["id"] for path in paths]
                selected_path_id = st.selectbox(
                    "Select a Hyperpath: ", path_ids,
                    key="hyperpath_select"
                )
                if selected_path_id:
                    selected_path = next(path for path in paths if path ["id"] == selected_path_id)
                    display_hypergraph_visualization(
                        st.session_state.hyperedges, "Hyperpath", visualize_mode="edges",
                        tab_key="hyperpath", highlighted_path=selected_path
                    )
                else:
                    st.warning("No Hyperpath found in paths.json")

        elif selected == "Dual Hypergraph":
            st.title("Dual Hypergraph")
            tabs = st.tabs(["Visualization", "Properties", "Tables"])
            
            with tabs[0]:
                display_hypergraph_visualization(st.session_state.hyperedges, "Dual Hypergraph", is_dual=True, tab_key="dual")
            
            with tabs[1]:
                H_dual, _ = create_dual_hypergraph(st.session_state.hyperedges)
                display_data_properties(H_dual, "Dual Hypergraph")
            
            with tabs[2]:
                _, dual_hyperedges = create_dual_hypergraph(st.session_state.hyperedges)
                display_table(dual_hyperedges, "Dual Hypergraph")

        elif selected == "Layered Hypergraph":
            st.title("Layered Hypergraph")
            display_hypergraph_visualization(st.session_state.hyperedges, "Layered Hypergraph", is_layered=True, tab_key="layered")

        elif selected == "Files/Analysis":
            st.title("Files/Analysis")
            edit_sub_tabs = st.tabs(["Add File", "Delete File", "Files List"])
            upload_dir = ensure_upload_dir()
            
            with edit_sub_tabs[0]:
                st.write("#### Add File")
                uploaded_file = st.file_uploader("Upload file:", key="file_uploader")
                if st.button("Upload File", key="upload_file_btn"):
                    if uploaded_file and upload_dir:
                        file_path = os.path.join(upload_dir, uploaded_file.name)
                        try:
                            with open(file_path, "wb") as f:
                                f.write(uploaded_file.getbuffer())
                            st.session_state.file_db[uploaded_file.name] = file_path
                            st.success(f"Uploaded '{uploaded_file.name}' to {upload_dir}!")
                        except Exception as e:
                            st.error(f"Upload failed: {str(e)}")

            with edit_sub_tabs[1]:
                st.write("#### Delete File")
                if st.session_state.file_db:
                    file_to_delete = st.selectbox(
                        "File:", list(st.session_state.file_db.keys()),
                        key="delete_file_select"
                    )
                    if st.button("Delete File", key="delete_file_btn"):
                        file_path = st.session_state.file_db[file_to_delete]
                        try:
                            os.remove(file_path)
                            del st.session_state.file_db[file_to_delete]
                            st.success(f"Deleted '{file_to_delete}'!")
                        except Exception as e:
                            st.error(f"Deletion failed: {str(e)}")
                else:
                    st.write("No files uploaded yet.")

            with edit_sub_tabs[2]:
                st.write("#### Files List")
                if st.session_state.file_db:
                    file_data = [
                        {"File Name": name, "Path": path, "Size (bytes)": os.path.getsize(path), "Upload Directory": upload_dir}
                        for name, path in st.session_state.file_db.items() if os.path.exists(path)
                    ]
                    st.dataframe(pd.DataFrame(file_data))
                else:
                    st.write("No files uploaded yet.")

        elif selected == "Setting":
            st.title("Configuration")
            tabs = st.tabs(["Configuration", "DB-Config Properties"])
            
            with tabs[0]:
                st.write("Apply configurations to avoid errors! ⚠️")
                
                st.subheader("Database Path")
                current_db_path = load_db_path()
                new_db_path = st.text_input("New database path:", value=current_db_path, key="db_path_input")
                if st.button("Update DB Path", key="update_db_path_btn"):
                    if update_db_path(new_db_path):
                        st.success("Database path updated!")
                    else:
                        st.error("Failed to update database path.")
                
                st.subheader("Python Configuration")
                if st.button("Auto-configure Python", key="auto_config_python_btn"):
                    python_lib_path = get_python_library_path()
                    python_version = get_python_version()
                    if python_lib_path and python_version:
                        if update_build_rs(python_lib_path, python_version):
                            st.success(f"Python {python_version} configured at {python_lib_path}!")
                        else:
                            st.error("Failed to update build.rs.")
                    else:
                        st.error("Could not detect Python configuration.")
                
                st.subheader("Upload Directory")
                new_upload_dir = st.text_input("New upload directory:", value=ensure_upload_dir(), key="upload_dir_input")
                if st.button("Update Upload Directory", key="update_upload_dir_btn"):
                    try:
                        os.makedirs(new_upload_dir, exist_ok=True)
                        st.session_state.upload_dir = new_upload_dir
                        st.session_state.file_db = {}
                        st.success("Upload directory updated!")
                    except Exception as e:
                        st.error(f"Failed to update upload directory: {str(e)}")

            with tabs[1]:
                st.write("### DB-Config Properties")
                display_db_config_properties()

    elif st.session_state.active_menu == "bio":
        if selected == "Atoms":
            st.title("Atoms")
            tabs = st.tabs(["Visualization", "Layered Visualization", "Properties"])
            
            with tabs[0]:
                display_hypergraph_visualization(load_atoms_data(), "Atoms", visualize_mode="nodes", tab_key="atoms")
            
            with tabs[1]:
                display_hypergraph_visualization(load_atoms_data(), "Layered Atoms", is_layered=True, tab_key="layered_atoms")
            
            with tabs[2]:
                display_data_properties(load_atoms_data(), "Atoms")

        elif selected == "Molecules":
            st.title("Molecules")
            tabs = st.tabs(["Visualization", "Layered Visualization", "Properties"])
            
            with tabs[0]:
                display_hypergraph_visualization(load_mols_data(), "Molecules", visualize_mode="edges", tab_key="molecules")
            
            with tabs[1]:
                display_hypergraph_visualization(load_mols_data(), "Layered Molecules", is_layered=True, tab_key="layered_molecules")
            
            with tabs[2]:
                display_data_properties(load_mols_data(), "Molecules")

        elif selected == "Genes":
            st.title("Genes")
            tabs = st.tabs(["Visualization", "Layered Visualization", "Properties"])
            
            with tabs[0]:
                display_hypergraph_visualization(load_genes_data(), "Genes", visualize_mode="edges", tab_key="genes")
            
            with tabs[1]:
                data = load_genes_data()
                for gene_name, gene_data in data.items():
                    chrom = gene_data["properties"]["chromosome_location"]
                    chrom_num = ''.join(filter(str.isdigit, chrom.split('q')[0].split('p')[0]))
                    gene_data["layer"] = int(chrom_num) % 3 if chrom_num else 0
                display_hypergraph_visualization(data, "Layered Genes", is_layered=True, tab_key="layered_genes")
            
            with tabs[2]:
                display_data_properties(load_genes_data(), "Genes")

if __name__ == "__main__":
    main()