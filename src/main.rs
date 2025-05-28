mod calcs;
mod config;
mod constants;
mod types;

use enum_map::enum_map;

use crate::calcs::simulate_hexacores;
use crate::types::{HexacoreSkill, HexacoreSpec};

/// FD is multiplicative, so we use the nth root to calculate a per-cost amount.
fn fd_per_cost(new_fd_increase: f64, old_fd_increase: f64, new_cost: u16, old_cost: u16) -> f64 {
    f64::powf(
        (1.0 + new_fd_increase) / (1.0 + old_fd_increase),
        1.0 / (new_cost - old_cost) as f64,
    ) - 1.0
}

fn differing_skills(a: HexacoreSpec, b: HexacoreSpec) -> Vec<HexacoreSkill> {
    let mut ret = Vec::new();
    for (skill, _) in a.0 {
        if a.0[skill] != b.0[skill] {
            ret.push(skill);
        }
    }
    ret
}

/// Don't print multiple lines when the same skill is increasing in level.
fn should_display(next: HexacoreSpec, last_printed: HexacoreSpec) -> bool {
    differing_skills(next, last_printed).len() > 1
}

fn display(cur: HexacoreSpec, last: HexacoreSpec) {
    let base_fd = simulate_hexacores(HexacoreSpec(enum_map! { _ => 0 }));

    let last_cost = last.cost();
    let last_fd = (1.0 + simulate_hexacores(last)) / (1.0 + base_fd) - 1.0;

    let cost = cur.cost();
    let fd = (1.0 + simulate_hexacores(cur)) / (1.0 + base_fd) - 1.0;

    let fd_diff = if cur != last {
        format!(
            "{:>6.4}%",
            100.0 * fd_per_cost(fd, last_fd, cost, last_cost)
        )
    } else {
        "       ".to_owned()
    };
    #[cfg(feature = "table_output")]
    {
        let changed = differing_skills(cur, last);
        if changed.len() > 0 {
            assert_eq!(changed.len(), 1);
            let changed = changed[0];
            let skill_name = match changed {
                HexacoreSkill::DefyingFate => "Defying Fate",
                HexacoreSkill::TempestVI => "Tempest VI",
                HexacoreSkill::MilleVI => "Mille VI",
                HexacoreSkill::AceInTheHole => "Ace",
                HexacoreSkill::LuckOfTheDraw => "LotD",
                HexacoreSkill::PhantomsMark => "Phantom's Mark",
                HexacoreSkill::RiftBreak => "Rift Break",
            };
            let start = last.0[changed] + 1;
            let end = cur.0[changed];
            let level_diff = if end == start {
                end.to_string()
            } else {
                format!("{start}-{end}")
            };
            println!("{skill_name} {level_diff}\t{fd_diff}");
        }
    }

    #[cfg(not(feature = "table_output"))]
    println!(
        "Cost: {:<5}    FD Gain: {:5.2}%    FD/cost: {}    Origin: {:<2}    Tempest VI: {:<2}    Mille VI: {:<2}    LotD: {:<2}    Ace: {:<2}    Mark: {:<2}    Rift Break: {:<2}",
        cur.cost(),
        100.0 * fd,
        fd_diff,
        cur.0[HexacoreSkill::DefyingFate],
        cur.0[HexacoreSkill::TempestVI],
        cur.0[HexacoreSkill::MilleVI],
        cur.0[HexacoreSkill::LuckOfTheDraw],
        cur.0[HexacoreSkill::AceInTheHole],
        cur.0[HexacoreSkill::PhantomsMark],
        cur.0[HexacoreSkill::RiftBreak],
    );
}

/// We rely on a couple of observations here to reduce the possible search space of next possible
/// configuration:
///   - We only need to look at configurations that increase single skills, as the fd/cost
///     efficiency of increasing 2 different skills will never be higher than 1 of them individually.
fn best_next_skill(spec: HexacoreSpec) -> HexacoreSpec {
    let old_fd = simulate_hexacores(spec);
    let old_cost = spec.cost();

    let mut choices = Vec::new();
    for (skill, &level) in &spec.0 {
        let mut new_spec = spec.clone();

        for next_level in (level + 1)..=30 {
            new_spec.0[skill] = next_level;
            if new_spec.valid() {
                let effic = fd_per_cost(
                    simulate_hexacores(new_spec),
                    old_fd,
                    new_spec.cost(),
                    old_cost,
                );
                choices.push((effic, new_spec.clone()));
            }
        }
    }
    choices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    choices.last().expect("no choices found").1
}

fn main() {
    let start: HexacoreSpec = HexacoreSpec(enum_map! {
        HexacoreSkill::DefyingFate => 1,
        _ => 0,
    });
    let goal: HexacoreSpec = HexacoreSpec(enum_map! {
        _ => 30,
    });

    display(start, start);

    let mut cur = start;
    let mut last_printed = start;
    while cur != goal {
        let next = best_next_skill(cur);

        if should_display(next, last_printed) {
            display(cur, last_printed);
            last_printed = cur;
        }

        cur = next;
    }

    display(goal, last_printed);
}
