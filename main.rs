use axum::{
    extract::{Json, Path, Query, Request, State}, 
    http::HeaderMap, 
    routing::{get, post}, 
    Router
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use std::path::PathBuf;
use love_daily::{self as lib, route::get_all_messages};
use love_daily::route as r0out;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {

    let api_router = Router::new()
        .route("/login", post(love_daily::login::login_user))
        .route("/daily_messages", post(r0out::daily_messages))
        .route("/get_all_msgs", post(get_all_messages));

    let legacy_router = Router::new()
        .route("/fun", get(r0out::fun).post(r0out::post_fun))
        .route("/users", post(r0out::post_fun))
        .route("/math_add", post(r0out::math_thingy))
        .route("/math/{num1}/-{num2}", get(r0out::simple_math).post(r0out::simple_math))
        .route("/user_register", post(r0out::user_check))
        .route("/register", post(r0out::user_register))
        .route("/email_update", post(r0out::email_update))
        .route("/user_find", post(r0out::find_user_data))
        .route("/user_delete", post(r0out::delete_user_main))
        .route("/decode_user", post(r0out::decode_user));

    let static_files_service = ServeDir::new("frontend/build")
        .append_index_html_on_directories(true)
        .fallback(ServeDir::new("frontend/build/index.html"));

    let app = Router::new()
        .nest("/api", api_router)
        .merge(legacy_router)
        .fallback_service(static_files_service)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4500").await.unwrap();
    println!(" Rust server running on http://0.0.0.0:4500");
    
    r0out::connect_database().await;

    axum::serve(listener, app).await.unwrap();
}
