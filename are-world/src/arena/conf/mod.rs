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
    pub gen: gen::Conf,
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

pub mod bio;
pub mod gen;
pub mod plant;
