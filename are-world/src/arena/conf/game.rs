#[derive(super::Deserialize, Debug)]
pub struct Conf {
    /// 区块数量。地图大小等于区块数量乘以区块大小。  
    /// 必须可以被`padding * (2, 2) + (1, 1)`整除(?)
    pub chunk_count: super::Crd,
    /// 区块的大小。必须大于(0, 0)
    pub chunk_size: super::Crd,
    /// 不懂啥意思就写(1, 1)
    pub padding: super::Crd,
}
