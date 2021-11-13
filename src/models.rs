use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsTimeRange {
    pub max_time: Option<String>,
    pub min_time: Option<String>,
}

/// Wrapper for all response data.
///
/// This only exists because existing API useses this. Would be ideal to remove
/// it as it is fairly redundant.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Data<T> {
    pub data: T,
    pub cursor: Option<String>,
}
