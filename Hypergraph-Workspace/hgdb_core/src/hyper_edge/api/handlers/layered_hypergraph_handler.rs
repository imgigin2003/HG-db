use crate::hyper_edge::api::client::layered_service_client::LayeredServiceClient;
use crate::hyper_edge::dto::layered_hypergraph_dto::LayeredHypergraphCreateDto;
use crate::hyper_edge::dto::layered_visualization_request::LayeredVisualizationRequestDto;
use crate::hyper_edge::services::h_graph_service::LayeredHypergraphService;
use crate::hyper_edge::mapper::layered_visualization_mapper;
use crate::hyper_edge::mapper::normalize_mapper;
use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub layered_hypergraph_service: Arc<LayeredHypergraphService>,
    pub layered_service_client: Arc<LayeredServiceClient>,
}

pub async fn create_layered_hypergraph(
    state: web::Data<Arc<AppState>>,
    dto: web::Json<LayeredHypergraphCreateDto>,
) -> impl Responder {
    match state
        .layered_hypergraph_service
        .create_layered_hypergraph(dto.into_inner())
        .await
    {
        Ok(response_dto) => {
            if let Err(e) = state
                .layered_service_client
                .send_for_visualization(&response_dto)
                .await
            {
                eprintln!("Failed to send to layered service: {}", e);
            }
            HttpResponse::Created().json(response_dto)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn get_layered_hypergraph(
    state: web::Data<Arc<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match state
        .layered_hypergraph_service
        .get_layered_hypergraph(&id)
        .await
    {
        Some(response_dto) => HttpResponse::Ok().json(response_dto),
        None => HttpResponse::NotFound().body("Layered hypergraph not found"),
    }
}

pub async fn update_layered_hypergraph(
    state: web::Data<Arc<AppState>>,
    path: web::Path<String>,
    dto: web::Json<LayeredHypergraphCreateDto>,
) -> impl Responder {
    let id = path.into_inner();
    match state
        .layered_hypergraph_service
        .update_layered_hypergraph(&id, dto.into_inner())
        .await
    {
        Ok(response_dto) => {
            if let Err(e) = state
                .layered_service_client
                .send_for_visualization(&response_dto)
                .await
            {
                eprintln!("Failed to send update to layered service: {}", e);
            }
            HttpResponse::Ok().json(response_dto)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn delete_layered_hypergraph(
    state: web::Data<Arc<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match state
        .layered_hypergraph_service
        .delete_layered_hypergraph(&id)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            if e.to_string().contains("not found") || e.to_string().contains("Key") {
                HttpResponse::NotFound().body("Layered hypergraph not found")
            } else {
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}

pub async fn list_layered_hypergraphs(state: web::Data<Arc<AppState>>) -> impl Responder {
    match state
        .layered_hypergraph_service
        .list_layered_hypergraphs()
        .await
    {
        Ok(response_dtos) => HttpResponse::Ok().json(response_dtos),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn visualize_layered_hypergraph(
    state: web::Data<Arc<AppState>>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match state
        .layered_hypergraph_service
        .get_layered_hypergraph(&id)
        .await
    {
        Some(response_dto) => {
            match state
                .layered_service_client
                .visualize_directly(&response_dto)
                .await
            {
                Ok(visualization_result) => HttpResponse::Ok().json(visualization_result),
                Err(e) => {
                    eprintln!("Failed to visualize layered hypergraph: {}", e);
                    HttpResponse::ServiceUnavailable()
                        .body(format!("Layered visualization service error: {}", e))
                }
            }
        }
        None => HttpResponse::NotFound().body("Layered hypergraph not found"),
    }
}

pub async fn visualize_from_json(
    state: web::Data<Arc<AppState>>,
    request: web::Json<LayeredVisualizationRequestDto>,
) -> impl Responder {
    match layered_visualization_mapper::transform_visualization_request(request.into_inner()) {
        Ok(hypergraph_dto) => {
            // Create and save hypergraph
            match state.layered_hypergraph_service.create_layered_hypergraph(hypergraph_dto).await {
                Ok(response_dto) => {
                    // Send to layered service for visualization
                    match state.layered_service_client.visualize_directly(&response_dto).await {
                        Ok(visualization_result) => {
                            HttpResponse::Ok().json(json!({
                                "status": "success",
                                "hypergraph": response_dto,
                                "visualization": visualization_result,
                                "message": "Hypergraph created and sent for visualization successfully"
                            }))
                        }
                        Err(e) => {
                            HttpResponse::Ok().json(json!({
                                "status": "partial_success",
                                "hypergraph": response_dto,
                                "message": format!("Hypergraph created but visualization failed: {}", e)
                            }))
                        }
                    }
                }
                Err(e) => HttpResponse::BadRequest().body(format!("Failed to create hypergraph: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest().body(format!("Invalid input data: {}", e)),
    }
}

pub async fn validate_json_input(
    request: web::Json<LayeredVisualizationRequestDto>,
) -> impl Responder {
    match layered_visualization_mapper::transform_visualization_request(request.into_inner()) {
        Ok(hypergraph_dto) => {
            HttpResponse::Ok().json(json!({
                "status": "valid",
                "hypergraph": hypergraph_dto,
                "message": "Input JSON is valid and can be processed"
            }))
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "status": "invalid",
            "error": e.to_string(),
            "message": "Input JSON validation failed"
        })),
    }
}

/// Best-effort normalize: accept an arbitrary uploaded JSON document and coerce it into
/// the canonical hypergraph edge format consumed by the rest of the pipeline.
/// Stateless — does not touch RocksDB — so it works even without a database.
pub async fn normalize_hypergraph(body: web::Json<Value>) -> impl Responder {
    match normalize_mapper::normalize(body.into_inner()) {
        Ok(normalized) => HttpResponse::Ok().json(normalized),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "status": "invalid",
            "error": e.to_string(),
            "message": "Could not normalize the uploaded JSON into a hypergraph",
        })),
    }
}

/// Validate-only variant: same logic, but reports validity without returning the payload.
pub async fn validate_normalize(body: web::Json<Value>) -> impl Responder {
    match normalize_mapper::normalize(body.into_inner()) {
        Ok(normalized) => HttpResponse::Ok().json(json!({
            "status": "valid",
            "edge_count": normalized.edges.len(),
            "message": "Input JSON can be normalized successfully",
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "status": "invalid",
            "error": e.to_string(),
            "message": "Input JSON normalization failed",
        })),
    }
}

// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
