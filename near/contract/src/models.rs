use std::io::{Read, Write};

use base64::{encode_config, STANDARD};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

use crate::utils::AccountId;

const DEFAULT_BIT_STRING_SIZE_KB: usize = 4;
const DEFAULT_BIT_STRING_SIZE: usize = DEFAULT_BIT_STRING_SIZE_KB * 1024 * 8;

#[derive(Debug)]
pub struct RLError {
    pub message: String,
}

/// Implement the `std::error::Error` trait for `RLError`.
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
    // the encoded list as a byte vec (compressed)
    pub bit_set: Vec<u8>,
    pub creator: AccountId,
}

/// implement the `std::fmt::Display` trait for `RL2020`.
impl ToString for RL2020 {
    fn to_string(&self) -> String {
        encode_config(&self.bit_set, STANDARD)
    }
}

/// Reference implementation here
/// https://github.com/noandrea/rl2020.rs/blob/ab747623429438334484df308884bd9da4c06e93/src/lib.rs#L105
impl RL2020 {
    /// constructor
    pub fn new() -> Result<Self, RLError> {
        // initialize the bitset
        Ok(RL2020 {
            bit_set: Self::pack(vec![0; DEFAULT_BIT_STRING_SIZE_KB * 1024].as_ref())?,
            creator: env::predecessor_account_id().to_string(),
        })
    }

    /// capacity returns the capacity of the bitset in number of elements
    pub fn capacity(&self) -> usize {
        DEFAULT_BIT_STRING_SIZE
    }

    /// size returns the size of the bitset in kilobytes
    pub fn size(&self) -> usize {
        DEFAULT_BIT_STRING_SIZE_KB
    }

    /// sets the bit at the given index to the given value
    /// if the index is out of bounds, returns an error
    /// if do_set is true, sets the bit to 1
    /// if do_set is false, sets the bit to 0
    fn set(bit_set: &mut Vec<u8>, do_set: bool, index: u64) -> Result<(), RLError> {
        // check bounds
        Self::check_bounds(bit_set.len(), index)?;
        // calculate the position of the bit
        let pos = (index / 8) as usize;
        let j = (index % 8) as u8;
        // set the bit
        match do_set {
            true => bit_set[pos] |= 1 << j,
            false => bit_set[pos] &= !(1 << j),
        };
        Ok(())
    }

    /// sets the bits at the given indexes to the given values
    pub fn set_many(&mut self, to_set: Vec<u64>, to_unset: Vec<u64>) -> Result<(), RLError> {
        let mut bit_set = Self::unpack(&self.bit_set)?;
        for i in to_set {
            Self::set(&mut bit_set, true, i)?;
        }
        for i in to_unset {
            Self::set(&mut bit_set, false, i)?;
        }
        self.bit_set = Self::pack(&bit_set)?;
        Ok(())
    }

    /// replaces the bitset with the given one
    pub fn replace(&mut self, new_bit_set: Vec<u8>) -> Result<(), RLError> {
        if new_bit_set.len() != DEFAULT_BIT_STRING_SIZE_KB * 1024 {
            return Err(RLError::new("invalid bitset size"));
        }
        self.bit_set = Self::pack(&new_bit_set)?;
        Ok(())
    }

    /// returns the value of the bit at the given index
    /// if the index is out of bounds, returns an error
    /// if the bit is 0, returns false
    /// if the bit is 1, returns true
    pub fn get(&self, index: u64) -> Result<bool, RLError> {
        Self::check_bounds(self.capacity(), index)?;

        let pos = (index / 8) as usize;
        let j = (index % 8) as u8;

        let bit_set = Self::unpack(&self.bit_set)?;

        match bit_set[pos] & (1 << j) {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    /// pack encodes and compresses the bitset
    fn pack(data: &Vec<u8>) -> Result<Vec<u8>, RLError> {
        // compress the data
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(data)
            .map_err(|e| RLError::new(&e.to_string()))?;
        let compressed = e.finish().map_err(|e| RLError::new(&e.to_string()))?;
        // encode the data
        Ok(compressed)
    }

    /// unpack decodes and decompresses the bitset
    fn unpack(data: &Vec<u8>) -> Result<Vec<u8>, RLError> {
        let mut d = ZlibDecoder::new(&data[..]);
        let mut buf = Vec::new();
        d.read_to_end(&mut buf)
            .map_err(|e| RLError::new(&e.to_string()))?;
        Ok(buf)
    }

    /// check_bounds checks if the index is within the bounds of the bitset
    fn check_bounds(size: usize, index: u64) -> Result<(), RLError> {
        if (index as usize) >= size {
            return Err(RLError::new(&format!(
                "max indexable element is {}, provided index {} is out of range",
                size, index,
            )));
        }
        Ok(())
    }
}
