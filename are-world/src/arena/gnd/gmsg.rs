use crate::arena;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Msg {
    // 让某块在下个fire_tick时变成点燃状态
    Ignite,
}
