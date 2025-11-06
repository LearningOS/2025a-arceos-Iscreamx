use core::hash::{BuildHasher, Hasher};

pub use alloc::collections::{
    BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque,
};

#[derive(Clone)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    pub fn new() -> Self {
        let random = crate::os::get_random();
        let k0 = random as u64;
        let k1 = (random >> 64) as u64;
        Self { k0, k1 }
    }
}

impl Default for RandomState {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher::new(self.k0, self.k1)
    }
}

pub struct DefaultHasher {
    state: u64,
}

impl DefaultHasher {
    fn new(k0: u64, k1: u64) -> Self {
        Self { state: k0 ^ k1 }
    }
}

impl Hasher for DefaultHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(5).wrapping_add(byte as u64);
        }
    }
}

pub type HashMap<K, V, S = RandomState> = hashbrown::HashMap<K, V, S>;

pub type HashSet<T, S = RandomState> = hashbrown::HashSet<T, S>;