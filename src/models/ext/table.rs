pub trait TableField:
    std::fmt::Display + std::fmt::Debug + std::marker::Send + std::marker::Sync
{
}

#[derive(Debug, Clone)]
pub struct TableName(pub &'static str);
impl std::fmt::Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
unsafe impl std::marker::Send for TableName {}
unsafe impl std::marker::Sync for TableName {}
