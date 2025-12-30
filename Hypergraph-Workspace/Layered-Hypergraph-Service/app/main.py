from fastapi import FastAPI, Request
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from pathlib import Path
from app.api.layered import router as layered_router



app = FastAPI()


BASE_DIR = Path(__file__).resolve().parent.parent
print((BASE_DIR / "static").exists())  # should print True
print(list((BASE_DIR / "static").iterdir()))  # should list folders like css, js

app.mount("/static", StaticFiles(directory=BASE_DIR / "static"), name="static")

templates = Jinja2Templates(directory=BASE_DIR / "templates")

app.include_router(layered_router)


@app.get("/viewer")
def viewer(request: Request):
    return templates.TemplateResponse(
        "index.html",
        {
            "request": request
        }
    )
