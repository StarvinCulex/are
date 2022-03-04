#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Message {
    Light,
    // 让某块在下个fire_tick时变成点燃状态
    Ignite,
}
