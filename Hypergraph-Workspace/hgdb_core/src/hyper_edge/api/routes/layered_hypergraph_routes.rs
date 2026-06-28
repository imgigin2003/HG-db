use crate::hyper_edge::api::handlers::layered_hypergraph_handler;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/normalize")
                    .route(web::post().to(layered_hypergraph_handler::normalize_hypergraph)),
            )
            .service(
                web::resource("/normalize/validate")
                    .route(web::post().to(layered_hypergraph_handler::validate_normalize)),
            )
            .service(
                web::resource("/layered-hypergraphs/visualize-from-json")
                    .route(web::post().to(layered_hypergraph_handler::visualize_from_json)),
            )
            .service(
                web::resource("/layered-hypergraphs/validate-json")
                    .route(web::post().to(layered_hypergraph_handler::validate_json_input)),
            )
            .service(
                web::resource("/layered-hypergraphs/{id}/visualize").route(
                    web::post().to(layered_hypergraph_handler::visualize_layered_hypergraph),
                ),
            )
            .service(
                web::resource("/layered-hypergraphs/{id}")
                    .route(web::get().to(layered_hypergraph_handler::get_layered_hypergraph))
                    .route(web::put().to(layered_hypergraph_handler::update_layered_hypergraph))
                    .route(web::delete().to(layered_hypergraph_handler::delete_layered_hypergraph)),
            )
            .service(
                web::resource("/layered-hypergraphs")
                    .route(web::post().to(layered_hypergraph_handler::create_layered_hypergraph))
                    .route(web::get().to(layered_hypergraph_handler::list_layered_hypergraphs)),
            )
            .route(
                "/health",
                web::get().to(layered_hypergraph_handler::health_check),
            ),
    );
}
