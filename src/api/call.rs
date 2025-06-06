use crate::utils::errors::SherlockError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ApiCall {
    Show,
    Clear,
    InputOnly,
    Obfuscate(bool),
    SherlockError(SherlockError),
    DisplayPipe(String),
}
