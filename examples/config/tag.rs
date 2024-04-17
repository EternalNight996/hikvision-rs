use serde::{Deserialize, Serialize};

/// 日志标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ETag {
    Tauri,
    HikMvs,
    HikNet,
    Unknow,
    GUI,
    Init,
    File,
}
impl Default for ETag {
    fn default() -> Self {
        Self::Unknow
    }
}
