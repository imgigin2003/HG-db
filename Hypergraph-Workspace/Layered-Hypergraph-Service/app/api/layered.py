from fastapi import APIRouter
from app.models.layered import LayeredHypergraphRequest
from app.services.renderer import render_layered_hypergraph

router = APIRouter()

@router.post("/render-layered")
def render_layered(req: LayeredHypergraphRequest):
    result = render_layered_hypergraph(req.hyperedges)
    return result
