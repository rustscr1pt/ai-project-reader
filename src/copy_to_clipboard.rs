use std::sync::Arc;
use clipboard::ClipboardContext;
use tokio::sync::Mutex;

pub async fn copy_to_clipboard(code_buffer : Arc<Mutex<String>>) -> Result<(), String> {
    let code_buffer : Arc<Mutex<String>> = Arc::clone(&code_buffer);
    tokio::task::spawn_blocking(move || {
        let unlocked = code_buffer.lock().await;
        let mut ctx =
    })
}