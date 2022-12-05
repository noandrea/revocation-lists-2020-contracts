use std::io::{Read, Write};

use base64::{decode_config, encode_config, STANDARD};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

use crate::utils::AccountId;

const DEFAULT_BITSTRING_SIZE_KN: usize = 16;

#[derive(Debug)]
pub struct RLError {
    pub message: String,
}

impl RLError {
    pub fn new(msg: &str) -> Self {
        RLError {
            message: String::from(msg),
        }
    }
}

/// this comes from https://github.com/noandrea/rl2020.rs
#[derive(Clone, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RL2020 {
    pub encoded_list: String,
    pub creator: AccountId,
}

/// Reference implementation here
/// https://github.com/noandrea/rl2020.rs/blob/ab747623429438334484df308884bd9da4c06e93/src/lib.rs#L105
///
///
impl RL2020 {
    pub fn new() -> Result<Self, RLError> {
        // if size < MIN_BITSTRING_SIZE_KN {
        //     return Err(RLError::new(&format!(
        //         "minimum credential size is {}, got {}",
        //         MIN_BITSTRING_SIZE_KN, size
        //     )));
        // }
        // if size > MAX_BITSTRING_SIZE_KB {
        //     return Err(RLError::new(&format!(
        //         "maximum credential size is {}, got {}",
        //         MIN_BITSTRING_SIZE_KN, size
        //     )));
        // }

        // initialize the bitset
        let bs = vec![0; DEFAULT_BITSTRING_SIZE_KN * 1024];
        let el = Self::pack(&bs)?;
        Ok(RL2020 {
            encoded_list: el,
            creator: env::predecessor_account_id().to_string(),
        })
    }

    fn pack(data: &Vec<u8>) -> Result<String, RLError> {
        // compress the data
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(data)
            .map_err(|e| RLError::new(&e.to_string()))?;
        let compressed = e.finish().map_err(|e| RLError::new(&e.to_string()))?;
        // encode the data
        Ok(encode_config(&compressed, STANDARD))
    }

    fn unpack(data: &String) -> Result<Vec<u8>, RLError> {
        let bin = decode_config(&data, STANDARD).map_err(|e| RLError::new(&e.to_string()))?;
        let mut d = ZlibDecoder::new(&*bin);
        let mut buf = Vec::new();
        d.read_to_end(&mut buf)
            .map_err(|e| RLError::new(&e.to_string()))?;
        Ok(buf)
    }

    fn check_bounds(&self, index: u64) -> Result<(), RLError> {
        match index {
            i if (i as usize) >= self.capacity() => Err(RLError::new(&format!(
                "max indexable element is {}, provided index {} is out of range",
                self.capacity(),
                i,
            ))),
            _ => Ok(()),
        }
    }

    pub fn capacity(&self) -> usize {
        DEFAULT_BITSTRING_SIZE_KN * 1024 * 8
    }

    // size returns the size of the bitset int kb
    pub fn size(&self) -> usize {
        return DEFAULT_BITSTRING_SIZE_KN;
    }

    pub fn set(&mut self, revoke: bool, index: u64) -> Result<(), RLError> {
        self.check_bounds(index)?;

        let pos = (index / 8) as usize;
        let j = (index % 8) as u8;

        let mut bit_set = Self::unpack(&self.encoded_list)?;
        match revoke {
            true => bit_set[pos] |= 1 << j,
            false => bit_set[pos] &= !(1 << j),
        };
        self.encoded_list = Self::pack(&bit_set)?;
        Ok(())
    }

    pub fn get(&self, index: u64) -> Result<bool, RLError> {
        self.check_bounds(index)?;

        let pos = (index / 8) as usize;
        let j = (index % 8) as u8;

        let bit_set = Self::unpack(&self.encoded_list)?;

        match bit_set[pos] & (1 << j) {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    pub fn revoke(&mut self, idx: u64) -> Result<(), RLError> {
        self.set(true, idx)
    }

    pub fn reset(&mut self, idx: u64) -> Result<(), RLError> {
        self.set(false, idx)
    }

    pub fn is_revoked(&self, idx: u64) -> Result<bool, RLError> {
        return self.get(idx);
    }
}
