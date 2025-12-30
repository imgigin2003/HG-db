import streamlit as st
import hypernetx as hnx
import requests
import matplotlib as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import numpy as np
import networkx as nx
from matplotlib.patches import Ellipse, Circle
from pathlib import Path
import pandas as pd
import json
import streamlit.components.v1 as components
from data_loader import DataLoader
from hypergraph import HypergraphManager
from dual_hypergraph import DualHypergraphManager
from layered_hypergraph import LayeredHypergraphManager


class VisualizationManager:
    def __init__(self):
        self.project_root = DataLoader()
        self.data_loader = DataLoader()
        self.hypergraph = HypergraphManager()
        self.dual_hypergraph = DualHypergraphManager()
        self.layered_hypergraph = LayeredHypergraphManager()

    def serialize_hyperedges(self, hyperedges):
        clean = {}

        for edge_name, data in hyperedges.items():
            clean[edge_name] = {
                "nodes": list(data["nodes"]),  
                "layer": data.get("layer", 0)
            }

        return clean


    def send_to_layered_service(self, layers):
        url = "http://127.0.0.1:8000/render-layered"

        payload = {
            "layers": layers
        }

        response = requests.post(url, json=payload)

        if response.status_code != 200:
            raise RuntimeError(
                f"Layered service failed [{response.status_code}]: {response.text}"
            )

        return {
            "method": "POST",
            "endpoint": "/render-layered",
            "status": response.status_code,
            "request": payload,
            "body": response.json()
        }


    def display_hypergraph_visualization(self, hyperedges, graph_type, visualize_mode=None, 
                                        is_layered=False, is_dual=False, tab_key="", 
                                        highlighted_path=None, display_mode="default"):
        try:
            if is_dual:
                H, _ = self.dual_hypergraph.create_dual_hypergraph(hyperedges)
            else:
                H = hnx.Hypergraph({k: v["nodes"] for k, v in hyperedges.items()})
            
            if st.button(f"Visualize {graph_type} ✨", key=f"viz_{tab_key}_{graph_type}"):
                    if is_layered:
                        st.info("Sending Layered Hypergraph to 3D Service...")


                        # --------- Build request payload (Layered format) ----------
                        serialized = {}

                        for edge_id, data in hyperedges.items():
                            layer = str(data.get("layer", 0))
                            serialized.setdefault(layer, {})
                            serialized[layer][edge_id] = list(data["nodes"])
                        
                        request_payload = {
                            "layers": serialized
                        }

                        # --------- Send to microservice ----------
                        result = self.send_to_layered_service(serialized)


                        # --------- Layout: 2 columns ----------
                        viewer_url = "http://127.0.0.1:8000/viewer"

                        colA, colB, colC = st.columns([1,2,1])
                        with colB:
                            st.markdown(
                                f"""
                                <div style="text-align:center; margin-bottom:15px;">
                                    <a href="{viewer_url}" target="_blank">
                                        <button style="
                                            background:#6a00ff;
                                            color:white;
                                            border:none;
                                            padding:12px 20px;
                                            font-size:18px;
                                            border-radius:10px;
                                            cursor:pointer;
                                        ">
                                            🔮 Open 3D Viewer
                                        </button>
                                    </a>
                                </div>
                                """,
                                unsafe_allow_html=True
                            )

                        # -------- Request / Response --------
                        col1, col2 = st.columns(2)

                        with col1:
                            with st.expander("📤 Request → FastAPI", expanded=False):
                                st.code(json.dumps(request_payload, indent=2), language="json")

                        with col2:
                            with st.expander("📥 Response ← Microservice", expanded=False):
                                st.json(result)

                        # -------- Status --------
                        if result["status"] == 200:
                            st.success("3D Scene Generated Successfully 🚀")
                        else:
                            st.error("Layered service failed")

                        return
                    
                    elif is_dual:
                        fig = self.dual_hypergraph.draw_dual_hypergraph(H)
                    
                    else:
                        fig = self.hypergraph.draw_hypergraph(H, hyperedges, visualize_mode, 
                                                highlighted_path=highlighted_path, 
                                                display_mode=display_mode)
                    if fig:
                        st.pyplot(fig)
                    else:
                        st.error(f"Failed to visualize {graph_type}")
        
        except Exception as e:
            st.error(f"Visualization error: {str(e)}")

    def render_interactive_visualization(self):
            html_path = self.data_loader.get_statics_root() / "statics" / "interactive.html"
            json_path = self.data_loader.get_statics_root() / "statics" / "test_edge.json"
            
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

    def display_data_properties(self, data, data_type):
        """Display properties for atoms, molecules, genes, hypergraphs, or dual hypergraphs"""
        st.write(f"### {data_type} Properties")
        
        try:
            if data_type in ["Hypergraph", "Dual Hypergraph"]:
                # Handle hypergraph and dual hypergraph (expects hypernetx.Hypergraph object)
                if isinstance(data, hnx.Hypergraph):
                    nodes_list = list(data.nodes())
                    edges_list = list(data.edges())
                    st.write(f"Nodes: {nodes_list}")
                    st.write(f"Number of nodes: {len(nodes_list)}")
                    st.write(f"Edges: {edges_list}")
                    st.write(f"Number of edges: {len(edges_list)}")
                else:
                    st.error(f"Invalid {data_type} data: Expected hypernetx.Hypergraph object")
                    return

            else:
                # Handle atoms, molecules, genes
                FILE_PATHS = {
                    "Atoms": self.data_loader.get_resources_root() / "resources" / "Atoms.json",
                    "Molecules": self.data_loader.get_resources_root() / "resources" / "Mols.json",
                    "Genes": self.data_loader.get_resources_root() / "resources" / "Genes.json"
                }
                
                if data_type not in FILE_PATHS:
                    st.error(f"Unsupported data type: {data_type}")
                    return

                with open(FILE_PATHS[data_type], "r") as f:
                    json_data = json.load(f)
                    if data_type == "Genes":
                        json_data = json_data["Genes"]  # Genes data is nested under "Genes" key

                properties = []
                if data_type == "Atoms":
                    for atom in json_data:
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
                    
                    col1, col2, col3 = st.columns(3)
                    with col1:
                        st.metric("Total Atoms", len(df))
                    with col2:
                        st.metric("Average Weight", f"{df['Weight'].mean():.2f}")
                    with col3:
                        st.metric("Unique Layers", df['Layer'].nunique())

                elif data_type == "Molecules":
                    for mol in json_data:
                        props = mol['properties'][0]
                        properties.append({
                            'Name': mol['name'],
                            'Formula': props['molecular_formula'],
                            'Weight': props['molecular_weight'],
                            'IUPAC Name': props['iupac_name'],
                            'Roles': ', '.join(props['roles']),
                            'Layer': props.get('layer', 0)
                        })
                    df = pd.DataFrame(properties)
                    st.dataframe(df)
                    
                    col1, col2, col3 = st.columns(3)
                    with col1:
                        st.metric("Total Molecules", len(properties))
                    with col2:
                        avg_weight = sum(float(mol['properties'][0]['molecular_weight']) for mol in json_data) / len(json_data)
                        st.metric("Avg Weight", f"{avg_weight:.2f}")
                    with col3:
                        st.metric("Layers Used", len({mol['properties'][0].get('layer', 0) for mol in json_data}))

                elif data_type == "Genes":
                    for gene in json_data:
                        props = gene["properties"]
                        properties.append({
                            'Gene Name': gene["name"],
                            'Chromosome': props["chromosome_location"],
                            'Protein': props["encoded_protein"],
                            'Function': props["function"],
                            'Biological Processes': ', '.join(props["biological_processes"])
                        })
                    df = pd.DataFrame(properties)
                    st.dataframe(df)
                    
                    col1, col2, col3 = st.columns(3)
                    with col1:
                        st.metric("Total Genes", len(json_data))
                    with col2:
                        unique_chromosomes = len(set(g["properties"]["chromosome_location"] for g in json_data))
                        st.metric("Unique Chromosomes", unique_chromosomes)
                    with col3:
                        process_counts = [len(g["properties"]["biological_processes"]) for g in json_data]
                        avg_processes = sum(process_counts) / len(process_counts) if process_counts else 0
                        st.metric("Avg Processes", f"{avg_processes:.1f}")

        except Exception as e:
            st.error(f"Error loading {data_type.lower()} properties: {str(e)}")

    def display_table(self, graph, graph_type):
        st.write(f"### {graph_type} Table")
        edge_data = []
        if graph_type == "Hypergraph":
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