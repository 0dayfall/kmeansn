#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub columns: Vec<String>,
    pub has_id: bool,
}

impl Dataset {
    pub fn new() -> Self {
        Self::default()
    }
}
