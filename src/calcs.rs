use lazy_static::lazy_static;

use crate::config::{BA_SPEC, BOSS_DMG, ENEMY_PDR, IED, IED_DEBUFFS, IED_SOURCES, POSTNEWAGE_BA};
use crate::constants::FD_ENHANCEMENT;
use crate::types::{HexacoreSkill, HexacoreSpec, Skill, BA};

lazy_static! {
    pub static ref REMAINING_PDR: f64 = {
        let mut defense_pen = 1.0;
        for &ied in IED_SOURCES {
            defense_pen *= 1.0 - ied as f64 / 100.0;
        }

        // Check that the calculated IED matches the Stats window IED.
        assert_eq!(
            ((1.0 - defense_pen) * 1000.0).trunc(),
            (IED * 1000.0).trunc(),
            "Stats window IED doesn't match calculated IED"
        );

        for &ied in IED_DEBUFFS {
            defense_pen *= 1.0 - ied as f64 / 100.0;
        }
        defense_pen * ENEMY_PDR / 100.0
    };
}

fn simulate_origin(ba: &BA, old_origin: u8, cur_origin: u8, boss: f64, remaining_pdr: f64) -> f64 {
    fn defying_fate_damage(level: u8) -> f64 {
        if level == 0 {
            return 0.0;
        }

        ((3150 + level as u64 * 105) * 15 * 7 + (3600 + level as u64 * 120) * 4 * 16) as f64
    }

    let old_damage: f64 = defying_fate_damage(old_origin);
    let new_damage: f64 = defying_fate_damage(cur_origin);

    fn breakpoint_multiplier(level: u8, boss: f64, remaining_pdr: f64) -> f64 {
        if level == 30 {
            (100.0 + boss + 50.0) / (100.0 + boss) * (1.0 - remaining_pdr * 0.50)
                / (1.0 - remaining_pdr)
        } else if level >= 20 {
            (100.0 + boss + 20.0) / (100.0 + boss) * (1.0 - remaining_pdr * 0.80)
                / (1.0 - remaining_pdr)
        } else if level >= 10 {
            (1.0 - remaining_pdr * 0.80) / (1.0 - remaining_pdr)
        } else {
            1.0
        }
    }

    let old_breakpoint_multiplier = breakpoint_multiplier(old_origin, boss, remaining_pdr);
    let new_breakpoint_multiplier = breakpoint_multiplier(cur_origin, boss, remaining_pdr);

    (new_damage / old_damage - 1.0)
        * (new_breakpoint_multiplier / old_breakpoint_multiplier)
        * ba.columns[Skill::DefyingFate].contribution
}

fn simulate_tempest(ba: &BA, old_mastery: u8, cur_mastery: u8) -> f64 {
    if cur_mastery == 0 {
        return 0.0;
    }

    fn tempest_damage(level: u8) -> f64 {
        assert!(level > 0);
        ((480 + level as u64 * 12) * 4 * 15) as f64
    }

    let old_tempest_damage: f64 = tempest_damage(old_mastery);
    let new_tempest_damage: f64 = tempest_damage(cur_mastery);

    (new_tempest_damage / old_tempest_damage - 1.0) * ba.columns[Skill::Tempest].contribution
}

pub fn simulate_hexacores(spec: HexacoreSpec) -> f64 {
    let defying_fate_contrib = simulate_origin(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::DefyingFate],
        spec.0[HexacoreSkill::DefyingFate],
        BOSS_DMG,
        *REMAINING_PDR,
    );
    let tempestvi_contrib = simulate_tempest(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::TempestVI],
        spec.0[HexacoreSkill::TempestVI],
    );

    let lotd_fd_increase = (1.0
        + FD_ENHANCEMENT[spec.0[HexacoreSkill::LuckOfTheDraw] as usize] as f64 / 100.0)
        / (1.0 + FD_ENHANCEMENT[BA_SPEC.0[HexacoreSkill::LuckOfTheDraw] as usize] as f64 / 100.0)
        - 1.0;
    let ace_fd_increase = (1.0
        + FD_ENHANCEMENT[spec.0[HexacoreSkill::AceInTheHole] as usize] as f64 / 100.0)
        / (1.0 + FD_ENHANCEMENT[BA_SPEC.0[HexacoreSkill::AceInTheHole] as usize] as f64 / 100.0)
        - 1.0;
    let rift_break_fd_increase = (1.0
        + FD_ENHANCEMENT[spec.0[HexacoreSkill::RiftBreak] as usize] as f64 / 100.0)
        / (1.0 + FD_ENHANCEMENT[BA_SPEC.0[HexacoreSkill::RiftBreak] as usize] as f64 / 100.0)
        - 1.0;
    let mark_fd_increase = (1.0
        + FD_ENHANCEMENT[spec.0[HexacoreSkill::PhantomsMark] as usize] as f64 / 100.0)
        / (1.0 + FD_ENHANCEMENT[BA_SPEC.0[HexacoreSkill::PhantomsMark] as usize] as f64 / 100.0)
        - 1.0;

    lotd_fd_increase * POSTNEWAGE_BA.columns[Skill::LuckOfTheDraw].contribution
        + ace_fd_increase * POSTNEWAGE_BA.columns[Skill::AceInTheHole].contribution
        + rift_break_fd_increase * POSTNEWAGE_BA.columns[Skill::RiftBreak].contribution
        + mark_fd_increase * POSTNEWAGE_BA.columns[Skill::PhantomsMark].contribution
        + defying_fate_contrib
        + tempestvi_contrib
}
