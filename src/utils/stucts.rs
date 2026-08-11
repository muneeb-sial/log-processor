use crate::utils::enums::LogLevel;

pub struct Log {
    pub timestamp: String,
    pub level: LogLevel,
    pub user_id: String,
    pub message: String,
    pub duration_ms: u64,
}
