from fastapi import APIRouter, HTTPException
from app.models.layered import LayeredHypergraphRequest
from app.services import renderer

router = APIRouter()

@router.post("/render-layered")
def render_layered(req: LayeredHypergraphRequest):
    return renderer.render_layered_hypergraph(req.layers)


@router.get("/api/hypergraph")
def get_current_hypergraph():
    scene = renderer.get_current_scene()
    if scene is None:
        raise HTTPException(404, "No hypergraph loaded")
    return scene

