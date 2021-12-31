// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for parami_linker
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-20, STEPS: `2`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/parami
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=parami_linker
// --extrinsic=*
// --steps=2
// --repeat=50
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/linker/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for parami_linker.
pub trait WeightInfo {
    fn link_sociality(n: u32, ) -> Weight;
    fn link_crypto() -> Weight;
    fn deposit() -> Weight;
    fn force_trust() -> Weight;
    fn force_block() -> Weight;
    fn force_unlink() -> Weight;
    fn submit_link(n: u32, ) -> Weight;
    fn submit_score(n: u32, ) -> Weight;
}

/// Weights for parami_linker using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker LinksOf (r:1 w:0)
    // Storage: Linker Linked (r:1 w:0)
    // Storage: Linker PendingOf (r:1 w:1)
    fn link_sociality(n: u32, ) -> Weight {
        (14_051_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker LinksOf (r:1 w:1)
    // Storage: Linker Linked (r:1 w:1)
    // Storage: Linker PendingOf (r:0 w:1)
    fn link_crypto() -> Weight {
        (24_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker Registrar (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:1)
    fn deposit() -> Weight {
        (29_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Did Metadata (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Linker Registrar (r:0 w:1)
    fn force_trust() -> Weight {
        (16_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Did Metadata (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Linker Registrar (r:0 w:1)
    fn force_block() -> Weight {
        (45_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Linker LinksOf (r:1 w:1)
    // Storage: Linker Linked (r:0 w:1)
    fn force_unlink() -> Weight {
        (15_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker Registrar (r:1 w:0)
    // Storage: Linker LinksOf (r:1 w:1)
    // Storage: Linker Linked (r:1 w:1)
    // Storage: Linker PendingOf (r:0 w:1)
    fn submit_link(n: u32, ) -> Weight {
        (24_699_000 as Weight)
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker Registrar (r:1 w:0)
    // Storage: Tag PersonasOf (r:1 w:1)
    fn submit_score(n: u32, ) -> Weight {
        (11_000_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker LinksOf (r:1 w:0)
    // Storage: Linker Linked (r:1 w:0)
    // Storage: Linker PendingOf (r:1 w:1)
    fn link_sociality(n: u32, ) -> Weight {
        (14_051_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker LinksOf (r:1 w:1)
    // Storage: Linker Linked (r:1 w:1)
    // Storage: Linker PendingOf (r:0 w:1)
    fn link_crypto() -> Weight {
        (24_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker Registrar (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:1)
    fn deposit() -> Weight {
        (29_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Did Metadata (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Linker Registrar (r:0 w:1)
    fn force_trust() -> Weight {
        (16_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Did Metadata (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Linker Registrar (r:0 w:1)
    fn force_block() -> Weight {
        (45_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Linker LinksOf (r:1 w:1)
    // Storage: Linker Linked (r:0 w:1)
    fn force_unlink() -> Weight {
        (15_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker Registrar (r:1 w:0)
    // Storage: Linker LinksOf (r:1 w:1)
    // Storage: Linker Linked (r:1 w:1)
    // Storage: Linker PendingOf (r:0 w:1)
    fn submit_link(n: u32, ) -> Weight {
        (24_699_000 as Weight)
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Linker Registrar (r:1 w:0)
    // Storage: Tag PersonasOf (r:1 w:1)
    fn submit_score(n: u32, ) -> Weight {
        (11_000_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}