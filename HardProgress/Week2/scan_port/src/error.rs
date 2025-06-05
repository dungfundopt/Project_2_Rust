use thiserror::Error;

#[derive(Error, Debug, Clone)]  //định nghĩa lỗi
pub enum Error {
    #[error("Failed to execute!")]
    CliUsage,
    #[error("Network Error: {0}")]
    Reqwest(String), 
}

impl std::convert::From<reqwest::Error> for Error { //chuyển đổi từ reqwest::Error sang Error
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}