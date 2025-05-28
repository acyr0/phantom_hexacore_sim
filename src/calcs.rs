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

        ((390 + level as u64 * 13) * 15 * 57 + (310 + level as u64 * 11) * 15 * 50) as f64
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

    (new_damage / old_damage * new_breakpoint_multiplier / old_breakpoint_multiplier - 1.0)
        * ba.columns[Skill::DefyingFate].contribution
}

fn simulate_tempest(ba: &BA, old_mastery: u8, cur_mastery: u8) -> f64 {
    fn tempest_damage(level: u8) -> f64 {
        if level == 0 {
            // Assume that the damage before and after is relatively the same.
            tempest_damage(1)
        } else {
            ((480 + level as u64 * 12) * 4 * 15) as f64
        }
    }

    let old_tempest_damage: f64 = tempest_damage(old_mastery);
    let new_tempest_damage: f64 = tempest_damage(cur_mastery);

    (new_tempest_damage / old_tempest_damage - 1.0) * ba.columns[Skill::Tempest].contribution
}

fn simulate_mille(ba: &BA, old_mastery: u8, cur_mastery: u8) -> f64 {
    fn mille_damage(level: u8) -> f64 {
        if level > 0 {
            ((300 + level as u64 * 5) * 3) as f64
        } else {
            (161 * 3) as f64
        }
    }

    let old_mille_damage: f64 = mille_damage(old_mastery);
    let new_mille_damage: f64 = mille_damage(cur_mastery);

    (new_mille_damage / old_mille_damage - 1.0) * ba.columns[Skill::Mille].contribution
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
    let millevi_contrib = simulate_mille(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::MilleVI],
        spec.0[HexacoreSkill::MilleVI],
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
        + millevi_contrib
}
