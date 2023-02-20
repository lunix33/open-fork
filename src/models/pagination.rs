use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationOptions {
    pub page: u32,
    pub limit: u32,
}

impl Default for PaginationOptions {
    fn default() -> Self {
        Self { page: 0, limit: 10 }
    }
}

impl PaginationOptions {
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}
