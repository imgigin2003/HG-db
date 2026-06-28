mod db_config;
mod hyper_edge;

use crate::hyper_edge::api::client::layered_service_client::LayeredServiceClient;
use crate::hyper_edge::api::handlers::layered_hypergraph_handler::AppState;
use crate::hyper_edge::api::routes::layered_hypergraph_routes;
use crate::hyper_edge::repository::h_graph_repository::LayeredHypergraphRepository;
use crate::hyper_edge::repository::Repository;
use crate::hyper_edge::services::h_graph_service::LayeredHypergraphService;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = db_config::get_config().expect("Failed to load configuration");

    // Create repository. If the configured RocksDB path is unavailable we fall back to a
    // temp directory instead of crashing, so stateless endpoints (e.g. /api/normalize)
    // keep working without a provisioned database.
    let repository = match LayeredHypergraphRepository::new(&config.database.db_path) {
        Ok(repo) => repo,
        Err(e) => {
            let fallback = std::env::temp_dir().join("hgdb_rocksdb");
            eprintln!(
                "Warning: could not open RocksDB at '{}' ({}). Falling back to '{}'.",
                config.database.db_path,
                e,
                fallback.display()
            );
            LayeredHypergraphRepository::new(&fallback.to_string_lossy())
                .expect("Failed to create fallback repository")
        }
    };
    let repository_arc = Arc::new(repository);

    // Create service
    let service = LayeredHypergraphService::new(repository_arc);
    let service_arc = Arc::new(service);

    // Create layered service client
    let layered_service_client = LayeredServiceClient::new(&config.layered_service.url);
    let layered_service_client_arc = Arc::new(layered_service_client);

    // Create app state
    let state = Arc::new(AppState {
        layered_hypergraph_service: service_arc,
        layered_service_client: layered_service_client_arc,
    });

    // Check if layered service is available
    match state.layered_service_client.health_check().await {
        Ok(true) => println!("Layered service is available"),
        _ => println!("Warning: Layered service is not available. Visualization may fail."),
    }

    let state_data = web::Data::new(state);
    let host = config.api.host.clone();
    let port = config.api.port;

    println!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(state_data.clone())
            .wrap(cors)
            .configure(layered_hypergraph_routes::configure_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
