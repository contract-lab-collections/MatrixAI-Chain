//! Benchmarking setup for pallet-hashrate-market
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as HashrateMarket;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn do_something() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		do_something(RawOrigin::Signed(caller));
	}

	impl_benchmark_test_suite!(HashrateMarket, crate::mock::new_test_ext(), crate::mock::Test);
}
