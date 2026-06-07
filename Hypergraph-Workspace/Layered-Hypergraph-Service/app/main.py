from fastapi import FastAPI, Request, HTTPException
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from fastapi.responses import JSONResponse, HTMLResponse
from fastapi.middleware.cors import CORSMiddleware
from pathlib import Path
import json

app = FastAPI(title="Layered Hypergraph Visualization Service")

# ─── CORS ────────────────────────────────────────────────────────────────────
# Streamlit runs on a different port, so we must allow cross-origin requests.
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # tighten this in production
    allow_methods=["*"],
    allow_headers=["*"],
)

BASE_DIR = Path(__file__).resolve().parent.parent

app.mount("/static", StaticFiles(directory=BASE_DIR / "static"), name="static")
templates = Jinja2Templates(directory=BASE_DIR / "templates")

# In-memory store for the latest hypergraph payload
current_hypergraph_data: dict | None = None


# ─── Root ─────────────────────────────────────────────────────────────────────
@app.get("/")
async def root():
    return HTMLResponse("""
    <!DOCTYPE html><html><head><title>Layered Hypergraph Service</title>
    <style>
      body{font-family:Arial,sans-serif;text-align:center;padding:50px;
           background:linear-gradient(135deg,#667eea,#764ba2);color:white;}
      .box{background:rgba(255,255,255,.12);padding:40px;border-radius:20px;
           max-width:600px;margin:auto;backdrop-filter:blur(10px);}
      a{display:inline-block;margin:10px;padding:12px 24px;background:white;
        color:#764ba2;text-decoration:none;border-radius:10px;font-weight:bold;}
      code{background:rgba(255,255,255,.2);padding:2px 6px;border-radius:4px;}
      ul{text-align:left;}
    </style></head><body>
    <div class="box">
      <h1>🌐 Layered Hypergraph Visualization</h1>
      <div><a href="/viewer">🎨 3D Viewer</a><a href="/health">🏥 Health</a><a href="/docs">📚 Docs</a></div>
      <ul>
        <li><code>POST /render-layered</code> – Streamlit → FastAPI (main entry)</li>
        <li><code>POST /api/visualize</code> – alias / Rust entry</li>
        <li><code>GET  /viewer</code> – Three.js viewer</li>
        <li><code>GET  /api/current-data</code> – last received payload</li>
      </ul>
    </div></body></html>
    """)


# ─── Viewer ───────────────────────────────────────────────────────────────────
@app.get("/viewer")
async def viewer(request: Request):
    return templates.TemplateResponse(
        "index.html",
        {"request": request, "hypergraph_data": current_hypergraph_data or {}},
    )


# ─── Health ───────────────────────────────────────────────────────────────────
@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "service": "Layered Hypergraph Visualization",
        "version": "1.0.0",
    }


# ─── /render-layered  (called by Streamlit's send_to_layered_service) ─────────
@app.post("/render-layered")
async def render_layered(request: Request):
    """
    Accepts:  { "layers": { "0": { "edgeA": ["n1","n2"], ... }, "1": {...} } }
    Returns:  { "status": 200, "scene_id": "...", "viewer_url": "..." }
    """
    global current_hypergraph_data

    try:
        body = await request.json()
        layers = body.get("layers", body)  # tolerate both {layers:{}} and bare {}

        current_hypergraph_data = {"layers": layers}

        # Persist so the viewer can reload on refresh
        data_file = BASE_DIR / "data_file.json"
        with open(data_file, "w") as f:
            json.dump(current_hypergraph_data, f, indent=2)

        # Build a simple scene_id so the viewer URL is meaningful
        import hashlib, time

        scene_id = hashlib.md5(f"{time.time()}".encode()).hexdigest()[:8]

        return {
            "status": 200,
            "scene_id": scene_id,
            "viewer_url": f"http://127.0.0.1:8000/viewer?scene={scene_id}",
            "layers_count": len(layers),
            "message": "Scene generated successfully",
        }

    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Error processing layered request: {e}"
        )


# ─── /api/visualize  (called by Rust backend) ─────────────────────────────────
@app.post("/api/visualize")
async def visualize_hypergraph(request: Request):
    global current_hypergraph_data
    try:
        data = await request.json()
        current_hypergraph_data = data

        data_file = BASE_DIR / "data_file.json"
        with open(data_file, "w") as f:
            json.dump(data, f, indent=2)

        return {
            "status": "success",
            "message": "Hypergraph data received",
            "viewer_url": "http://localhost:8000/viewer",
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


# ─── /api/current-data ────────────────────────────────────────────────────────
@app.get("/api/current-data")
async def get_current_data():
    if current_hypergraph_data:
        return JSONResponse(content=current_hypergraph_data)
    raise HTTPException(status_code=404, detail="No hypergraph data loaded")


# ─── Startup: reload persisted data ──────────────────────────────────────────
@app.on_event("startup")
async def load_existing_data():
    global current_hypergraph_data
    data_file = BASE_DIR / "data_file.json"
    if data_file.exists():
        try:
            with open(data_file) as f:
                current_hypergraph_data = json.load(f)
            print(f"✅ Loaded persisted hypergraph from {data_file}")
        except Exception as e:
            print(f"⚠️  Could not load persisted data: {e}")
    else:
        print("ℹ️  No persisted hypergraph data found.")


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
