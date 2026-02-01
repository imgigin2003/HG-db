# How to run

To install Python dependencies use any `pyproject.toml` compatible package
manager (e.g. `uv venv; uv sync`).

To run, add the scripts to Python path (assuming you are in `hg_app`):

``` sh
export PYTHONPATH="$PWD/py_scripts"
```

# Contribution

Every Python source must be formatted via `black`.
