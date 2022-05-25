use serde::Deserialize;

use crate::arena::defs::Crd;
use crate::meta::types::*;
use crate::Coord;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub game: GameConf,
    pub runtime: RuntimeConf,
    pub plant: plant::Conf,
    pub bio: bio::Conf,
}

#[derive(Deserialize, Debug)]
pub struct GameConf {
    /// 区块数量。地图大小等于区块数量乘以区块大小。  
    /// 必须可以被`padding * (2, 2) + (1, 1)`整除(?)
    pub chunk_count: Crd,
    /// 区块的大小。必须大于(0, 0)
    pub chunk_size: Crd,
    /// 不懂啥意思就写(1, 1)
    pub padding: Crd,
}

#[derive(Deserialize, Debug)]
pub struct RuntimeConf {
    /// 两个游戏刻之前至少需要等待的时间间隔。单位是毫秒
    pub period: u64,
    /// 游戏内核线程数量。至少是1
    pub thread_count: usize,
    ///
    pub retry: usize,
}

pub mod plant {
    #[derive(super::Deserialize, Debug)]
    pub struct Conf {
        /// 每个格子每一刻触发植物生长的几率  
        /// > 同时也是[`Corpse`]触发腐烂的几率，因为[`Corpse`]是植物的一种
        pub aging_possibility: f64,
        /// 每一刻每个没有植物的格子长出植物的几率
        pub sow_possibility: f64,
        pub corpse: Corpse,
        /// [`Grass`]的配置
        pub grass: Plant,
        /// [`Tree`]的配置
        pub tree: Plant,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Plant {
        /// 触发生长时生长的能量数值
        pub grow: super::EnergyT,
        /// 没有植物的格子长出这个植物的权重
        pub sow_weight: u32,
        /// 植物繁殖消耗的能量
        pub fruit_cost: super::EnergyT,
        /// 植物的能量大于等于这个数值会繁殖
        pub fruit_when: super::EnergyT,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Corpse {
        /// 触发腐烂时损失的能量数值
        pub rot: super::EnergyT,
        /// 其他能量转化为Corpse会按此比率折算
        pub convert_rate: f64,
    }
}

pub mod bio {
    #[derive(super::Deserialize, Debug)]
    pub struct Conf {
        /// [`GodOfBio`]在某一刻触发造生物的概率
        pub create_possibility: f64,
        pub mutation: Mutation,
        /// 每个生物的基础数值。  
        /// 其基因提供的数值会累加到基础数值上面，作为游戏中的属性
        pub init: Properties,
        /// [`GodOfBio`]随机制造生物的模板。
        pub creatures: Vec<Creature>,
        /// 生物基因基本单元的词典
        pub acids: std::collections::HashMap<String, Acid>,
    }

    #[derive(super::Deserialize, Debug)]
    pub struct Acid {
        /// 突变成这种基本单元的权重
        pub mutate_rate: usize,
        /// 这个基本单元提供的属性
        pub prop: Properties,
    }

    #[derive(super::Deserialize, Debug, Clone)]
    pub struct Properties {
        /// 生物醒来的周期。最小是0  
        /// 每当醒来，年龄增加1
        pub wake_period: f64,
        /// 未实装
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
        pub watch_range: f64,
        /// 生物醒来移动的周期。至少是1（醒来就移动）
        pub move_period: f64,
        /// 移动的能量消耗。至少是0
        pub move_cost: f64,
        /// 植物能量高于此数值时才会吃它。至少是0
        pub eat_threshold: f64,
        /// 每次吃植物活的的能量。至少是0
        pub eat_takes: f64,
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

    impl std::ops::Add<&Properties> for Properties {
        type Output = Properties;
        #[inline]
        fn add(self, rhs: &Self) -> Self::Output {
            Properties {
                wake_period: self.wake_period + rhs.wake_period,
                wake_energy_consume: self.wake_energy_consume + rhs.wake_energy_consume,
                energy_cost: self.energy_cost + rhs.energy_cost,
                spawn_loss: self.spawn_loss + rhs.spawn_loss,
                spawn_init_energy: self.spawn_init_energy + rhs.spawn_init_energy,
                incubation_delay: self.incubation_delay + rhs.incubation_delay,
                size: self.size + rhs.size,
                watch_period: self.watch_period + rhs.watch_period,
                watch_range: self.watch_range + rhs.watch_range,
                move_period: self.move_period + rhs.move_period,
                move_cost: self.move_cost + rhs.move_cost,
                eat_threshold: self.eat_threshold + rhs.eat_threshold,
                eat_takes: self.eat_takes + rhs.eat_takes,
            }
        }
    }
}
