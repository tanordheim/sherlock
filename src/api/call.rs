use serde::{Deserialize, Serialize};
use crate::utils::errors::SherlockError;

#[derive(Debug, Deserialize, Serialize)]
pub enum ApiCall {
    Show,
    Clear,
    InputOnly,
    Obfuscate(bool),
    SherlockError(SherlockError),
    DisplayPipe(String),
}
