use axum::{
    extract::{Path, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // ルーティング設定
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user));

    // サーバー起動
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ----------------------
// ハンドラ関数
// ----------------------

// GET /hello
async fn hello() -> &'static str {
    "Hello, World!"
}

// GET /users/:id
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

// POST /users
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    age: u32,
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<UserResponse> {
    let user = UserResponse {
        id: 1, // 本来はDBで生成
        name: payload.name,
        age: payload.age,
    };
    Json(user)
}
