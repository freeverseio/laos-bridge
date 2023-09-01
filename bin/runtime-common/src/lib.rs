#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

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
