use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry, Window};

use crate::config::Config;
use crate::history::ClipboardItem;

pub struct ClipboardWindow {
    window: Window,
    items: VecDeque<ClipboardItem>,
    config: Arc<Mutex<Config>>,
}

impl ClipboardWindow {
    pub fn new(items: VecDeque<ClipboardItem>, config: Arc<Mutex<Config>>) -> Self {
        let window = Window::new();
        window.set_title(Some("ClipStack"));
        window.set_default_size(500, 400);
        window.set_modal(true);
        window.set_hide_on_close(true);

        Self {
            window,
            items,
            config,
        }
    }

    pub fn show(self) {
        let vbox = Box::new(Orientation::Vertical, 5);
        vbox.set_margin_top(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);

        // Search entry
        let search_entry = SearchEntry::new();
        search_entry.set_placeholder_text(Some("Search clipboard history..."));
        vbox.append(&search_entry);

        // Scrolled window for list
        let scrolled_window = ScrolledWindow::new();
        scrolled_window.set_vexpand(true);

        // List box for clipboard items
        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        scrolled_window.set_child(Some(&list_box));

        vbox.append(&scrolled_window);

        // Close button
        let close_button = Button::with_label("Close");
        let window_clone = self.window.clone();
        close_button.connect_clicked(move |_| {
            window_clone.close();
        });
        vbox.append(&close_button);

        // Populate initial list
        self.populate_list(&list_box, "");

        // Connect search entry
        let list_box_clone = list_box.clone();
        let items_clone = self.items.clone();
        search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            Self::update_list(&list_box_clone, &items_clone, &query);
        });

        // Connect list box activation (double-click or Enter)
        let items_clone = self.items.clone();
        let window_clone = self.window.clone();
        list_box.connect_row_activated(move |_, row| {
            let index = row.index();
            if index >= 0 {
                if let Some(item) = items_clone.get(index as usize) {
                    Self::paste_item(&item.content);
                    window_clone.close();
                }
            }
        });

        self.window.set_child(Some(&vbox));
        self.window.present();
    }

    fn populate_list(&self, list_box: &ListBox, query: &str) {
        Self::update_list(list_box, &self.items, query);
    }

    fn update_list(list_box: &ListBox, items: &VecDeque<ClipboardItem>, query: &str) {
        // Clear existing rows
        while let Some(child) = list_box.first_child() {
            list_box.remove(&child);
        }

        // Filter and add items
        let filtered_items: Vec<&ClipboardItem> = if query.is_empty() {
            items.iter().collect()
        } else {
            items.iter()
                .filter(|item| item.content.to_lowercase().contains(&query.to_lowercase()))
                .collect()
        };

        for item in filtered_items {
            let row = ListBoxRow::new();
            let label = Label::new(None);

            // Truncate long content and format
            let content = if item.content.len() > 100 {
                format!("{}...", &item.content[..100])
            } else {
                item.content.clone()
            };

            // Replace newlines with spaces for display
            let display_text = content.replace('\n', " ").replace('\r', "");
            label.set_text(&display_text);
            label.set_halign(gtk4::Align::Start);
            label.set_margin_start(5);
            label.set_margin_end(5);
            label.set_margin_top(2);
            label.set_margin_bottom(2);

            row.set_child(Some(&label));
            list_box.append(&row);
        }

        // Select first item if available
        if list_box.row_at_index(0).is_some() {
            list_box.select_row(list_box.row_at_index(0).as_ref());
        }
    }

    fn paste_item(content: &str) {
        // Copy to clipboard
        if let Some(display) = gdk4::Display::default() {
            let clipboard = display.clipboard();
            clipboard.set_text(content);
        }

        log::info!("Copied item to clipboard: {}", content);
    }
}
