use axum::response::IntoResponse;

pub struct ApiRoute {
    pub method: &'static str,
    pub path: &'static str,
    pub handler: fn() -> Box<dyn IntoResponse + Send + Sync>, // <- This is what the macro expects
}

inventory::collect!(ApiRoute);