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

//! Autogenerated weights for `pallet_xcm`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-rococo-dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/cumulus/.git/.artifacts/bench.json
// --pallet=pallet_xcm
// --chain=bridge-hub-rococo-dev
// --header=./file_header.txt
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn send() -> Weight {
		// Minimum execution time: 27_797 nanoseconds.
		Weight::from_ref_time(28_356_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	fn teleport_assets() -> Weight {
		// Minimum execution time: 32_656 nanoseconds.
		Weight::from_ref_time(33_011_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn reserve_transfer_assets() -> Weight {
		// Minimum execution time: 18_446_744_073_709_551 nanoseconds.
		Weight::from_ref_time(18_446_744_073_709_551_000)
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn execute() -> Weight {
		// Minimum execution time: 18_446_744_073_709_551 nanoseconds.
		Weight::from_ref_time(18_446_744_073_709_551_000)
	}
	// Storage: PolkadotXcm SupportedVersion (r:0 w:1)
	fn force_xcm_version() -> Weight {
		// Minimum execution time: 15_101 nanoseconds.
		Weight::from_ref_time(15_535_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PolkadotXcm SafeXcmVersion (r:0 w:1)
	fn force_default_xcm_version() -> Weight {
		// Minimum execution time: 4_700 nanoseconds.
		Weight::from_ref_time(4_896_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PolkadotXcm VersionNotifiers (r:1 w:1)
	// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn force_subscribe_version_notify() -> Weight {
		// Minimum execution time: 31_532 nanoseconds.
		Weight::from_ref_time(32_180_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: PolkadotXcm VersionNotifiers (r:1 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Storage: PolkadotXcm Queries (r:0 w:1)
	fn force_unsubscribe_version_notify() -> Weight {
		// Minimum execution time: 32_515 nanoseconds.
		Weight::from_ref_time(33_097_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: PolkadotXcm SupportedVersion (r:4 w:2)
	fn migrate_supported_version() -> Weight {
		// Minimum execution time: 13_811 nanoseconds.
		Weight::from_ref_time(14_149_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PolkadotXcm VersionNotifiers (r:4 w:2)
	fn migrate_version_notifiers() -> Weight {
		// Minimum execution time: 13_708 nanoseconds.
		Weight::from_ref_time(14_067_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:5 w:0)
	fn already_notified_target() -> Weight {
		// Minimum execution time: 15_277 nanoseconds.
		Weight::from_ref_time(15_626_000)
			.saturating_add(T::DbWeight::get().reads(5))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:2 w:1)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn notify_current_targets() -> Weight {
		// Minimum execution time: 29_263 nanoseconds.
		Weight::from_ref_time(29_831_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:3 w:0)
	fn notify_target_migration_fail() -> Weight {
		// Minimum execution time: 7_787 nanoseconds.
		Weight::from_ref_time(7_858_000)
			.saturating_add(T::DbWeight::get().reads(3))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:4 w:2)
	fn migrate_version_notify_targets() -> Weight {
		// Minimum execution time: 15_252 nanoseconds.
		Weight::from_ref_time(15_558_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:4 w:2)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn migrate_and_notify_old_targets() -> Weight {
		// Minimum execution time: 35_105 nanoseconds.
		Weight::from_ref_time(35_719_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
