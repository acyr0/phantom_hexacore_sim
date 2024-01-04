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
    20, // Penombre
];

lazy_static! {
    pub static ref BA_SPEC: HexacoreSpec = HexacoreSpec(enum_map! {
        HexacoreSkill::DefyingFate => 9,
        HexacoreSkill::TempestVI => 1,

        HexacoreSkill::LuckOfTheDraw => 9,
        HexacoreSkill::AceInTheHole => 1,
        HexacoreSkill::PhantomsMark => 1,
        HexacoreSkill::RiftBreak => 0,
    });

    /// 6 burst 3 origin Extreme Seren BA.
    pub static ref POSTNEWAGE_BA: BA = BA {
        columns: enum_map! {
            Skill::Noir => BAColumn {
                contribution: 0.1392,
            },
            Skill::AceInTheHole => BAColumn {
                contribution: 0.1519,
            },
            Skill::Tempest => BAColumn {
                contribution: 0.0741,
            },
            Skill::LuckOfTheDraw => BAColumn {
                contribution: 0.2575,
            },
            Skill::DefyingFate => BAColumn {
                contribution: 0.2175,
            },
            Skill::PhantomsMark => BAColumn {
                contribution: 0.0569,
            },
            Skill::Mille => BAColumn {
                contribution: 0.0227,
            },
            Skill::RiftBreak => BAColumn {
                contribution: 0.0450,
            },
            Skill::Finale => BAColumn {
                contribution: 0.0095 + 0.0088,
            },
        },
    };
}
