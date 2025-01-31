#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

use core::marker::PhantomData;
use ethereum_types::{H256, H64, U256};

pub trait Patch {
    fn epoch_length() -> u64;
}

pub struct EthereumPatch;
impl Patch for EthereumPatch {
    fn epoch_length() -> u64 { 30000u64 }
}

pub struct LightDAG<P: Patch> {
    pub epoch: usize,
    pub cache: Vec<u8>,
    #[allow(dead_code)]
    cache_size: usize,
    pub full_size: usize,
    _marker: PhantomData<P>,
}

impl<P: Patch> LightDAG<P> {
    pub fn new(number: u64) -> Self {
        let epoch = (number / P::epoch_length()) as usize;
        let cache_size = crate::get_cache_size(epoch);
        let full_size = crate::get_full_size(epoch);
        let seed = crate::get_seedhash(epoch);

        let mut cache = vec![0u8; cache_size];
        crate::make_cache(&mut cache, seed);

        Self {
            cache,
            cache_size,
            full_size,
            epoch,
            _marker: PhantomData,
        }
    }

    pub fn hashimoto(&self, hash: H256, nonce: H64) -> (H256, H256) {
        crate::hashimoto_light(hash, nonce, self.full_size, &self.cache)
    }

    pub fn is_valid_for(&self, number: U256) -> bool {
        (number / P::epoch_length()).as_usize() == self.epoch
    }

    pub fn from_cache(cache: Vec<u8>, number: U256) -> Self {
        let epoch = (number / P::epoch_length()).as_usize();
        let cache_size = crate::get_cache_size(epoch);
        let full_size = crate::get_full_size(epoch);

        Self {
            cache,
            cache_size,
            full_size,
            epoch,
            _marker: PhantomData,
        }
    }
}
