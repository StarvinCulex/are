#[derive(super::Deserialize, Debug)]
pub struct Conf {
    /// 每个格子每一刻触发植物生长的几率  
    /// > 同时也是[`Corpse`]触发腐烂的几率，因为[`Corpse`]是植物的一种
    pub aging_possibility: f64,
    pub aging_energy_weight: super::EnergyT,
}
