// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Common types/functions that may be used by runtimes of all bridged chains.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

// use crate::messages_call_ext::MessagesCallSubType;
// use pallet_bridge_grandpa::CallSubType as GrandpaCallSubType;
// use pallet_bridge_parachains::CallSubType as ParachainsCallSubtype;
// use sp_runtime::transaction_validity::TransactionValidity;
use xcm::v3::NetworkId;


/// A mapping over `NetworkId`.
/// Since `NetworkId` doesn't include `Evochain`, `Rococo` and `OwnershipParachain`, we create some
/// synthetic associations between these chains and `NetworkId` chains.
pub enum CustomNetworkId {
	/// The Evochain network ID, associated with Kusama.
	Evochain,
	/// The Rococo network ID, associated with Polkadot.
	Rococo,
	/// The OwnershipParachain network ID, associated with Westend.
	OwnershipParachain,
}

// impl TryFrom<bp_runtime::ChainId> for CustomNetworkId {
// 	type Error = ();

// 	fn try_from(chain: bp_runtime::ChainId) -> Result<Self, Self::Error> {
// 		// TODO: this code needs to be removed or fixed (use constants) in the
// 		// https://github.com/paritytech/parity-bridges-common/issues/2068
// 		if chain == *b"evol" {
// 			Ok(Self::Evochain)
// 		} else if chain == *b"roco" {
// 			Ok(Self::Rococo)
// 		} else if chain == *b"ownp" {
// 			Ok(Self::OwnershipParachain)
// 		} else {
// 			Err(())
// 		}
// 	}
// }

impl CustomNetworkId {
	/// Converts self to XCM' network id.
	pub const fn as_network_id(&self) -> NetworkId {
		match *self {
			CustomNetworkId::Evochain => NetworkId::Kusama,
			CustomNetworkId::Rococo => NetworkId::Polkadot,
			CustomNetworkId::OwnershipParachain => NetworkId::Westend,
		}
	}
}
