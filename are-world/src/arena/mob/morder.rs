use crate::arena::types::ThreatT;
use crate::arena::Weak;
use crate::mob::bio::atk::ATK;
use crate::MobBlock;

pub enum Order {
    MobMainTick,
    Attack {
        atk: ATK,
        attacker: Option<Weak<MobBlock>>,
        threat: ThreatT,
    },
}
