// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-238-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=bridge-hub-rococo-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_balances
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/pallet_balances.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		// Minimum execution time: 45_363 nanoseconds.
		Weight::from_ref_time(46_431_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 35_046 nanoseconds.
		Weight::from_ref_time(36_176_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_creating() -> Weight {
		// Minimum execution time: 24_477 nanoseconds.
		Weight::from_ref_time(25_202_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_killing() -> Weight {
		// Minimum execution time: 26_979 nanoseconds.
		Weight::from_ref_time(27_647_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:2 w:2)
	fn force_transfer() -> Weight {
		// Minimum execution time: 47_275 nanoseconds.
		Weight::from_ref_time(48_268_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_all() -> Weight {
		// Minimum execution time: 42_149 nanoseconds.
		Weight::from_ref_time(42_735_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn force_unreserve() -> Weight {
		// Minimum execution time: 22_495 nanoseconds.
		Weight::from_ref_time(23_245_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
