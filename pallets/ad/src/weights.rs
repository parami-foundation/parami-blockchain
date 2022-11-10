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

//! Autogenerated weights for parami_ad
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/parami
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=parami_ad
// --extrinsic=*
// --steps=50
// --repeat=20
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/ad/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for parami_ad.
pub trait WeightInfo {
    fn create() -> Weight;
    fn update_reward_rate() -> Weight;
    fn update_tags() -> Weight;
    fn bid_with_fraction() -> Weight;
    fn add_budget() -> Weight;
    fn pay() -> Weight;
}

/// Weights for parami_ad using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Tag Metadata (r:1 w:0)
    // Storage: Ad AdsOf (r:1 w:1)
    // Storage: Ad EndtimeOf (r:0 w:1)
    // Storage: Ad Metadata (r:0 w:1)
    // Storage: Tag TagsOf (r:0 w:1)
    fn create() -> Weight {
        (24_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:1)
    fn update_reward_rate() -> Weight {
        (21_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:0)
    // Storage: Tag Metadata (r:1 w:0)
    // Storage: Tag TagsOf (r:0 w:1)
    fn update_tags() -> Weight {
        (27_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:1)
    // Storage: Nft Metadata (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Ad SlotOf (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Ad DeadlineOf (r:0 w:1)
    fn bid_with_fraction() -> Weight {
        (60_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad SlotOf (r:1 w:0)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Assets Asset (r:1 w:1)
    fn add_budget() -> Weight {
        (45_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:0)
    // Storage: Nft Metadata (r:1 w:0)
    // Storage: Ad DeadlineOf (r:1 w:1)
    // Storage: Ad Payout (r:1 w:1)
    // Storage: Ad SlotOf (r:1 w:1)
    // Storage: Tag TagsOf (r:2 w:0)
    // Storage: Tag PersonasOf (r:2 w:1)
    // Storage: Assets Account (r:3 w:3)
    // Storage: Did Metadata (r:2 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn pay() -> Weight {
        (123_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(20 as Weight))
            .saturating_add(T::DbWeight::get().writes(10 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Tag Metadata (r:1 w:0)
    // Storage: Ad AdsOf (r:1 w:1)
    // Storage: Ad EndtimeOf (r:0 w:1)
    // Storage: Ad Metadata (r:0 w:1)
    // Storage: Tag TagsOf (r:0 w:1)
    fn create() -> Weight {
        (24_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:1)
    fn update_reward_rate() -> Weight {
        (21_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:0)
    // Storage: Tag Metadata (r:1 w:0)
    // Storage: Tag TagsOf (r:0 w:1)
    fn update_tags() -> Weight {
        (27_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:1)
    // Storage: Nft Metadata (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Ad SlotOf (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Ad DeadlineOf (r:0 w:1)
    fn bid_with_fraction() -> Weight {
        (60_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(10 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad SlotOf (r:1 w:0)
    // Storage: Assets Account (r:2 w:2)
    // Storage: Assets Asset (r:1 w:1)
    fn add_budget() -> Weight {
        (45_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(7 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    // Storage: Did DidOf (r:1 w:0)
    // Storage: Balances Reserves (r:1 w:0)
    // Storage: Ad EndtimeOf (r:1 w:0)
    // Storage: Ad Metadata (r:1 w:0)
    // Storage: Nft Metadata (r:1 w:0)
    // Storage: Ad DeadlineOf (r:1 w:1)
    // Storage: Ad Payout (r:1 w:1)
    // Storage: Ad SlotOf (r:1 w:1)
    // Storage: Tag TagsOf (r:2 w:0)
    // Storage: Tag PersonasOf (r:2 w:1)
    // Storage: Assets Account (r:3 w:3)
    // Storage: Did Metadata (r:2 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn pay() -> Weight {
        (123_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(20 as Weight))
            .saturating_add(RocksDbWeight::get().writes(10 as Weight))
    }

    fn drawback_slot() -> Weight {
        (123_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(20 as Weight))
            .saturating_add(RocksDbWeight::get().writes(10 as Weight))
    }
}
