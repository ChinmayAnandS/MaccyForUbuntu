use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use clipboard::{ClipboardContext, ClipboardProvider};

use crate::history::HistoryManager;

pub async fn start_monitoring(history: Arc<Mutex<HistoryManager>>) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Clipboard monitoring started");

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let mut last_content = String::new();

    // Get initial clipboard content if any
    if let Ok(content) = ctx.get_contents() {
        if !content.trim().is_empty() {
            last_content = content.clone();
            let mut history = history.lock().await;
            history.add_item(content).await;
            log::info!("Added initial clipboard content to history");
        }
    }

    let mut iteration = 0;
    loop {
        iteration += 1;
        // Check clipboard every 500ms
        sleep(Duration::from_millis(500)).await;

        log::debug!("Clipboard check iteration: {}", iteration);

        match ctx.get_contents() {
            Ok(content) => {
                log::debug!("Got clipboard content: '{}' (len: {})", content, content.len());
                // Skip if content is empty or same as last
                if content.trim().is_empty() || content == last_content {
                    continue;
                }

                // Add new content to history
                let mut history = history.lock().await;
                history.add_item(content.clone()).await;
                last_content = content.clone();
                log::info!("Added new clipboard content to history: '{}'", content);
            }
            Err(e) => {
                log::warn!("Failed to get clipboard contents: {}", e);
                // Try to reinitialize clipboard context on error
                match ClipboardProvider::new() {
                    Ok(new_ctx) => {
                        ctx = new_ctx;
                        log::info!("Reinitialized clipboard context");
                    }
                    Err(init_err) => {
                        log::error!("Failed to reinitialize clipboard context: {}", init_err);
                    }
                }
            }
        }
    }
}
