import json
import requests
import streamlit as st
from streamlit_option_menu import option_menu
from data_loader import DataLoader
from visualization_manager import VisualizationManager, RUST_CORE_URL
from hypergraph import HypergraphManager
from dual_hypergraph import DualHypergraphManager
from layered_hypergraph import LayeredHypergraphManager


class AppManager:
    def __init__(self):
        self.data_loader = DataLoader()
        self.visualizer = VisualizationManager()
        self.hypergraph = HypergraphManager()
        self.dual_hypergraph = DualHypergraphManager()
        self.layered_hypergraph = LayeredHypergraphManager()
        self.initialize_session_state()

    def initialize_session_state(self):
        if "hyperedges" not in st.session_state:
            try:
                _, hyperedges = self.hypergraph.create_hypergraph()
                st.session_state.hyperedges = hyperedges if hyperedges else {}
            except Exception as e:
                st.error(f"Error initializing hypergraph: {str(e)}")
                st.session_state.hyperedges = {}

        if "active_menu" not in st.session_state:
            st.session_state.active_menu = "main"

        if "selected_node" not in st.session_state:
            st.session_state.selected_node = None

    def render_sidebar(self):
        with st.sidebar:
            col1, col2 = st.columns(2)
            with col1:
                if st.button("HG-DB Menu", key="main_menu_btn"):
                    st.session_state.active_menu = "main"
            with col2:
                if st.button("Bio Menu", key="bio_menu_btn"):
                    st.session_state.active_menu = "bio"

            menu_options = (
                [
                    "Introduction",
                    "Upload Data",
                    "Interactive Hypergraph",
                    "Hyperpath",
                    "Dual Hypergraph",
                    "Layered Hypergraph",
                ]
                if st.session_state.active_menu == "main"
                else ["Atoms", "Molecules", "Genes"]
            )

            menu_icons = (
                [
                    "house",
                    "cloud-upload",
                    "graph-up",
                    "sign-turn-slight-right",
                    "kanban",
                    "graph-down",
                ]
                if st.session_state.active_menu == "main"
                else ["capsule", "virus", "gender-ambiguous"]
            )

            selected = option_menu(
                (
                    "HG-DB Project"
                    if st.session_state.active_menu == "main"
                    else "Bio"
                ),
                menu_options,
                icons=menu_icons,
                menu_icon=(
                    "bezier"
                    if st.session_state.active_menu == "main"
                    else "file-medical"
                ),
                default_index=0,
                styles={
                    "container": {"padding": "5!important"},
                    "icon": {"color": "orange", "font-size": "25px"},
                    "nav-link": {
                        "font-size": "16px",
                        "text-align": "left",
                        "margin": "0px",
                        "--hover-color": "#A594F9",
                    },
                    "nav-link-selected": {"background-color": "#7E60BF"},
                },
                key=(
                    "main_menu"
                    if st.session_state.active_menu == "main"
                    else "bio_menu"
                ),
            )

            return selected

    def render_main_menu(self, selected):
        if selected == "Introduction":
            self.render_introduction()
        elif selected == "Upload Data":
            self.render_upload_data()
        elif selected == "Interactive Hypergraph":
            self.render_interactive_hypergraph()
        elif selected == "Hyperpath":
            self.render_hyperpath()
        elif selected == "Dual Hypergraph":
            self.render_dual_hypergraph()
        elif selected == "Layered Hypergraph":
            self.render_layered_hypergraph()

    def render_bio_menu(self, selected):
        if selected == "Atoms":
            self.render_atoms()
        elif selected == "Molecules":
            self.render_molecules()
        elif selected == "Genes":
            self.render_genes()

    def render_introduction(self):
        st.title("🕸️ HG-DB — Hypergraph Explorer")
        st.write(
            "Upload a hypergraph, let the Rust core normalize it, and explore it "
            "across interactive 2D, dual, and layered 3D views."
        )
        col1, col2, col3 = st.columns(3)
        with col1:
            st.subheader("📤 Upload")
            st.write("Drop in any hypergraph JSON. The Rust core formats it for you.")
        with col2:
            st.subheader("🔗 Explore")
            st.write("Interactive hypergraph, hyperpaths, and the dual hypergraph.")
        with col3:
            st.subheader("🧊 Layered 3D")
            st.write("Send layers to the Three.js service for a 3D scene.")
        st.divider()
        st.info("Start in the **Upload Data** tab — or load the bundled sample there.")

    def render_interactive_hypergraph(self):
        st.title("Interactive Hypergraph")
        tabs = st.tabs(["Visualization", "Properties", "Tables", "Edit"])

        with tabs[0]:
            st.write("### Hypergraph Visualization")
            col1, col2 = st.columns(2)
            with col1:
                visualize_mode = st.radio(
                    "Visualize based on traversable:",
                    options=["edges", "nodes"],
                    format_func=lambda x: f"{'Edges' if x == 'edges' else 'Nodes'} (traversable: {x == 'edges'})",
                    key="hypergraph_viz_mode",
                )
            with col2:
                display_mode = st.radio(
                    "Display mode:",
                    options=["default", "simple", "minimal"],
                    format_func=lambda x: {
                        "default": "Default (filled ellipses)",
                        "simple": "Simple (outline ellipses)",
                        "minimal": "Minimal (lines only)",
                    }[x],
                    key="hypergraph_display_mode",
                )

            self.visualizer.display_hypergraph_visualization(
                st.session_state.hyperedges,
                "Hypergraph",
                visualize_mode,
                tab_key="hypergraph",
                highlighted_path=None,
                display_mode=display_mode,
            )

        with tabs[1]:
            st.write("### Properties")
            H = self.hypergraph.create_hypergraph()[0]
            self.visualizer.display_data_properties(H, "Hypergraph")

        with tabs[2]:
            st.write("### Tables")
            self.visualizer.display_table(
                st.session_state.hyperedges, "Hypergraph"
            )

        with tabs[3]:
            self.render_hypergraph_editor()

    def render_hypergraph_editor(self):
        st.write("### Edit Hypergraph")
        edit_sub_tabs = st.tabs(["Add", "Edit", "Delete"])

        with edit_sub_tabs[0]:
            st.write("#### Add Hyperedge")
            new_edge_id = st.text_input("Edge ID:", key="add_edge_id")
            new_nodes = st.text_input(
                "Nodes (comma-separated):", key="add_nodes"
            )
            new_layer = st.selectbox(
                "Layer:", ["Top", "Middle", "Lower"], key="add_layer_select"
            )
            is_traverse = st.checkbox("Traversable", key="add_traverse")
            is_directed = st.checkbox("Directed", key="add_directed")

            all_nodes = set().union(
                *[
                    data["nodes"]
                    for data in st.session_state.hyperedges.values()
                ]
            )
            target_node = None
            if is_directed:
                target_node = st.selectbox(
                    "Target Node:",
                    list(all_nodes),
                    key="add_target_node_select",
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
                            "target_node": target_node,
                        }
                        st.success(
                            f"Added hyperedge '{new_edge_id}' in {new_layer} layer!"
                        )

        with edit_sub_tabs[1]:
            st.write("#### Edit Hyperedge")
            edge_to_edit = st.selectbox(
                "Edge:",
                list(st.session_state.hyperedges.keys()),
                key="edit_edge_select",
            )
            edited_nodes = st.text_input(
                "New nodes (comma-separated):", key="edit_nodes_input"
            )
            edited_layer = st.selectbox(
                "New layer:",
                ["Top", "Middle", "Lower"],
                key="edit_layer_select",
            )
            if st.button("Edit Hyperedge", key="edit_hyperedge_btn"):
                if edge_to_edit and edited_nodes:
                    new_nodes_set = set(edited_nodes.split(","))
                    layer_map = {"Top": 0, "Middle": 1, "Lower": 2}
                    st.session_state.hyperedges[edge_to_edit].update(
                        {
                            "nodes": new_nodes_set,
                            "head": new_nodes_set,
                            "tail": set(),
                            "layer": layer_map[edited_layer],
                        }
                    )
                    st.success(f"Updated hyperedge '{edge_to_edit}'!")

        with edit_sub_tabs[2]:
            st.write("#### Delete Hyperedge")
            edge_to_delete = st.selectbox(
                "Edge:",
                list(st.session_state.hyperedges.keys()),
                key="delete_edge_select",
            )
            if st.button("Delete Hyperedge", key="delete_hyperedge_btn"):
                del st.session_state.hyperedges[edge_to_delete]
                st.success(f"Deleted hyperedge '{edge_to_delete}'!")

    def render_hyperpath(self):
        st.write("### Hyperpath")
        paths = self.data_loader.load_paths_data()
        if paths:
            path_ids = [path["id"] for path in paths]
            selected_path_id = st.selectbox(
                "Select a Hyperpath: ", path_ids, key="hyperpath_select"
            )
            if selected_path_id:
                selected_path = next(
                    path for path in paths if path["id"] == selected_path_id
                )
                self.visualizer.display_hypergraph_visualization(
                    st.session_state.hyperedges,
                    "Hyperpath",
                    visualize_mode="edges",
                    tab_key="hyperpath",
                    highlighted_path=selected_path,
                )
            else:
                st.warning("No Hyperpath found in paths.json")

    def render_dual_hypergraph(self):
        st.title("Dual Hypergraph")
        tabs = st.tabs(["Visualization", "Properties", "Tables"])

        with tabs[0]:
            self.visualizer.display_hypergraph_visualization(
                st.session_state.hyperedges,
                "Dual Hypergraph",
                is_dual=True,
                tab_key="dual",
            )

        with tabs[1]:
            H_dual = self.dual_hypergraph.create_dual_hypergraph(
                st.session_state.hyperedges
            )[0]
            self.visualizer.display_data_properties(H_dual, "Dual Hypergraph")

        with tabs[2]:
            _, dual_hyperedges = self.dual_hypergraph.create_dual_hypergraph(
                st.session_state.hyperedges
            )
            self.visualizer.display_table(dual_hyperedges, "Dual Hypergraph")

    def render_layered_hypergraph(self):
        st.title("Layered Hypergraph")
        self.visualizer.display_hypergraph_visualization(
            st.session_state.hyperedges,
            "Layered Hypergraph",
            is_layered=True,
            tab_key="layered",
        )

    def render_upload_data(self):
        st.title("Upload Data")
        st.caption(
            "Upload any hypergraph JSON. The Rust core normalizes it into the "
            "canonical format, then every view below uses it."
        )

        col_a, col_b = st.columns([3, 1])
        with col_a:
            uploaded_file = st.file_uploader(
                "Hypergraph JSON file", type=["json"], key="hg_uploader"
            )
        with col_b:
            st.write("")
            st.write("")
            if st.button("↺ Load sample", key="load_sample_btn"):
                self._load_sample_into_session()

        if uploaded_file and st.button("⚙️ Process file", key="process_file_btn"):
            self._process_upload(uploaded_file)

        st.divider()
        self._render_active_dataset_summary()

    def _process_upload(self, uploaded_file):
        """Send raw JSON to the Rust normalizer and load the result into session."""
        try:
            raw = json.load(uploaded_file)
        except Exception as e:
            st.error(f"That file isn't valid JSON: {e}")
            return

        try:
            resp = requests.post(
                f"{RUST_CORE_URL}/api/normalize", json=raw, timeout=10
            )
        except requests.exceptions.ConnectionError:
            st.error(
                f"Cannot reach the Rust core at {RUST_CORE_URL}. "
                "Start it with `cargo run` inside `hgdb_core`."
            )
            return

        if resp.status_code != 200:
            detail = resp.json().get("error", resp.text)
            st.error(f"Normalization failed: {detail}")
            return

        canonical = resp.json()
        try:
            hyperedges = self.hypergraph.parse_edges(canonical)
        except Exception as e:
            st.error(f"Could not build hypergraph from normalized data: {e}")
            return

        st.session_state.hyperedges = hyperedges
        st.session_state.canonical_data = canonical
        st.success(
            f"✅ Loaded {len(hyperedges)} edges from '{uploaded_file.name}'. "
            "Open any visualization tab to explore it."
        )
        with st.expander("📦 Normalized JSON (canonical format)", expanded=False):
            st.json(canonical)

    def _load_sample_into_session(self):
        try:
            st.session_state.hyperedges = self.hypergraph.load_sample()
            st.session_state.pop("canonical_data", None)
            st.success("✅ Loaded the bundled sample hypergraph.")
        except Exception as e:
            st.error(f"Failed to load sample data: {e}")

    def _render_active_dataset_summary(self):
        st.subheader("Active dataset")
        hyperedges = st.session_state.get("hyperedges") or {}
        if not hyperedges:
            st.info("No data loaded yet — upload a file or load the sample.")
            return

        nodes = set().union(*[d["nodes"] for d in hyperedges.values()])
        layers = {d.get("layer", 0) for d in hyperedges.values()}
        col1, col2, col3 = st.columns(3)
        col1.metric("Hyperedges", len(hyperedges))
        col2.metric("Nodes", len(nodes))
        col3.metric("Layers", len(layers))

    def render_atoms(self):
        st.title("Atoms")
        tabs = st.tabs(
            ["Visualization", "Layered Visualization", "Properties"]
        )

        with tabs[0]:
            self.visualizer.display_hypergraph_visualization(
                self.data_loader.load_atoms_data(),
                "Atoms",
                visualize_mode="nodes",
                tab_key="atoms",
            )

        with tabs[1]:
            self.visualizer.display_hypergraph_visualization(
                self.data_loader.load_atoms_data(),
                "Layered Atoms",
                is_layered=True,
                tab_key="layered_atoms",
            )

        with tabs[2]:
            self.visualizer.display_data_properties(
                self.data_loader.load_atoms_data(), "Atoms"
            )

    def render_molecules(self):
        st.title("Molecules")
        tabs = st.tabs(
            ["Visualization", "Layered Visualization", "Properties"]
        )

        with tabs[0]:
            self.visualizer.display_hypergraph_visualization(
                self.data_loader.load_mols_data(),
                "Molecules",
                visualize_mode="edges",
                tab_key="molecules",
            )

        with tabs[1]:
            self.visualizer.display_hypergraph_visualization(
                self.data_loader.load_mols_data(),
                "Layered Molecules",
                is_layered=True,
                tab_key="layered_molecules",
            )

        with tabs[2]:
            self.visualizer.display_data_properties(
                self.data_loader.load_mols_data(), "Molecules"
            )

    def render_genes(self):
        st.title("Genes")
        tabs = st.tabs(
            ["Visualization", "Layered Visualization", "Properties"]
        )

        with tabs[0]:
            self.visualizer.display_hypergraph_visualization(
                self.data_loader.load_genes_data(),
                "Genes",
                visualize_mode="edges",
                tab_key="genes",
            )

        with tabs[1]:
            data = self.data_loader.load_genes_data()
            for gene_name, gene_data in data.items():
                chrom = gene_data["properties"]["chromosome_location"]
                chrom_num = "".join(
                    filter(str.isdigit, chrom.split("q")[0].split("p")[0])
                )
                gene_data["layer"] = int(chrom_num) % 3 if chrom_num else 0
            self.visualizer.display_hypergraph_visualization(
                data, "Layered Genes", is_layered=True, tab_key="layered_genes"
            )

        with tabs[2]:
            self.visualizer.display_data_properties(
                self.data_loader.load_genes_data(), "Genes"
            )

    def run(self):
        self.initialize_session_state()
        selected = self.render_sidebar()

        if st.session_state.active_menu == "main":
            self.render_main_menu(selected)
        elif st.session_state.active_menu == "bio":
            self.render_bio_menu(selected)
