#[derive(super::Deserialize, Debug)]
pub struct Conf {
    /// 触发腐烂时损失的能量数值
    pub rot: super::EnergyT,
    /// 其他能量转化为Corpse会按此比率折算
    pub convert_rate: f64,
}
