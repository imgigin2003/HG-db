"""Layered Hypergraph Visualization Service.

Receives a hypergraph (canonical edge format, see hgdb_core/hg_app/testing/test.json),
turns it into a layered scene description, and serves a Three.js viewer that renders it
in 3D. The Streamlit app POSTs to /render; the viewer fetches GET /api/hypergraph.
"""

from fastapi import FastAPI, Request, HTTPException
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from fastapi.responses import JSONResponse, HTMLResponse
from fastapi.middleware.cors import CORSMiddleware
from pathlib import Path
import hashlib
import json
import time

app = FastAPI(title="Layered Hypergraph Visualization Service")

# Streamlit runs on a different port, so allow cross-origin requests.
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # tighten this in production
    allow_methods=["*"],
    allow_headers=["*"],
)

BASE_DIR = Path(__file__).resolve().parent.parent
DATA_FILE = BASE_DIR / "data_file.json"

app.mount("/static", StaticFiles(directory=BASE_DIR / "static"), name="static")
templates = Jinja2Templates(directory=BASE_DIR / "templates")

# In-memory copy of the latest scene: {"layers": {...}, "node_to_layers": {...}}
current_scene: dict | None = None


# ─── Scene building ────────────────────────────────────────────────────────────
def _nodes_of(edge: dict) -> list[str]:
    """Collect every node id referenced by an edge (head + tail)."""
    nodes: list[str] = []
    for key in ("head_hyper_nodes", "tail_hyper_nodes"):
        for node in edge.get(key) or []:
            if isinstance(node, dict) and "id" in node:
                nodes.append(node["id"])
            elif isinstance(node, str):
                nodes.append(node)
    return nodes


def build_scene_from_edges(edges: list[dict]) -> dict:
    """Canonical edge list  →  {layers, node_to_layers} for the Three.js viewer."""
    layers: dict[str, dict[str, list[str]]] = {}
    node_to_layers: dict[str, set[int]] = {}

    for idx, edge in enumerate(edges):
        layer = int(edge.get("layer", 0))
        edge_id = edge.get("id") or f"e{idx + 1}"
        nodes = _nodes_of(edge)

        layers.setdefault(str(layer), {})[edge_id] = nodes
        for node in nodes:
            node_to_layers.setdefault(node, set()).add(layer)

    return _finalize_scene(layers, node_to_layers)


def build_scene_from_layers(layers_payload: dict) -> dict:
    """Pre-grouped {layer: {edge: [nodes]}}  →  scene with derived node_to_layers."""
    node_to_layers: dict[str, set[int]] = {}
    for layer_id, group in layers_payload.items():
        for nodes in group.values():
            for node in nodes:
                node_to_layers.setdefault(node, set()).add(int(layer_id))
    return _finalize_scene(layers_payload, node_to_layers)


def _finalize_scene(layers: dict, node_to_layers: dict[str, set[int]]) -> dict:
    return {
        "layers": layers,
        "node_to_layers": {node: sorted(ls) for node, ls in node_to_layers.items()},
    }


def _persist(scene: dict) -> None:
    global current_scene
    current_scene = scene
    DATA_FILE.write_text(json.dumps(scene, indent=2))


def _scene_id() -> str:
    return hashlib.md5(str(time.time()).encode()).hexdigest()[:8]


# ─── Root ───────────────────────────────────────────────────────────────────────
@app.get("/")
async def root():
    return HTMLResponse(
        """
    <!DOCTYPE html><html><head><title>Layered Hypergraph Service</title>
    <style>
      body{font-family:system-ui,Arial,sans-serif;text-align:center;padding:50px;
           background:linear-gradient(135deg,#7E60BF,#A594F9);color:white;}
      .box{background:rgba(255,255,255,.12);padding:40px;border-radius:20px;
           max-width:600px;margin:auto;backdrop-filter:blur(10px);}
      a{display:inline-block;margin:10px;padding:12px 24px;background:white;
        color:#7E60BF;text-decoration:none;border-radius:10px;font-weight:bold;}
      code{background:rgba(255,255,255,.2);padding:2px 6px;border-radius:4px;}
      ul{text-align:left;}
    </style></head><body>
    <div class="box">
      <h1>🌐 Layered Hypergraph Visualization</h1>
      <div><a href="/viewer">🎨 3D Viewer</a><a href="/health">🏥 Health</a><a href="/docs">📚 Docs</a></div>
      <ul>
        <li><code>POST /render</code> – canonical edge format → 3D scene (main entry)</li>
        <li><code>POST /render-layered</code> – pre-grouped layers payload</li>
        <li><code>GET  /viewer</code> – Three.js viewer</li>
        <li><code>GET  /api/hypergraph</code> – current scene (layers + node_to_layers)</li>
      </ul>
    </div></body></html>
    """
    )


# ─── Viewer ─────────────────────────────────────────────────────────────────────
@app.get("/viewer")
async def viewer(request: Request):
    return templates.TemplateResponse("index.html", {"request": request})


# ─── Health ───────────────────────────────────────────────────────────────────
@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "service": "Layered Hypergraph Visualization",
        "version": "2.0.0",
    }


# ─── /render  (Streamlit → FastAPI, canonical edge format) ──────────────────────
@app.post("/render")
async def render(request: Request):
    """Accepts the canonical hypergraph: { "edges": [ ... ] } (test.json format)."""
    try:
        body = await request.json()
        edges = body.get("edges", body if isinstance(body, list) else [])
        if not edges:
            raise ValueError("Payload contains no `edges`.")

        _persist(build_scene_from_edges(edges))

        scene_id = _scene_id()
        return {
            "status": 200,
            "scene_id": scene_id,
            "viewer_url": f"http://127.0.0.1:8000/viewer?scene={scene_id}",
            "layers_count": len(current_scene["layers"]),
            "message": "Scene generated successfully",
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Error rendering hypergraph: {e}")


# ─── /render-layered  (pre-grouped layers payload, back-compat) ─────────────────
@app.post("/render-layered")
async def render_layered(request: Request):
    """Accepts: { "layers": { "0": { "edgeA": ["n1","n2"] } } }."""
    try:
        body = await request.json()
        layers = body.get("layers", body)
        _persist(build_scene_from_layers(layers))

        scene_id = _scene_id()
        return {
            "status": 200,
            "scene_id": scene_id,
            "viewer_url": f"http://127.0.0.1:8000/viewer?scene={scene_id}",
            "layers_count": len(layers),
            "message": "Scene generated successfully",
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Error processing layered request: {e}")


# ─── /api/hypergraph  (the viewer fetches this) ─────────────────────────────────
@app.get("/api/hypergraph")
async def get_hypergraph():
    if current_scene:
        return JSONResponse(content=current_scene)
    raise HTTPException(status_code=404, detail="No hypergraph data loaded")


# ─── Startup: reload persisted scene ────────────────────────────────────────────
@app.on_event("startup")
async def load_existing_data():
    global current_scene
    if DATA_FILE.exists():
        try:
            data = json.loads(DATA_FILE.read_text())
            # Tolerate older files that only stored {"layers": {...}}.
            current_scene = (
                data
                if "node_to_layers" in data
                else build_scene_from_layers(data.get("layers", {}))
            )
            print(f"✅ Loaded persisted scene from {DATA_FILE}")
        except Exception as e:
            print(f"⚠️  Could not load persisted data: {e}")
    else:
        print("ℹ️  No persisted hypergraph data found.")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
