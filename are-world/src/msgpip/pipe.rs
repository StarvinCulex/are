use std::collections::HashMap;
use std::sync::Mutex;

pub struct MPipe<K, V> {
    data: Mutex<HashMap<u64, HashMap<K, Vec<V>>>>,
    turn_now: u64,
}

pub struct Output<K, V> {
    data: HashMap<u64, HashMap<K, Vec<V>>>,
}

impl<K, V> MPipe<K, V> {
    pub fn new() -> Self {
        Self {
            data: Mutex::default(),
            turn_now: u64::default(),
        }
    }

    /// 存储事件  
    /// 事件按[`next_turn`]和[`key`]维度进行汇聚
    pub fn push(&self, next_turn: u64, key: K, value: V) {
        todo!()
    }

    /// 返回一个查询集，将所有注册时[`next_turn`]是0的事件移动到查询集中。  
    /// 将剩下注册的所有事件的[`next_turn`]减一。
    pub fn pop_this_turn(&mut self) -> Output<K, V> {
        todo!()
    }
}

impl<K, V> Output<K, V> {
    pub fn append(&mut self, key: K, values: Vec<V>) {
        todo!()
    }
}

impl<K, V> std::iter::Iterator for Output<K, V> {
    type Item = (K, Vec<V>);
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
