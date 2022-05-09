
#[derive(Debug)]
pub struct App {
    pub config_file_path: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            config_file_path: None,
        }
    }
}
