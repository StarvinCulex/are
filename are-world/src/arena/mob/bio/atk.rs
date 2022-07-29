use serde::{Deserialize, Serialize};

use crate::meta::types::HitPointT;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ATK {
    Normal(HitPointT),
}
