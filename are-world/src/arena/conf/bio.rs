use crate::arena::types::HitPointT;
use crate::arena::mob::bio::gene::GeneCnt;

#[derive(super::Deserialize, Debug)]
pub struct Conf {
    /// [`GodOfBio`]在某一刻触发造生物的概率
    pub initial_population: u32,
    pub mutation: Mutation,
    /// 每个生物的基础数值。  
    /// 其基因提供的数值会累加到基础数值上面，作为游戏中的属性
    pub init: Properties,
    /// [`GodOfBio`]随机制造生物的模板。
    pub creatures: Vec<Creature>,
    /// 生物基因基本单元的词典
    pub acids: std::collections::BTreeMap<String, Acid>,
}

#[derive(super::Deserialize, Debug)]
pub struct Acid {
    /// 突变成这种基本单元的权重
    pub mutate_rate: usize,
    /// 这个基本单元提供的属性
    pub prop: Properties,
}

#[derive(super::Deserialize, Debug, Clone, derive_more::Add, derive_more::Mul)]
pub struct Properties {
    /// 生物醒来的周期。最小是0  
    /// 每当醒来，年龄增加1
    pub wake_period: f64,
    /// 醒来的能量开销  
    /// 能量低于0会饿死
    pub wake_energy_consume: f64,
    /// 这个生物价值的能量。最小是1  
    /// 生物繁殖需要消耗的能量是`energy_cost + spawn_loss + spawn_init_energy`
    pub energy_cost: f64,
    /// 生物繁殖时需要额外损耗的能量。最小是0
    pub spawn_loss: f64,
    /// 生物出生时带有的能量。最小是1
    pub spawn_init_energy: f64,
    /// 刚被创造的生物休眠的时间长度。最小是0
    pub incubation_delay: f64,
    /// 生物的大小。最小是(0, 0)，占有一格
    pub size: super::Coord<f64>,
    /// 醒来观察周围的周期。至少是1（醒来就移动）
    pub watch_period: f64,
    /// 生物观察周围的范围。至少是0（不观察）
    pub watch_area: f64,
    /// 生物醒来移动的周期。至少是1（醒来就移动）
    pub move_period: f64,
    /// 移动的能量消耗。至少是0
    pub move_cost: f64,
    /// 植物能量高于此数值时才会吃它。至少是0
    pub eat_threshold: f64,
    /// 每次吃植物活的的能量。至少是0
    pub eat_takes: f64,

    pub combat: Combat,
}

#[derive(super::Deserialize, Debug, Clone, derive_more::Add, derive_more::Mul)]
pub struct Combat {
    /// 生命值。为0时会死亡。至少是1
    pub hit_point: f64,
    /// 每次醒来会回复的生命值（向下取整）。至少是0
    pub regeneration: f64,
    /// 每次醒来回复生命值造成的能量开销（向下取整）。至少是0
    pub regeneration_cost: f64,
    /// 攻击力数值。至少是0
    /// 攻击时会将目标的生命值减去攻击力。
    pub atk: f64,
    /// 每次攻击的能量开销。至少是0
    pub atk_cost: f64,

    /// 威胁系数
    pub threat: f64,
    /// 其他生物的威胁系数不低于此数值就会逃跑
    pub flee_threshold: f64,
    /// 逃跑时如果目标生物的威胁系数不高于此数值，在攻击范围内就会反击
    pub fight_back_threshold: f64,
    /// 其他生物的威胁系数不高于此数值就会设定为捕猎目标
    pub chase_threshold: f64,
}

#[derive(super::Deserialize, Debug)]
pub struct Mutation {
    /// 插入突变发生的概率。和其他突变互斥
    pub insert: f64,
    /// 移除突变发生的概率。和其他突变互斥
    pub remove: f64,
}

#[derive(super::Deserialize, Debug)]
pub struct Creature {
    /// 生物被选择的权重
    pub weight: usize,
    /// 进行插入突变的次数
    pub insertions: usize,
    /// 初始的基因
    pub gene: crate::arena::mob::bio::gene::Gene,
}
