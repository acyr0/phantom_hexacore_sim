use lazy_static::lazy_static;

use crate::config::{
    ATT_PERCENT, BOSS_DMG, CD_HAT, ENEMY_PDR, IED, IED_DEBUFFS, IED_SOURCES, MERCEDES_LEGION,
    PRENEWAGE_BA,
};
use crate::constants::FD_ENHANCEMENT;
use crate::types::{HexacoreSkill, HexacoreSpec, Skill, BA};

lazy_static! {
    pub static ref TEMPEST_CD: f64 = {
        let mut ret = 18.0 * 0.8 * (1.0 - MERCEDES_LEGION) - CD_HAT as f64;
        if ret < 10.0 {
            ret += (10.0 - ret) / 2.0;
        }
        ret
    };

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

fn simulate_origin(ba: &BA, origin: u8, att_percent: f64, boss: f64, remaining_pdr: f64) -> f64 {
    if origin == 0 {
        return 0.0;
    }

    let lotd_damage: f64 = (770 * 420) as f64;
    // Part of the Defying Fate damage over time will land outside of peak buffs (specifically,
    // RoR, LR2, AB link). Calculate a multiplier for the remainder of the damage.
    //
    // TODO: Bishop Bene WJ4?
    let non_peak_multiplier: f64 =
        (100.0 + att_percent) / (100.0 + att_percent + 100.0) *     // Ring of Restraint 4
        1.0 / 1.36 *                                                // Last Resort 2
        (100.0 + boss) / (100.0 + boss + 60.0)                      // AB Link
    ;

    let mut defying_fate_damage: f64 = ((3150 + origin as u64 * 105) * 15 * 7) as f64;
    // Split the Defying Fate damage, 6 in all the buffs LotD receives, 10 outside.
    // 6 is a bit generous, 5 might be more realistic, not too sure.
    defying_fate_damage += ((3600 + origin as u64 * 120) * 4 * 6) as f64;
    let defying_fate_dot = ((3600 + origin as u64 * 120) * 4 * 10) as f64;

    let breakpoint_multiplier = if origin == 30 {
        (100.0 + boss + 50.0) / (100.0 + boss) * (1.0 - remaining_pdr * 0.50)
            / (1.0 - remaining_pdr)
    } else if origin >= 20 {
        (100.0 + boss + 20.0) / (100.0 + boss) * (1.0 - remaining_pdr * 0.80)
            / (1.0 - remaining_pdr)
    } else if origin >= 10 {
        (1.0 - remaining_pdr * 0.80) / (1.0 - remaining_pdr)
    } else {
        1.0
    };

    // Divided by 2 at the end because the origin skill has 2x the CD.
    (defying_fate_damage * ba.columns[Skill::LuckOfTheDraw].contribution / lotd_damage
        + defying_fate_dot * non_peak_multiplier * ba.columns[Skill::LuckOfTheDraw].contribution
            / lotd_damage)
        * breakpoint_multiplier
        / 2.0
}

fn simulate_tempest(ba: &BA, mastery: u8, tempest_cd: f64) -> f64 {
    if mastery == 0 {
        return 0.0;
    }

    // Mille hits every 150ms.
    const MILLE_PERIOD: f64 = 0.15;

    const OLD_TEMPEST_DURATION: f64 = 5.0;
    const OLD_TEMPEST_HITS: u64 = 28;
    const OLD_TEMPEST_DAMAGE: f64 = (460 * 3 * OLD_TEMPEST_HITS) as f64;
    const NEW_TEMPEST_DURATION: f64 = 3.0;
    const NEW_TEMPEST_HITS: u64 = 15;
    let new_tempest_damage = ((480 + mastery as u64 * 12) * 4 * NEW_TEMPEST_HITS) as f64;

    let cur_tempest_count = ba.columns[Skill::Tempest].times_used as f64 / OLD_TEMPEST_HITS as f64;
    let cur_tempest_duration = cur_tempest_count * OLD_TEMPEST_DURATION;
    let cur_mille_duration = ba.columns[Skill::Mille].times_used as f64 * MILLE_PERIOD;

    // We should be able to do more of the new tempest because it has a lower
    // effective CD.
    let new_tempest_count = cur_tempest_count * (OLD_TEMPEST_DURATION + tempest_cd)
        / (NEW_TEMPEST_DURATION + tempest_cd);
    let new_tempest_duration = new_tempest_count * NEW_TEMPEST_DURATION;
    // New tempest is shorter, so fill in the rest of the time with Mille.
    let extra_mille_duration = cur_tempest_duration - new_tempest_duration;

    let extra_noir = extra_mille_duration / MILLE_PERIOD
        + new_tempest_count * NEW_TEMPEST_HITS as f64
        - cur_tempest_count * OLD_TEMPEST_HITS as f64;

    // Subtract 1 here because we only want the _extra_ damage offered by the new
    // tempest over the old one.
    ((new_tempest_count * new_tempest_damage) / (cur_tempest_count * OLD_TEMPEST_DAMAGE) - 1.0)
        * ba.columns[Skill::Tempest].contribution
        + extra_mille_duration / cur_mille_duration * ba.columns[Skill::Mille].contribution
        + extra_noir / ba.columns[Skill::Noir].times_used as f64
            * ba.columns[Skill::Noir].contribution
}

pub fn simulate_hexacores(spec: HexacoreSpec) -> f64 {
    let defying_fate_contrib = simulate_origin(
        &PRENEWAGE_BA,
        spec.0[HexacoreSkill::DefyingFate],
        ATT_PERCENT,
        BOSS_DMG,
        *REMAINING_PDR,
    );
    let tempestvi_contrib =
        simulate_tempest(&PRENEWAGE_BA, spec.0[HexacoreSkill::TempestVI], *TEMPEST_CD);

    (FD_ENHANCEMENT[spec.0[HexacoreSkill::LuckOfTheDraw] as usize] as f64 / 100.0)
        * PRENEWAGE_BA.columns[Skill::LuckOfTheDraw].contribution
        + (FD_ENHANCEMENT[spec.0[HexacoreSkill::AceInTheHole] as usize] as f64 / 100.0)
            * PRENEWAGE_BA.columns[Skill::AceInTheHole].contribution
        + (FD_ENHANCEMENT[spec.0[HexacoreSkill::RiftBreak] as usize] as f64 / 100.0)
            * PRENEWAGE_BA.columns[Skill::RiftBreak].contribution
        + (FD_ENHANCEMENT[spec.0[HexacoreSkill::PhantomsMark] as usize] as f64 / 100.0)
            * PRENEWAGE_BA.columns[Skill::PhantomsMark].contribution
        + defying_fate_contrib
        + tempestvi_contrib
}
