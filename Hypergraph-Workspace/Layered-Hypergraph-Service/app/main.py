from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from app.api.layered import router as layered_router

app = FastAPI(title="Layered Hypergraph Service")

app.include_router(layered_router, prefix="/api")

app.mount("/static", StaticFiles(directory="static"), name="static")