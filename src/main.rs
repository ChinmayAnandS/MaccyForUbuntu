mod clipboard;
mod config;
mod history;
mod tray;
mod ui;

use std::sync::Arc;
use tokio::sync::Mutex;

use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button, Box as GtkBox, Orientation};

use crate::config::Config;
use crate::history::HistoryManager;
use crate::tray::TrayIcon;

#[tokio::main]
async fn main() -> glib::ExitCode {
    env_logger::init();

    let app = Application::builder()
        .application_id("com.clipstack.app")
        .build();

    app.connect_activate(move |app| {
        let config = Arc::new(Mutex::new(Config::load().unwrap_or_default()));
        let history_manager = Arc::new(Mutex::new(HistoryManager::new(config.clone())));

        // Create main window
        let window = ApplicationWindow::new(app);
        window.set_title(Some("ClipStack"));
        window.set_default_size(300, 200);

        let vbox = GtkBox::new(Orientation::Vertical, 5);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);

        // Status label
        let status_label = gtk4::Label::new(Some("ClipStack is running..."));
        vbox.append(&status_label);

        // Show clipboard button
        let show_button = Button::with_label("Show Clipboard History");
        let history_clone = history_manager.clone();
        let config_clone = config.clone();
        show_button.connect_clicked(move |_| {
            let history_clone = history_clone.clone();
            let config_clone = config_clone.clone();
            glib::spawn_future_local(async move {
                let history = history_clone.lock().await;
                let items = history.get_items().clone();
                drop(history);

                let window = crate::ui::ClipboardWindow::new(items, config_clone);
                window.show();
            });
        });
        vbox.append(&show_button);

        // Quit button
        let quit_button = Button::with_label("Quit");
        quit_button.connect_clicked(|_| {
            std::process::exit(0);
        });
        vbox.append(&quit_button);

        window.set_child(Some(&vbox));
        window.present();

        // Start clipboard monitoring
        let history_clone = history_manager.clone();
        glib::spawn_future_local(async move {
            if let Err(e) = clipboard::start_monitoring(history_clone).await {
                log::error!("Failed to start clipboard monitoring: {}", e);
            }
        });

        // Create tray icon (placeholder)
        let tray = TrayIcon::new(app.clone(), history_manager, config);
        tray.setup();
    });

    app.run()
}
