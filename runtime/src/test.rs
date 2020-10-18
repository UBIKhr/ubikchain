
// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Macro for creating the tests for the module.

#![cfg(test)]

#[derive(Debug)]
pub struct CallWithDispatchInfo;
impl sp_runtime::traits::Dispatchable for CallWithDispatchInfo {
	type Origin = ();
	type Trait = ();
	type Info = frame_support::weights::DispatchInfo;
	type PostInfo = frame_support::weights::PostDispatchInfo;

	fn dispatch(self, _origin: Self::Origin)
		-> sp_runtime::DispatchResultWithInfo<Self::PostInfo> {
			panic!("Do not use dummy implementation for dispatch.");
	}
}

#[macro_export]
macro_rules! decl_tests {
	($test:ty, $ext_builder:ty, $existential_deposit:expr) => {

		use crate::*;
		use sp_runtime::{FixedPointNumber, traits::{SignedExtension, BadOrigin}};
		use frame_support::{
			assert_noop, assert_ok, assert_err,
			traits::{
				LockableCurrency, LockIdentifier, WithdrawReason, WithdrawReasons,
				Currency, ReservableCurrency, ExistenceRequirement::AllowDeath, StoredMap
			}
		};
		use pallet_transaction_payment::{ChargeTransactionPayment, Multiplier};
		use frame_system::RawOrigin;

		const ID_1: LockIdentifier = *b"1       ";
		const ID_2: LockIdentifier = *b"2       ";

		pub type System = frame_system::Module<$test>;
		pub type Balances = Module<$test>;

		pub const CALL: &<$test as frame_system::Trait>::Call = &$crate::tests::CallWithDispatchInfo;

		/// create a transaction info struct from weight. Handy to avoid building the whole struct.
		pub fn info_from_weight(w: Weight) -> DispatchInfo {
			DispatchInfo { weight: w, ..Default::default() }
		}

		fn events() -> Vec<Event> {
			let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();

			System::reset_events();

			evt
		}

		fn last_event() -> Event {
			system::Module::<Test>::events().pop().expect("Event expected").event
		}

		#[test]
		fn basic_locking_should_work() {
			<$ext_builder>::default().existential_deposit(1).monied(true).build().execute_with(|| {
				assert_eq!(Balances::free_balance(1), 10);
				Balances::set_lock(ID_1, &1, 9, WithdrawReasons::all());
				assert_noop!(
					<Balances as Currency<_>>::transfer(&1, &2, 5, AllowDeath),
					Error::<$test, _>::LiquidityRestrictions
				);
			});
		}
	}
}