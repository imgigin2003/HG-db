use crate::hyper_edge::api::client::layered_service_client::LayeredServiceClient;
use crate::hyper_edge::dto::layered_hypergraph_dto::LayeredHypergraphCreateDto;
use crate::hyper_edge::services::h_graph_service::LayeredHypergraphService;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
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
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
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

// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
