#[derive(super::Deserialize, Debug)]
pub struct Conf {
    /// 两个游戏刻之前至少需要等待的时间间隔。单位是毫秒
    pub period: u64,
    /// 游戏内核线程数量。至少是1
    pub thread_count: usize,
    /// 游戏内部失败的重试次数
    pub retry: usize,
}
