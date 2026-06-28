# 🕸️ HG-DB — Hypergraph Explorer

> **Upload it. Normalize it. See it in 3D.** A microservice stack for exploring hypergraphs.

HG-DB takes any hypergraph JSON you throw at it, normalizes it in a fast Rust core, and lets
you explore it across interactive 2D, dual, and layered **3D** views — without you ever having
to hand-format your data.

---

## ✨ Features

### 📤 Upload & Normalize

- **Bring any JSON** — edge lists, node-link graphs, pre-grouped layers, or the canonical format
- **Rust does the work** — the `hgdb_core` service coerces it into one clean, supported format
- **Best-effort & forgiving** — missing ids, layers, and properties are filled with sane defaults
- **Bundled sample** — load demo data with one click to explore the app immediately

### 🔗 Explore

- **Interactive Hypergraph** — static views (filled / outline / minimal), properties, tables
- **Hyperpaths** — highlight curated paths through the graph
- **Dual Hypergraph** — flip nodes and edges and see the dual structure

### 🧊 Layered 3D

- **Three.js viewer** — layers sent to the Layered-Hypergraph-Service render as an interactive 3D scene
- **Inter-layer connectors** — nodes shared across layers are wired together automatically
- **Graceful fallback** — if the 3D service is down, a static matplotlib 3D view is shown instead

### 🎨 Clean UI

- **Purple-themed Streamlit** front end with a focused sidebar
- **Bio datasets** — explore Atoms, Molecules, and Genes as ready-made examples

---

## 🛠️ Tech Stack

| Layer             | Technology                       |
| ----------------- | -------------------------------- |
| Front end         | Streamlit, streamlit-option-menu |
| 2D / static viz   | HyperNetX, NetworkX, Matplotlib  |
| Normalizer (core) | Rust, Actix-web, Serde           |
| 3D service        | FastAPI, Three.js                |
| Data format       | JSON (canonical edge format)     |

---

## 📁 Project Structure

```
Hypergraph-Workspace/
├── hgdb_core/                  # Rust core (normalizer + hypergraph DB)
│   ├── src/
│   │   └── hyper_edge/
│   │       ├── mapper/normalize_mapper.rs   # best-effort JSON → canonical
│   │       ├── api/                          # actix handlers & routes
│   │       └── ...
│   ├── Config.toml             # ports, db path, layered-service URL
│   └── hg_app/                 # Streamlit front end
│       ├── py_scripts/
│       │   ├── main.py                 # entry point (page config + theme)
│       │   ├── app_manager.py          # sidebar, tabs, upload flow
│       │   ├── visualization_manager.py# 2D/3D rendering + service calls
│       │   ├── hypergraph.py           # canonical parser + 2D drawing
│       │   ├── dual_hypergraph.py
│       │   ├── layered_hypergraph.py   # matplotlib 3D fallback
│       │   └── data_loader.py
│       ├── .streamlit/config.toml      # theme
│       └── testing/test.json           # canonical sample data
└── Layered-Hypergraph-Service/ # FastAPI + Three.js 3D viewer
    ├── app/main.py             # /render, /api/hypergraph, /viewer
    ├── templates/index.html
    └── static/                 # Three.js viewer (js, css, vendor)
```

---

## 🔄 How It Works

```
Streamlit (Upload tab)
  ── raw JSON ──▶  Rust hgdb_core (:8080)  POST /api/normalize
  ◀── canonical edges ──
Streamlit: store in session → drive 2D / dual / table views
  ── canonical edges ──▶  Layered service (:8000)  POST /render
        builds {layers, node_to_layers}, persists, returns viewer_url
  Streamlit embeds the Three.js viewer (it fetches GET /api/hypergraph)
```

The **canonical format** is the shape in [`testing/test.json`](testing/test.json): a list of
`edges`, each with `id`, `name`, `main_properties`, `traversable`, `directed`,
`head_hyper_nodes`, `tail_hyper_nodes`, and `layer`.

---

## ⚡ Quick Start

Run the three services in three terminals.

**Terminal 1 — Rust core** (http://localhost:8080):

```bash
cd hgdb_core
cargo run --release
```

**Terminal 2 — Layered 3D service** (http://localhost:8000):

```bash
cd Layered-Hypergraph-Service
python -m venv venv && source venv/bin/activate
pip install -r requirements.txt
uvicorn app.main:app --port 8000
```

**Terminal 3 — Streamlit app** (http://localhost:8501):

```bash
cd hgdb_core/hg_app
python -m venv venv && source venv/bin/activate
pip install streamlit hypernetx networkx matplotlib pandas requests streamlit-option-menu
export PYTHONPATH="$PWD/py_scripts"
streamlit run py_scripts/main.py
```

Open the app, go to **Upload Data**, and drop in a JSON file (or click _Load sample_).

---

## 🗺️ API Routes

**Rust core** (`hgdb_core`, `:8080`):

```
POST /api/normalize            # arbitrary JSON → canonical edge format
POST /api/normalize/validate   # validate only (no payload returned)
GET  /api/health
```

**Layered 3D service** (`:8000`):

```
POST /render                   # canonical edges → 3D scene (main entry)
POST /render-layered           # pre-grouped {layers:{...}} payload
GET  /api/hypergraph           # current scene {layers, node_to_layers}
GET  /viewer                   # Three.js viewer
GET  /health
```

---

## ⚙️ Configuration

`hgdb_core/Config.toml`:

| Key                     | Default                 | Purpose                                          |
| ----------------------- | ----------------------- | ------------------------------------------------ |
| `api.host` / `api.port` | `127.0.0.1` / `8080`    | Where the Rust core listens                      |
| `database.db_path`      | local RocksDB path      | Hypergraph store (falls back to temp if missing) |
| `layered_service.url`   | `http://localhost:8000` | The 3D viewer service                            |

---

## 🤝 Contributing

Format Rust with `cargo fmt` and Python with `black`. Found a bug or have an idea? Open a PR.

---

_Bring your hypergraph. We'll make it make sense. 🕸️✨_
