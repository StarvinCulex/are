use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::arena::types::ThreatT;
use crate::arena::Weak;
use crate::mob::bio::atk::ATK;
use crate::MobBlock;

#[derive(Serialize, Deserialize)]
pub enum Order {
    MobMainTick,
    Attack {
        atk: ATK,
        attacker: Option<Weak<MobBlock>>,
        threat: ThreatT,
    },
}

impl Debug for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::MobMainTick => {
                write!(f, "MobMainTick")
            }
            Order::Attack {
                atk,
                attacker,
                threat,
            } => {
                write!(
                    f,
                    "Attack{{atk:{atk:?}, attacker:{attacker:?}, threat:{threat}}}",
                    atk = atk,
                    attacker = attacker.as_ref().map(|x| x.as_ptr()),
                    threat = threat
                )
            }
        }
    }
}
