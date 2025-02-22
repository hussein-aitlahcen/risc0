// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Serialization and deserialization tools for the RISC Zero zkVM
//!
//! Data needs to be serialized for transmission between the zkVM host and
//! guest. This module contains tools for this serialization and the
//! corresponding deserialization.
//!
//! On the host side, a serialization function such as [to_vec] should be used
//! when transmitting data to the guest, e.g.
//! ```ignore
//! prover.add_input_u32_slice(
//!     &to_vec(&a).expect("should be given serializeable type")
//! );
//! ```
//! Similarly, the deserialization function [from_slice] should be used when
//! reading data from the guest, e.g.
//! ```ignore
//! let c: u64 = from_slice(&receipt.journal)
//!     .expect("should deserialize if type matches what the guest committed");
//! ```
//!
//! On the guest side, the necessary (de)serialization functionality is
//! included in [`env`] module functions such as [`env::read`] and
//! [`env::commit`], so this crate rarely needs to be directly used in the
//! guest.
//!
//! [`env`]: ../guest/env/index.html
//! [`env::commit`]: ../guest/env/fn.commit.html
//! [`env::read`]: ../guest/env/fn.read.html

mod deserializer;
mod err;
mod serializer;

pub use deserializer::{from_slice, Deserializer};
pub use serializer::{to_slice, to_vec, to_vec_with_capacity, AllocVec, Serializer, Slice};

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::serde::{from_slice, to_vec};

    #[test]
    fn test_vec_round_trip() {
        let input: Vec<u64> = vec![1, 2, 3];
        let data = to_vec(&input).unwrap();
        let output: Vec<u64> = from_slice(data.as_slice()).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_map_round_trip() {
        let input: HashMap<&str, u32> = HashMap::from([("foo", 1), ("bar", 2), ("baz", 3)]);
        let data = to_vec(&input).unwrap();
        let output: HashMap<&str, u32> = from_slice(data.as_slice()).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_tuple_round_trip() {
        let input: (u32, u64) = (1, 2);
        let data = to_vec(&input).unwrap();
        let output: (u32, u64) = from_slice(data.as_slice()).unwrap();
        assert_eq!(input, output);
    }
}
