use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(code_buffer: String) -> Result<(), String> {
    let mut ctx : ClipboardContext =
        ClipboardProvider::new()
            .map_err(|err| err.to_string())
            .unwrap();
    ctx
        .set_contents(code_buffer)
        .map_err(|err| err.to_string())
}