from pydantic import BaseModel
from typing import Dict, List, Optional

class Hyperedge(BaseModel):
    nodes: List[str]
    layer: Optional[int] = 0

class LayeredHypergraphRequest(BaseModel):
    layers: Dict[int, Dict[str, List[str]]]