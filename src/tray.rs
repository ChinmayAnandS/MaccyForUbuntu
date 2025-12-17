use std::sync::Arc;
use tokio::sync::Mutex;

use gtk4::{glib, Application};

use crate::config::Config;
use crate::history::HistoryManager;
use crate::ui::ClipboardWindow;

pub struct TrayIcon {
    app: Application,
    history: Arc<Mutex<HistoryManager>>,
    config: Arc<Mutex<Config>>,
}

impl TrayIcon {
    pub fn new(app: Application, history: Arc<Mutex<HistoryManager>>, config: Arc<Mutex<Config>>) -> Self {
        Self {
            app,
            history,
            config,
        }
    }

    pub fn setup(&self) {
        // For now, we'll use a simple approach - the app will run without a visible tray icon
        // In a production version, you'd integrate with libappindicator or similar
        // The clipboard popup can be triggered by other means (global hotkey, etc.)
        log::info!("Tray icon setup complete (placeholder)");
    }

    async fn show_clipboard_popup(history: Arc<Mutex<HistoryManager>>, config: Arc<Mutex<Config>>) {
        let history = history.lock().await;
        let items = history.get_items().clone();
        drop(history);

        let window = ClipboardWindow::new(items, config);
        window.show();
    }

    fn show_context_menu(_history: Arc<Mutex<HistoryManager>>, _config: Arc<Mutex<Config>>, _button: u32, _time: u32) {
        // TODO: Implement context menu for system tray
        // For now, this is a placeholder since GTK4 doesn't have easy system tray support
    }

    async fn show_preferences_window(config: Arc<Mutex<Config>>) {
        // TODO: Implement preferences window
        // For now, just print current config
        let config = config.lock().await;
        println!("Current config: {:?}", *config);
    }
}
