
//! Autogenerated weights for `estate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-03, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/metaverse-node
// benchmark
// --pallet=estate
// --extrinsic=*
// --steps=20
// --repeat=10
// --execution=wasm
// --wasm-execution=compiled
// --raw
// --output
// ./pallets/estate/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `estate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> estate::WeightInfo for WeightInfo<T> {
	// Storage: Estate MaxBounds (r:0 w:1)
	fn set_max_bounds() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
