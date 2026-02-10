from fastapi import FastAPI, Request, HTTPException
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from fastapi.responses import JSONResponse, HTMLResponse
from pathlib import Path
import json

app = FastAPI(title="Layered Hypergraph Visualization Service")

BASE_DIR = Path(__file__).resolve().parent.parent

# Mount static files
app.mount("/static", StaticFiles(directory=BASE_DIR / "static"), name="static")

# Templates
templates = Jinja2Templates(directory=BASE_DIR / "templates")

# Global variable to store hypergraph data
current_hypergraph_data = None

@app.get("/")
async def root():
    """Root endpoint redirects to viewer"""
    return HTMLResponse("""
    <!DOCTYPE html>
    <html>
    <head>
        <title>Layered Hypergraph Visualization</title>
        <style>
            body { 
                font-family: Arial, sans-serif; 
                text-align: center; 
                padding: 50px; 
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: white;
            }
            .container {
                background: rgba(255, 255, 255, 0.1);
                padding: 40px;
                border-radius: 20px;
                backdrop-filter: blur(10px);
                max-width: 600px;
                margin: 0 auto;
            }
            h1 { font-size: 2.5em; margin-bottom: 20px; }
            .links { margin-top: 30px; }
            a {
                display: inline-block;
                margin: 10px;
                padding: 12px 24px;
                background: white;
                color: #764ba2;
                text-decoration: none;
                border-radius: 10px;
                font-weight: bold;
                transition: transform 0.3s;
            }
            a:hover {
                transform: translateY(-3px);
                box-shadow: 0 10px 20px rgba(0,0,0,0.2);
            }
            .endpoints {
                text-align: left;
                background: rgba(255,255,255,0.1);
                padding: 20px;
                border-radius: 10px;
                margin-top: 30px;
            }
            code {
                background: rgba(255,255,255,0.2);
                padding: 2px 6px;
                border-radius: 4px;
                font-family: monospace;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>🌐 Layered Hypergraph Visualization Service</h1>
            <p>3D Visualization of Layered Hypergraphs with D3.js</p>
            
            <div class="links">
                <a href="/viewer">🎨 Open 3D Viewer</a>
                <a href="/health">🏥 Health Check</a>
                <a href="/docs">📚 API Documentation</a>
            </div>
            
            <div class="endpoints">
                <h3>Available Endpoints:</h3>
                <ul>
                    <li><code>GET /</code> - This page</li>
                    <li><code>GET /viewer</code> - 3D Visualization Interface</li>
                    <li><code>GET /health</code> - Service health check</li>
                    <li><code>POST /api/visualize</code> - Send hypergraph for visualization</li>
                    <li><code>POST /api/visualize/direct</code> - Direct visualization endpoint</li>
                    <li><code>GET /api/current-data</code> - Get current hypergraph data</li>
                    <li><code>GET /docs</code> - Swagger UI documentation</li>
                </ul>
            </div>
        </div>
    </body>
    </html>
    """)

@app.get("/viewer")
async def viewer(request: Request):
    """Render the main 3D visualization page"""
    return templates.TemplateResponse(
        "index.html",
        {
            "request": request,
            "hypergraph_data": current_hypergraph_data or {}
        }
    )

@app.get("/health")
async def health_check():
    """Health check endpoint for service monitoring"""
    return {
        "status": "healthy",
        "service": "Layered Hypergraph Visualization",
        "version": "1.0.0"
    }

@app.get("/api/current-data")
async def get_current_data():
    """Get the currently loaded hypergraph data"""
    if current_hypergraph_data:
        return JSONResponse(content=current_hypergraph_data)
    else:
        raise HTTPException(status_code=404, detail="No hypergraph data loaded")

@app.post("/api/visualize")
async def visualize_hypergraph(request: Request):
    """Receive hypergraph data from Rust API and store it for visualization"""
    global current_hypergraph_data
    
    try:
        data = await request.json()
        current_hypergraph_data = data
        
        # Save to file for persistence
        data_file = BASE_DIR / "data_file.json"
        with open(data_file, 'w') as f:
            json.dump(data, f, indent=2)
        
        return {
            "status": "success",
            "message": "Hypergraph data received and stored successfully",
            "data_size": len(str(data)),
            "viewer_url": "http://localhost:3000/viewer"
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Error processing request: {str(e)}")

@app.post("/api/visualize/direct")
async def visualize_direct(request: Request):
    """Direct visualization endpoint with immediate response"""
    global current_hypergraph_data
    
    try:
        data = await request.json()
        current_hypergraph_data = data
        
        # Save to file
        data_file = BASE_DIR / "data_file.json"
        with open(data_file, 'w') as f:
            json.dump(data, f, indent=2)
        
        # Return enhanced response
        return {
            "status": "success",
            "message": "Hypergraph visualized successfully",
            "viewer_url": "http://localhost:3000/viewer",
            "data_preview": {
                "id": data.get("id", "unknown"),
                "name": data.get("name", "Unnamed"),
                "layers_count": len(data.get("layers", [])),
                "total_edges": sum(len(layer) for layer in data.get("layers", []))
            } if isinstance(data, dict) else {"type": type(data).__name__}
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Error processing request: {str(e)}")

# Load existing data on startup
@app.on_event("startup")
async def load_existing_data():
    """Load existing data on service startup"""
    global current_hypergraph_data
    data_file = BASE_DIR / "data_file.json"
    
    if data_file.exists():
        try:
            with open(data_file, 'r') as f:
                current_hypergraph_data = json.load(f)
            print(f"✅ Loaded existing hypergraph data from {data_file}")
        except Exception as e:
            print(f"⚠️ Could not load existing data: {e}")
    else:
        print("ℹ️ No existing hypergraph data found")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=3000)