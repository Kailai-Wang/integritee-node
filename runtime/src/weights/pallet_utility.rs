//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-solo-fresh"), DB CACHE: 128

// Executed Command:
// ./integritee-node
// benchmark
// --chain=integritee-solo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_utility.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_utility.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32, ) -> Weight {
		(63_793_000 as Weight)
			// Standard Error: 36_000
			.saturating_add((9_370_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(4_912_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(58_530_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((10_017_000 as Weight).saturating_mul(c as Weight))
	}
	//TODO: update weight value
	fn dispatch_as() -> Weight {
		(14_340_000 as Weight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		(19_136_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_697_000 as Weight).saturating_mul(c as Weight))
	}
}
