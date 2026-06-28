//! Best-effort normalizer.
//!
//! Takes an arbitrary uploaded JSON value and coerces it into the canonical
//! hypergraph format used everywhere downstream (see `hg_app/testing/test.json`):
//!
//! ```json
//! { "edges": [ { "id", "name", "main_properties", "traversable",
//!               "directed", "head_hyper_nodes", "tail_hyper_nodes", "layer" } ],
//!   "hypergraph_id": "", "hypergraph_name": "" }
//! ```
//!
//! It accepts several common shapes and does its best to map them onto that form:
//!   1. Canonical edge list:   `{ "edges": [ { "head_hyper_nodes": [...] } ] }`  (or a bare `[...]`)
//!   2. Node-link graph:       `{ "nodes": [...], "links" | "edges": [ { "source", "target" } ] }`
//!   3. Layers payload:        `{ "layers": { "0": { "e1": ["v1","v2"] } } }`
//!   4. Simple edge list:      `{ "edges": [ { "id", "nodes": [...], "layer"? } ] }`

use crate::hyper_edge::dto::layered_visualization_request::{
    LayeredVisualizationRequestDto, NodeInputDto, PropertyInputDto, SimpleEdgeInputDto,
};
use serde_json::Value;
use std::error::Error;

/// Normalize an arbitrary JSON value into the canonical hypergraph format.
pub fn normalize(value: Value) -> Result<LayeredVisualizationRequestDto, Box<dyn Error>> {
    let edges = match &value {
        Value::Array(items) => edges_from_edge_array(items)?,
        Value::Object(map) => {
            if let Some(Value::Object(layers)) = map.get("layers") {
                edges_from_layers(layers)?
            } else if let Some(Value::Array(items)) = map.get("edges") {
                // Could be canonical, simple, or node-link depending on element shape.
                if items.iter().any(is_node_link_edge) {
                    edges_from_links(items)?
                } else {
                    edges_from_edge_array(items)?
                }
            } else if let Some(Value::Array(items)) = map.get("links") {
                edges_from_links(items)?
            } else {
                return Err("Unrecognized JSON shape: expected one of \
                    `edges`, `links`, or `layers`."
                    .into());
            }
        }
        _ => return Err("Top-level JSON must be an object or an array.".into()),
    };

    if edges.is_empty() {
        return Err("No edges could be derived from the input.".into());
    }

    let (hypergraph_id, hypergraph_name) = match &value {
        Value::Object(map) => (
            string_field(map.get("hypergraph_id")),
            string_field(map.get("hypergraph_name")),
        ),
        _ => (String::new(), String::new()),
    };

    Ok(LayeredVisualizationRequestDto {
        edges,
        hypergraph_id,
        hypergraph_name,
    })
}

/// Shapes 1 & 4: an array of edge objects (canonical or `{id, nodes, layer}`).
fn edges_from_edge_array(items: &[Value]) -> Result<Vec<SimpleEdgeInputDto>, Box<dyn Error>> {
    let mut edges = Vec::with_capacity(items.len());
    for (idx, item) in items.iter().enumerate() {
        let obj = item
            .as_object()
            .ok_or_else(|| format!("Edge at index {} is not an object", idx))?;

        let id = string_field(obj.get("id"));
        let id = if id.is_empty() {
            format!("e{}", idx + 1)
        } else {
            id
        };

        // Heads can come from `head_hyper_nodes`, `nodes`, or `head`.
        let head = extract_nodes(
            obj.get("head_hyper_nodes")
                .or_else(|| obj.get("nodes"))
                .or_else(|| obj.get("head")),
        );
        let tail = extract_nodes(obj.get("tail_hyper_nodes").or_else(|| obj.get("tail")));

        let name = {
            let n = string_field(obj.get("name"));
            if n.is_empty() {
                id.clone()
            } else {
                n
            }
        };

        edges.push(SimpleEdgeInputDto {
            id,
            name,
            main_properties: properties(obj.get("main_properties")),
            traversable: bool_field(obj.get("traversable"), true),
            directed: bool_field(obj.get("directed"), !tail.is_empty()),
            head_hyper_nodes: if head.is_empty() { None } else { Some(head) },
            tail_hyper_nodes: if tail.is_empty() { None } else { Some(tail) },
            layer: int_field(obj.get("layer"), 0),
        });
    }
    Ok(edges)
}

