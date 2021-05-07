pub mod requests;

pub enum CutepawError {
    ReqwestError(reqwest::Error),
    APIError,
}
