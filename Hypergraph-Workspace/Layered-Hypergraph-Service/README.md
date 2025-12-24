# Hypergraph Layered Viewer
A web app to visualize hypergraph layers in 3D using Three.js and Python.

## Description
This project allows users to view hypergraphs as layers in a 3D interactive web app.  
It uses **Python + Flask** to serve the hypergraph data and **Three.js** to display it in the browser.

## Features
- Load hypergraph data from JSON  
- Display hypergraph layers in 3D  
- Read info about nodes when clicking on them  

## Requirements
- Python 3.x  
- pip (Python package manager)  
- Node.js  
- A modern web browser (Chrome, Firefox, Edge)  

## Installation

1. **Clone this repository**
   ```bash
   git clone https://github.com/yourusername/hypergraph-layered-viewer.git
   cd hypergraph-layered-viewer
2.Install Python dependencies

pip install -r requirements.txt


3.Start the Flask server

python app.py


4.Open your browser and go to:

http://localhost:5000

##JSON format
the app expects this JSON format:

{
  "layers": [
    {
      "nodes": ["A", "B", "C"],
      "edges": [["A", "B"], ["B", "C"]]
    }
  ]
}
