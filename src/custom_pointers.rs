// Custom pointer chains for data not exposed by libeldenring
//
// These pointers were reverse-engineered from Cheat Engine tables
// (eldenring_all-in-one_Hexinton-v5.0_ce7.5.ct)

use libeldenring::memedit::PointerChain;
use libeldenring::prelude::base_addresses::{BaseAddresses, Version};
use libeldenring::version::get_version;
use serde::Serialize;

/// Debug info for Torrent/riding state - used to identify which values change
#[derive(Debug, Clone, Serialize, Default)]
pub struct TorrentDebugInfo {
    /// RideParam ID (4 bytes at +0x190 +0xE8 +0x20)
    pub ride_param_id: Option<i32>,
    /// IsRidingEnabled (byte at +0x190 +0xE8 +0x31)
    pub is_riding_enabled: Option<u8>,
    /// Riding (byte at +0x190 +0xE8 +0x32)
    pub riding: Option<u8>,
    /// IsItAHorse (byte at +0x190 +0xE8 +0x33)
    pub is_it_a_horse: Option<u8>,
    /// HorseState (4 bytes at +0x190 +0xE8 +0x10 +0x50)
    /// Values: 0=None, 1=AreYouRiding?, 3=IsThereRidingRequest?, 5=Success
    pub horse_state: Option<i32>,
    /// HorseHP (4 bytes at +0x190 +0xE8 +0x12C)
    pub horse_hp: Option<i32>,
    /// IsInsideNoRideArea (byte at +0x190 +0xE8 +0x164)
    pub is_inside_no_ride_area: Option<u8>,
}

/// Custom pointers for route tracking features
pub struct CustomPointers {
    // Ride module pointers (PlayerIns + 0x190 + 0xE8 + offset)
    ride_param_id: PointerChain<i32>,
    is_riding_enabled: PointerChain<u8>,
    riding: PointerChain<u8>,
    is_it_a_horse: PointerChain<u8>,
    horse_state: PointerChain<i32>,
    horse_hp: PointerChain<i32>,
    is_inside_no_ride_area: PointerChain<u8>,
    // Death counter (GameDataMan + 0x94)
    death_count: PointerChain<u32>,
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
            _ => 0x1E508, // V1_07_0 and later (including 2.x)
        };

        let world_chr_man = base_addresses.world_chr_man;

        Self {
            // +0x190 +0xE8 +0x20
            ride_param_id: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x20]),
            // +0x190 +0xE8 +0x31
            is_riding_enabled: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x31]),
            // +0x190 +0xE8 +0x32
            riding: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x32]),
            // +0x190 +0xE8 +0x33
            is_it_a_horse: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x33]),
            // +0x190 +0xE8 +0x10 +0x50
            horse_state: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x10, 0x50]),
            // +0x190 +0xE8 +0x12C
            horse_hp: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x12C]),
            // +0x190 +0xE8 +0x164
            is_inside_no_ride_area: PointerChain::new(&[world_chr_man, player_ins, 0x190, 0xE8, 0x164]),
            // GameDataMan + 0x94
            death_count: PointerChain::new(&[base_addresses.game_data_man, 0x94]),
        }
    }

    /// Read all Torrent-related debug values
    pub fn read_torrent_debug(&self) -> TorrentDebugInfo {
        TorrentDebugInfo {
            ride_param_id: self.ride_param_id.read(),
            is_riding_enabled: self.is_riding_enabled.read(),
            riding: self.riding.read(),
            is_it_a_horse: self.is_it_a_horse.read(),
            horse_state: self.horse_state.read(),
            horse_hp: self.horse_hp.read(),
            is_inside_no_ride_area: self.is_inside_no_ride_area.read(),
        }
    }

    /// Returns true if the player is currently riding Torrent
    /// Uses "HorseState" - returns true if value != 0
    pub fn is_on_torrent(&self) -> bool {
        self.horse_state.read().map(|v| v != 0).unwrap_or(false)
    }

    /// Read the current death count
    pub fn read_death_count(&self) -> Option<u32> {
        self.death_count.read()
    }
}
