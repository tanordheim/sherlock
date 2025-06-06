use crate::utils::errors::SherlockError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ApiCall {
    Show,
    Clear,
    InputOnly,
    Obfuscate(bool),
    SherlockError(SherlockError),
    ClearAwaiting,
    Pipe(String),
}
