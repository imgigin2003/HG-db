import json
from pathlib import Path
import streamlit as st

class DataLoader:
    def get_resources_root(self):
         """Helper function to get the project root relative to hgdb_core location"""
         return Path(__file__).parent.parent.parent # Goes up to hgdb_core folder
    
    def get_statics_root(self):
         """Helper function to get the project root relative to hg_app location"""
         return Path(__file__).parent.parent # Goes up to hg_app folder

    def load_atoms_data(self):
        FILE_PATH = self.get_resources_root() / "resources" / "Atoms.json"
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
    
    def load_mols_data(self):
        FILE_PATH = self.get_resources_root() / "resources" / "Mols.json"
        try:
            with open(FILE_PATH, "r") as f:
                molecules = json.load(f)
            
            hyperedges = {}
            for mol in molecules:
                formula = mol['properties'][0]['molecular_formula']
                atoms = self.parse_formula(formula)
                nodes = set(atoms.keys())  # Use individual elements as nodes
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
        
    def parse_formula(self, formula):
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

    def load_genes_data(self):
        """Load genes data from JSON file and format for hypergraph visualization"""
        FILE_PATH = self.get_resources_root() / "resources" / "Genes.json"
        try:
            with open(FILE_PATH, "r") as f:
                genes_data = json.load(f)
            
            hyperedges = {}
            for gene in genes_data["Genes"]:
                gene_name = gene["name"]
                hyperedges[gene_name] = {
                    "nodes": {gene_name},
                    "layer": 0,  # Default layer
                    "traversable": True,
                    "directed": False,
                    "head": {gene_name},
                    "tail": set(),
                    "type": "gene",
                    "properties": gene["properties"]  # Store all properties
                }
            return hyperedges
        except Exception as e:
            st.error(f"Error loading Genes data: {e}")
            return None

    def load_paths_data(self):
        FILE_PATH = self.get_resources_root() / "resources" / "paths.json"
        try:
            with open(FILE_PATH, "r") as f:
                paths_data = json.load(f)
            return paths_data.get("Hyperpaths", [])
        except FileNotFoundError:
            st.error(f"paths.json not found at {FILE_PATH}")
            return []
        except Exception as e:
            st.error(f"Error loading paths data: {str(e)}")
            return []
