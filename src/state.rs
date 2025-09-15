use crate::services::base;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct AppState {
    pub component: Mutex<Option<base::Component>>,
    pub directory: Mutex<Option<PathBuf>>,
    pub is_selected: Mutex<Option<bool>>,
    pub selected_file: Mutex<Option<PathBuf>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component: Mutex::new(None),
            directory: Mutex::new(None),
            is_selected: Mutex::new(None),
            selected_file: Mutex::new(None),
        }
    }
}
