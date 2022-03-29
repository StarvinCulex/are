use std::collections::HashMap;
use std::sync::Mutex;

pub struct MPipe<K: std::cmp::Eq + std::hash::Hash, V> {
    data: Mutex<HashMap<u64, HashMap<K, Vec<V>>>>,
    turn_now: u64,
}

pub struct Output<K, V> {
    data: HashMap<K, Vec<V>>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> MPipe<K, V> {
    pub fn new() -> Self {
        Self {
            data: Mutex::default(),
            turn_now: u64::default(),
        }
    }

    /// 存储事件  
    /// 事件按[`next_turn`]和[`key`]维度进行汇聚
    pub fn push(&self, next_turn: u64, key: K, value: V) {
        self.data
            .lock()
            .unwrap()
            .entry(next_turn)
            .or_default()
            .entry(key)
            .or_insert_with(|| { Vec::with_capacity(1) })
            .push(value);
    }

    /// 返回一个查询集，将所有注册时[`next_turn`]是0的事件移动到查询集中。  
    /// 将剩下注册的所有事件的[`next_turn`]减一。
    pub fn pop_this_turn(&mut self) -> Output<K, V> {
        let last_turn = self.turn_now;
        self.turn_now += 1;
        Output {
            data: self.data.get_mut().unwrap().remove(&last_turn).unwrap_or_else(|| {
                HashMap::default()
            }),
        }
    }
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Output<K, V> {
    pub fn append(&mut self, key: K, mut values: Vec<V>) {
        self.data
            .entry(key)
            .and_modify(|v| { v.append(&mut values) })
            .or_insert(values);
    }
}

impl<K: std::cmp::Eq + std::hash::Hash, V> std::iter::IntoIterator for Output<K, V> {
    type Item = <HashMap<K, Vec<V>> as std::iter::IntoIterator>::Item;
    type IntoIter = <HashMap<K, Vec<V>> as std::iter::IntoIterator>::IntoIter;
    
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
