use enum_map::enum_map;
use lazy_static::lazy_static;

use crate::types::{BAColumn, Skill, BA};

/// Only used for the calculation for Tempest -> Tempest VI.
pub const CD_HAT: u8 = 4;
pub const MERCEDES_LEGION: f64 = 0.06;

pub const ENEMY_PDR: f64 = 300.0;

/// Sum of ATT % sources from WSE/Familiar badges/Hero's Echo/etc.
pub const ATT_PERCENT: f64 = 88.0;

/// Damage + Boss Damage from the Stats window. Don't include the base 100% here.
pub const BOSS_DMG: f64 = 900.0;

/// IED from the Stats window.
pub const IED: f64 = 0.9774;
/// Any IED sources that are part of the Stats window calculation.
pub const IED_SOURCES: &'static [u8] = &[
    31, // Priere D'Aria + DCO
    20, // Luminous link
    30, // Event buff
    10, // CRA Hat
    5,  // CRA Top
    5,  // CRA Bottom
    20, // Weapon
    10, // 7 Day Monster Parker medal
    3,  // Familiar badge
    3,  // Familiar badge
    40, // Familiar effect
    10, // Arcane Umbra 3 set
    30, // Superior Gollux
    10, // Pitched 3 set
    5,  // Blaster legion
    5,  // Beast tamer legion
    40, // Legion board
    36, // 12 IED hyper
    10, // Ambition
];
/// Any IED sources that are not part of the Stats window calculation
pub const IED_DEBUFFS: &'static [u8] = &[
    20, // Penombre
];

lazy_static! {
    /// 4.5 min Hard Seren P2 ft. Kanna + Bishop
    pub static ref PRENEWAGE_BA: BA = BA {
        columns: enum_map! {
            Skill::Noir => BAColumn {
                contribution: 0.2122,
                times_used: 1813,
            },
            Skill::AceInTheHole => BAColumn {
                contribution: 0.1644,
                times_used: 410,
            },
            Skill::Tempest => BAColumn {
                contribution: 0.0889,
                times_used: 187,
            },
            Skill::LuckOfTheDraw => BAColumn {
                contribution: 0.2990,
                times_used: 840,
            },
            Skill::PhantomsMark => BAColumn {
                contribution: 0.0809,
                times_used: 86,
            },
            Skill::Mille => BAColumn {
                contribution: 0.0658,
                times_used: 499,
            },
            Skill::RiftBreak => BAColumn {
                contribution: 0.0597,
                times_used: 58,
            },
            Skill::Finale => BAColumn {
                contribution: 0.0152 + 0.0135,
                times_used: 10,
            },
        },
    };
}

// lazy_static! {
//     /// 9 min dummy BA starting and ending in a burst.
//     pub static ref PRENEWAGE_BA: BA = BA {
//         columns: enum_map! {
//             Skill::Noir => BAColumn {
//                 contribution: 0.2051,
//                 times_used: 4970,
//             },
//             Skill::AceInTheHole => BAColumn {
//                 contribution: 0.1744,
//                 times_used: 1029,
//             },
//             Skill::Tempest => BAColumn {
//                 contribution: 0.1603,
//                 times_used: 851,
//             },
//             Skill::LuckOfTheDraw => BAColumn {
//                 contribution: 0.1652,
//                 times_used: 1680,
//             },
//             Skill::PhantomsMark => BAColumn {
//                 contribution: 0.0811,
//                 times_used: 189,
//             },
//             Skill::Mille => BAColumn {
//                 contribution: 0.0764,
//                 times_used: 1271,
//             },
//             Skill::RiftBreak => BAColumn {
//                 contribution: 0.0606,
//                 times_used: 147,
//             },
//             Skill::Finale => BAColumn {
//                 contribution: 0.0234 + 0.0137,
//                 times_used: 21,
//             },
//         },
//     };
// }
