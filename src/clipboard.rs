use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::history::HistoryManager;

pub async fn start_monitoring(history: Arc<Mutex<HistoryManager>>) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Clipboard monitoring started (simplified implementation)");

    // For demonstration purposes, we'll simulate clipboard content
    // In a real implementation, this would integrate with system clipboard APIs
    let mut last_content = String::new();
    let mut counter = 0;

    loop {
        sleep(Duration::from_secs(5)).await;

        // Simulate adding clipboard content every few seconds
        counter += 1;
        let test_content = format!("Demo clipboard item #{} - {}", counter, chrono::Utc::now().format("%H:%M:%S"));

        if test_content != last_content {
            let mut history = history.lock().await;
            history.add_item(test_content.clone()).await;
            last_content = test_content;
            log::info!("Added new clipboard content to history");
        }
    }
}
