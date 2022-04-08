use std::sync::{Arc, Mutex};

use axum::{extract::Extension, response::IntoResponse, routing::post, Router};

#[derive(Default)]
struct Counter {
    count: usize,
}

pub(crate) fn setup(router: Router) -> Router {
    router
        .nest(
            "/api",
            Router::new().route("/counter", post(server_counter)),
        )
        .layer(Extension(Arc::new(Mutex::new(Counter::default()))))
}

async fn server_counter(Extension(counter): Extension<Arc<Mutex<Counter>>>) -> impl IntoResponse {
    let mut counter = counter.lock().unwrap();
    counter.count += 1;
    format!("{}", counter.count)
}
