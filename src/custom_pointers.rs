// Custom pointer chains for data not exposed by libeldenring
//
// These pointers were reverse-engineered from Cheat Engine tables
// (eldenring_all-in-one_Hexinton-v5.0_ce7.5.ct)

use libeldenring::memedit::PointerChain;
use libeldenring::prelude::base_addresses::{BaseAddresses, Version};
use libeldenring::version::get_version;

/// Custom pointers for route tracking features
pub struct CustomPointers {
    /// Detects if player is currently riding Torrent (0 = on foot, != 0 = mounted)
    /// Path: WorldChrMan -> PlayerIns -> 0x190 -> 0xE8 -> 0x32
    is_riding: PointerChain<u8>,
}

impl CustomPointers {
    /// Create custom pointers using base addresses from libeldenring
    pub fn new(base_addresses: &BaseAddresses) -> Self {
        let version = get_version();

        // PlayerIns offset varies by game version
        let player_ins: usize = match version {
            Version::V1_02_0 | Version::V1_02_1 | Version::V1_02_2 | Version::V1_02_3
            | Version::V1_03_0 | Version::V1_03_1 | Version::V1_03_2 | Version::V1_04_0
            | Version::V1_04_1 | Version::V1_05_0 | Version::V1_06_0 => 0x18468,
            _ => 0x1E508, // V1_07_0 and later
        };

        Self {
            is_riding: PointerChain::new(&[
                base_addresses.world_chr_man,
                player_ins,
                0x190,
                0xE8,
                0x32,
            ]),
        }
    }

    /// Returns true if the player is currently riding Torrent
    pub fn is_on_torrent(&self) -> bool {
        self.is_riding.read().map(|v| v != 0).unwrap_or(false)
    }
}