/// Shape 2: node-link graph — every link becomes a directed 2-node edge.
fn edges_from_links(items: &[Value]) -> Result<Vec<SimpleEdgeInputDto>, Box<dyn Error>> {
    let mut edges = Vec::with_capacity(items.len());
    for (idx, item) in items.iter().enumerate() {
        let obj = item
            .as_object()
            .ok_or_else(|| format!("Link at index {} is not an object", idx))?;

        let source = string_field(obj.get("source"));
        let target = string_field(obj.get("target"));
        if source.is_empty() || target.is_empty() {
            return Err(format!("Link at index {} is missing `source`/`target`", idx).into());
        }

        let id = {
            let n = string_field(obj.get("id"));
            if n.is_empty() {
                format!("e{}", idx + 1)
            } else {
                n
            }
        };

        edges.push(SimpleEdgeInputDto {
            id: id.clone(),
            name: id,
            main_properties: properties(obj.get("main_properties")),
            traversable: bool_field(obj.get("traversable"), true),
            directed: true,
            head_hyper_nodes: Some(vec![NodeInputDto { id: source }]),
            tail_hyper_nodes: Some(vec![NodeInputDto { id: target }]),
            layer: int_field(obj.get("layer"), 0),
        });
    }
    Ok(edges)
}

/// Shape 3: `{ "layers": { "0": { "e1": ["v1","v2"] } } }`.
fn edges_from_layers(
    layers: &serde_json::Map<String, Value>,
) -> Result<Vec<SimpleEdgeInputDto>, Box<dyn Error>> {
    let mut edges = Vec::new();
    let mut layer_keys: Vec<&String> = layers.keys().collect();
    layer_keys.sort();

    for layer_key in layer_keys {
        let layer = layer_key.parse::<i32>().unwrap_or(0);
        let group = layers
            .get(layer_key)
            .and_then(Value::as_object)
            .ok_or_else(|| format!("Layer `{}` is not an object of edges", layer_key))?;

        for (edge_id, nodes_value) in group {
            let head = extract_nodes(Some(nodes_value));
            edges.push(SimpleEdgeInputDto {
                id: edge_id.clone(),
                name: edge_id.clone(),
                main_properties: default_properties(),
                traversable: true,
                directed: false,
                head_hyper_nodes: if head.is_empty() { None } else { Some(head) },
                tail_hyper_nodes: None,
                layer,
            });
        }
    }
    Ok(edges)
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn is_node_link_edge(item: &Value) -> bool {
    item.as_object()
        .map(|o| o.contains_key("source") && o.contains_key("target"))
        .unwrap_or(false)
}

/// Accepts `["v1","v2"]`, `[{"id":"v1"}]`, or a single `"v1"`.
fn extract_nodes(value: Option<&Value>) -> Vec<NodeInputDto> {
    match value {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|n| match n {
                Value::String(s) => Some(NodeInputDto { id: s.clone() }),
                Value::Object(o) => o
                    .get("id")
                    .and_then(Value::as_str)
                    .map(|s| NodeInputDto { id: s.to_string() }),
                _ => None,
            })
            .collect(),
        Some(Value::String(s)) => vec![NodeInputDto { id: s.clone() }],
        _ => Vec::new(),
    }
}

/// Parse `main_properties` if present and well-formed, otherwise fall back to the default.
fn properties(value: Option<&Value>) -> Vec<PropertyInputDto> {
    if let Some(Value::Array(items)) = value {
        let parsed: Vec<PropertyInputDto> = items
            .iter()
            .filter_map(|p| {
                let o = p.as_object()?;
                let value = match o.get("value") {
                    Some(Value::Array(vs)) => vs
                        .iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect(),
                    Some(Value::String(s)) => vec![s.clone()],
                    _ => Vec::new(),
                };
                Some(PropertyInputDto {
                    key: o.get("key").and_then(Value::as_str).unwrap_or("type").to_string(),
                    p_type: o
                        .get("p_type")
                        .and_then(Value::as_str)
                        .unwrap_or("Simple")
                        .to_string(),
                    value,
                })
            })
            .collect();
        if !parsed.is_empty() {
            return parsed;
        }
    }
    default_properties()
}

fn default_properties() -> Vec<PropertyInputDto> {
    vec![PropertyInputDto {
        key: "type".to_string(),
        p_type: "Simple".to_string(),
        value: vec!["linked".to_string()],
    }]
}

fn string_field(value: Option<&Value>) -> String {
    value.and_then(Value::as_str).unwrap_or("").to_string()
}

fn bool_field(value: Option<&Value>, default: bool) -> bool {
    match value {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => s.eq_ignore_ascii_case("true"),
        _ => default,
    }
}

fn int_field(value: Option<&Value>, default: i32) -> i32 {
    match value {
        Some(Value::Number(n)) => n.as_i64().map(|v| v as i32).unwrap_or(default),
        Some(Value::String(s)) => s.parse::<i32>().unwrap_or(default),
        _ => default,
    }
}
