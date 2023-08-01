use enum_map::{Enum, EnumMap};

use crate::constants::{TOTAL_COST_ENHANCEMENT, TOTAL_COST_MASTERY, TOTAL_COST_SKILL};

#[derive(Debug, Enum)]
pub enum Skill {
    Mille,
    Tempest,
    Noir,
    Finale,

    AceInTheHole,
    LuckOfTheDraw,
    PhantomsMark,
    RiftBreak,
}

#[derive(Debug)]
pub struct BA {
    // The sum of the contributions of all of the columns here should equal 1.
    pub columns: EnumMap<Skill, BAColumn>,
}

#[derive(Debug)]
pub struct BAColumn {
    pub contribution: f64,
    pub times_used: u64,
}

#[derive(Debug, Clone, Copy, Enum)]
pub enum HexacoreSkill {
    DefyingFate,
    TempestVI,

    AceInTheHole,
    LuckOfTheDraw,
    PhantomsMark,
    RiftBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HexacoreSpec(pub EnumMap<HexacoreSkill, u8>);

impl HexacoreSpec {
    pub fn valid(&self) -> bool {
        (1..=30).contains(&self.0[HexacoreSkill::DefyingFate])
            && (0..=30).contains(&self.0[HexacoreSkill::TempestVI])
            && (0..=30).contains(&self.0[HexacoreSkill::AceInTheHole])
            && (0..=30).contains(&self.0[HexacoreSkill::LuckOfTheDraw])
            && (0..=30).contains(&self.0[HexacoreSkill::PhantomsMark])
            && (0..=30).contains(&self.0[HexacoreSkill::RiftBreak])
    }

    pub fn cost(&self) -> u16 {
        TOTAL_COST_SKILL[self.0[HexacoreSkill::DefyingFate] as usize]
            + TOTAL_COST_MASTERY[self.0[HexacoreSkill::TempestVI] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::AceInTheHole] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::LuckOfTheDraw] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::PhantomsMark] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::RiftBreak] as usize]
    }
}
