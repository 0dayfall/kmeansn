#[derive(Debug, Clone)]
pub struct Dataset {
    pub columns: Vec<String>,
    pub has_id: bool,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            has_id: false,
        }
    }
}
