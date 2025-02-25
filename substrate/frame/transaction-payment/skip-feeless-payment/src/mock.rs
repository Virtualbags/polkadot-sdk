// Copyright (C) Parity Technologies (UK) Ltd.
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

use super::*;
use crate as pallet_skip_feeless_payment;

use frame_support::{derive_impl, parameter_types};
use frame_system as system;
use sp_runtime::{
	impl_tx_ext_default,
	traits::{OriginOf, TransactionExtension},
};

type Block = frame_system::mocking::MockBlock<Runtime>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub static PreDispatchCount: u32 = 0;
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
pub struct DummyExtension;

impl TransactionExtensionBase for DummyExtension {
	const IDENTIFIER: &'static str = "DummyExtension";
	type Implicit = ();
}
impl<C> TransactionExtension<RuntimeCall, C> for DummyExtension {
	type Val = ();
	type Pre = ();
	impl_tx_ext_default!(RuntimeCall; C; validate);
	fn prepare(
		self,
		_val: Self::Val,
		_origin: &OriginOf<RuntimeCall>,
		_call: &RuntimeCall,
		_info: &DispatchInfoOf<RuntimeCall>,
		_len: usize,
		_context: &C,
	) -> Result<Self::Pre, TransactionValidityError> {
		PreDispatchCount::mutate(|c| *c += 1);
		Ok(())
	}
}

#[frame_support::pallet(dev_mode)]
pub mod pallet_dummy {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::feeless_if(|_origin: &OriginFor<T>, data: &u32| -> bool {
			*data == 0
		})]
		pub fn aux(_origin: OriginFor<T>, #[pallet::compact] _data: u32) -> DispatchResult {
			unreachable!()
		}
	}
}

impl pallet_dummy::Config for Runtime {}

frame_support::construct_runtime!(
	pub enum Runtime {
		System: system,
		SkipFeeless: pallet_skip_feeless_payment,
		DummyPallet: pallet_dummy,
	}
);
