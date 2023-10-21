use std::sync::Arc;

use crate::{extractors::Json, store::AppState};
use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    redoc::Redoc,
};
use axum::{response::IntoResponse, Extension, Router};

pub fn docs_routes(state: AppState) -> ApiRouter<AppState> {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::gen::infer_responses(true);
    let router = ApiRouter::new()
        .api_route_with(
            "/docs",
            get_with(
                Redoc::new("/docs/private/api.json")
                    .with_title("API Specification")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/docs/private/api.json", get(serve_docs))
        .with_state(state);

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(
    Extension(api): Extension<Arc<OpenApi>>,
) -> impl IntoApiResponse {
    Json(api).into_response()
}
