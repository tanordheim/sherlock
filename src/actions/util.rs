use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::loader::util::SherlockError;

pub fn copy_to_clipboard(string: &str) -> Result<(), SherlockError>{
    let mut ctx = ClipboardContext::new().map_err(|e| SherlockError {
        name: "ClipboardError".to_string(),
        message: "Failed to get system clipboard".to_string(),
        traceback: e.to_string(),
    })?;
    let _ = ctx.set_contents(string.to_string());
    Ok(())
}
pub fn read_from_clipboard() -> Result<String, SherlockError>{
    let mut ctx = ClipboardContext::new().map_err(|e| SherlockError {
        name: "ClipboardError".to_string(),
        message: "Failed to get system clipboard".to_string(),
        traceback: e.to_string(),
    })?;
    Ok(ctx.get_contents().unwrap_or("".to_string()))
}

