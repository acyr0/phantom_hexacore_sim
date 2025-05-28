use enum_map::enum_map;
use lazy_static::lazy_static;

use crate::types::{BAColumn, HexacoreSkill, HexacoreSpec, Skill, BA};

pub const ENEMY_PDR: f64 = 300.0;

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
    20, // Tempest
];

// lazy_static! {
//     pub static ref BA_SPEC: HexacoreSpec = HexacoreSpec(enum_map! {
//         HexacoreSkill::DefyingFate => 30,
//         HexacoreSkill::TempestVI => 30,
//         HexacoreSkill::MilleVI => 29,
//
//         HexacoreSkill::LuckOfTheDraw => 30,
//         HexacoreSkill::AceInTheHole => 30,
//         HexacoreSkill::PhantomsMark => 10,
//         HexacoreSkill::RiftBreak => 10,
//     });
//
//     /// Dummy BA
//     pub static ref POSTNEWAGE_BA: BA = BA {
//         columns: enum_map! {
//             Skill::Noir => BAColumn {
//                 contribution: 0.1139,
//             },
//             Skill::AceInTheHole => BAColumn {
//                 contribution: 0.1538,
//             },
//             Skill::Tempest => BAColumn {
//                 contribution: 0.1855,
//             },
//             Skill::LuckOfTheDraw => BAColumn {
//                 contribution: 0.1180,
//             },
//             Skill::DefyingFate => BAColumn {
//                 contribution: 0.1234,
//             },
//             Skill::PhantomsMark => BAColumn {
//                 contribution: 0.0565,
//             },
//             Skill::Mille => BAColumn {
//                 contribution: 0.1695,
//             },
//             Skill::RiftBreak => BAColumn {
//                 contribution: 0.0449,
//             },
//             Skill::Finale => BAColumn {
//                 contribution: 0.0122 + 0.0062,
//             },
//         },
//     };
// }

// lazy_static! {
//     pub static ref BA_SPEC: HexacoreSpec = HexacoreSpec(enum_map! {
//         HexacoreSkill::DefyingFate => 18,
//         HexacoreSkill::TempestVI => 19,
//         HexacoreSkill::MilleVI => 0,
//
//         HexacoreSkill::LuckOfTheDraw => 11,
//         HexacoreSkill::AceInTheHole => 10,
//         HexacoreSkill::PhantomsMark => 1,
//         HexacoreSkill::RiftBreak => 1,
//     });
//
//     /// Culvert BA
//     pub static ref POSTNEWAGE_BA: BA = BA {
//         columns: enum_map! {
//             Skill::Noir => BAColumn {
//                 contribution: 0.1117,
//             },
//             Skill::AceInTheHole => BAColumn {
//                 contribution: 0.1237,
//             },
//             Skill::Tempest => BAColumn {
//                 contribution: 0.1496,
//             },
//             Skill::LuckOfTheDraw => BAColumn {
//                 contribution: 0.1275,
//             },
//             Skill::DefyingFate => BAColumn {
//                 contribution: 0.2614,
//             },
//             Skill::PhantomsMark => BAColumn {
//                 contribution: 0.0638,
//             },
//             Skill::Mille => BAColumn {
//                 contribution: 0.0383,
//             },
//             Skill::RiftBreak => BAColumn {
//                 contribution: 0.0471,
//             },
//             Skill::Finale => BAColumn {
//                 contribution: 0.0304 + 0.0052,
//             },
//         },
//     };
// }

// lazy_static! {
//     pub static ref BA_SPEC: HexacoreSpec = HexacoreSpec(enum_map! {
//         HexacoreSkill::DefyingFate => 30,
//         HexacoreSkill::TempestVI => 30,
//         HexacoreSkill::MilleVI => 29,
//
//         HexacoreSkill::LuckOfTheDraw => 30,
//         HexacoreSkill::AceInTheHole => 30,
//         HexacoreSkill::PhantomsMark => 10,
//         HexacoreSkill::RiftBreak => 10,
//     });
//
//     /// Seren BA 2024-11-22 16-38-58
//     pub static ref POSTNEWAGE_BA: BA = BA {
//         columns: enum_map! {
//             Skill::Noir => BAColumn {
//                 contribution: 0.1117,
//             },
//             Skill::AceInTheHole => BAColumn {
//                 contribution: 0.1713,
//             },
//             Skill::Tempest => BAColumn {
//                 contribution: 0.1243,
//             },
//             Skill::LuckOfTheDraw => BAColumn {
//                 contribution: 0.1787,
//             },
//             Skill::DefyingFate => BAColumn {
//                 contribution: 0.1784,
//             },
//             Skill::PhantomsMark => BAColumn {
//                 contribution: 0.0492,
//             },
//             Skill::Mille => BAColumn {
//                 contribution: 0.1167,
//             },
//             Skill::RiftBreak => BAColumn {
//                 contribution: 0.0436,
//             },
//             Skill::Finale => BAColumn {
//                 contribution: 0.0091 + 0.0078,
//             },
//         },
//     };
// }

lazy_static! {
    pub static ref BA_SPEC: HexacoreSpec = HexacoreSpec(enum_map! {
        HexacoreSkill::DefyingFate => 30,
        HexacoreSkill::TempestVI => 29,
        HexacoreSkill::MilleVI => 19,

        HexacoreSkill::LuckOfTheDraw => 30,
        HexacoreSkill::AceInTheHole => 26,
        HexacoreSkill::PhantomsMark => 10,
        HexacoreSkill::RiftBreak => 10,
    });

    /// Extreme Lotus BA
    pub static ref POSTNEWAGE_BA: BA = BA {
        columns: enum_map! {
            Skill::Noir => BAColumn {
                contribution: 0.1124,
            },
            Skill::AceInTheHole => BAColumn {
                contribution: 0.1825,
            },
            Skill::Tempest => BAColumn {
                contribution: 0.1463,
            },
            Skill::LuckOfTheDraw => BAColumn {
                contribution: 0.1526,
            },
            Skill::DefyingFate => BAColumn {
                contribution: 0.1605,
            },
            Skill::PhantomsMark => BAColumn {
                contribution: 0.0644,
            },
            Skill::Mille => BAColumn {
                contribution: 0.0973,
            },
            Skill::RiftBreak => BAColumn {
                contribution: 0.0536,
            },
            Skill::Finale => BAColumn {
                contribution: 0.0125 + 0.0052,
            },
        },
    };
}
