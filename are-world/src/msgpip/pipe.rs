use std::collections::HashMap;

pub struct MPipe<K: Eq + std::hash::Hash, V> {
    data: HashMap<u64, HashMap<K, Vec<V>>>,
    turn_now: u64,
}

pub struct Output<K, V> {
    data: HashMap<K, Vec<V>>,
}

impl<K: Eq + std::hash::Hash, V> MPipe<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
            turn_now: u64::default(),
        }
    }

    /// 存储事件  
    /// 事件按[`next_turn`]和[`key`]维度进行汇聚
    pub fn push(&mut self, delay: u64, key: K, value: V) {
        self.data
            .entry(self.turn_now + delay)
            .or_default()
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(1))
            .push(value);
    }

    pub fn append(&mut self, list: Vec<(u64, K, V)>) {
        for (t, k, v) in list {
            self.push(t, k, v);
        }
    }

    /// 返回一个查询集，将所有注册时[`next_turn`]是0的事件移动到查询集中。  
    /// 将剩下注册的所有事件的[`next_turn`]减一。
    pub fn pop_this_turn(&mut self) -> Output<K, V> {
        let last_turn = self.turn_now;
        self.turn_now += 1;
        Output {
            data: self
                .data
                .remove(&last_turn)
                .unwrap_or_else(|| HashMap::default()),
        }
    }
}

impl<K: Eq + std::hash::Hash, V> Output<K, V> {
    pub fn append(&mut self, key: K, mut values: Vec<V>) {
        self.data
            .entry(key)
            .and_modify(|v| v.append(&mut values))
            .or_insert(values);
    }
}

impl<K: Eq + std::hash::Hash, V> IntoIterator for Output<K, V> {
    type Item = <HashMap<K, Vec<V>> as IntoIterator>::Item;
    type IntoIter = <HashMap<K, Vec<V>> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<K: Eq + std::hash::Hash, V> Default for MPipe<K, V> {
    fn default() -> Self {
        MPipe::new()
    }
}
