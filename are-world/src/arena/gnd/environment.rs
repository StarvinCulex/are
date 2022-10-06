use serde::{Deserialize, Serialize};

use crate::meta::types::EnvT;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Environment {
    pub wet: EnvT,
    pub altitude: EnvT,
}
