use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(string:&String){
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(string.to_owned()).unwrap();
}

