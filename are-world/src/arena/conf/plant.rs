#[derive(super::Deserialize, Debug)]
pub struct Conf {
    /// 每个格子每一刻触发植物生长的几率  
    /// > 同时也是[`Corpse`]触发腐烂的几率，因为[`Corpse`]是植物的一种
    pub aging_possibility: f64,
    pub aging_energy_weight: super::EnergyT,
    pub corpse: Corpse,
}

#[derive(super::Deserialize, Debug)]
pub struct Corpse {
    /// 触发腐烂时损失的能量数值
    pub rot: super::EnergyT,
    /// 其他能量转化为Corpse会按此比率折算
    pub convert_rate: f64,
}
