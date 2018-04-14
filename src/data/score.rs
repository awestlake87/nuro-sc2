use sc2_proto::score::{
    CategoryScoreDetails as ProtoCategoryScoreDetails,
    Score as ProtoScore,
    ScoreDetails as ProtoScoreDetails,
    Score_ScoreType as ProtoScoreType,
    VitalScoreDetails as ProtoVitalScoreDetails,
};

use {FromProto, IntoSc2, Result};

/// Source of a score.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScoreType {
    /// Map generated score (from curriculum maps with special scoring).
    Curriculum,
    /// Melee score.
    ///
    /// Summation of in-progress and current units/buildings value + minerals
    /// + vespene.
    Melee,
}

/// Score evaluated at the end of a game.
#[derive(Debug, Copy, Clone)]
pub struct Score {
    /// Method of scoring.
    score_type: ScoreType,
    /// Overall score.
    score: f32,
    /// More detailed scoring.
    details: ScoreDetails,
}

impl FromProto<ProtoScore> for Score {
    fn from_proto(mut score: ProtoScore) -> Result<Self> {
        Ok(Self {
            score_type: {
                if score.has_score_type() {
                    match score.get_score_type() {
                        ProtoScoreType::Curriculum => ScoreType::Curriculum,
                        ProtoScoreType::Melee => ScoreType::Melee,
                    }
                } else {
                    ScoreType::Melee
                }
            },
            score: score.get_score() as f32,
            details: score.take_score_details().into_sc2()?,
        })
    }
}

/// Score by category.
#[derive(Debug, Copy, Clone)]
pub struct CategoryScoreDetails {
    /// Overall score.
    none: f32,
    /// Military score.
    army: f32,
    /// Economic score.
    economy: f32,
    /// Tech score.
    technology: f32,
    /// Upgrade score.
    upgrade: f32,
}

impl FromProto<ProtoCategoryScoreDetails> for CategoryScoreDetails {
    fn from_proto(details: ProtoCategoryScoreDetails) -> Result<Self> {
        Ok(Self {
            none: details.get_none(),
            army: details.get_army(),
            economy: details.get_economy(),
            technology: details.get_technology(),
            upgrade: details.get_upgrade(),
        })
    }
}

/// Details related to health or damage.
#[derive(Debug, Copy, Clone)]
pub struct VitalScoreDetails {
    /// Health score.
    life: f32,
    /// Shield score.
    shields: f32,
    /// Energy score.
    energy: f32,
}

impl FromProto<ProtoVitalScoreDetails> for VitalScoreDetails {
    fn from_proto(details: ProtoVitalScoreDetails) -> Result<Self> {
        Ok(Self {
            life: details.get_life(),
            shields: details.get_shields(),
            energy: details.get_energy(),
        })
    }
}

/// Detailed scoring.
#[derive(Debug, Copy, Clone)]
pub struct ScoreDetails {
    /// Time elapsed while production was idle.
    idle_production_time: f32,
    /// Time elapsed while workers were idle.
    idle_worker_time: f32,

    /// Total unit value.
    total_value_units: f32,
    /// Total structural value.
    total_value_structures: f32,

    /// Value of enemy units destroyed.
    ///
    /// Note that this field is a combo of minerals, vespene, and a human
    /// designer guess. Might be useful as a delta. the weighting of the
    /// combination and the human designer guess is asymmetric with the total
    /// value.
    killed_value_units: f32,
    /// Value of enemy structures destroyed.
    ///
    /// Note that this field is a combo of minerals, vespene, and a human
    /// designer guess. Might be useful as a delta. the weighting of the
    /// combination and the human designer guess is asymmetric with the total
    /// value.
    killed_value_structures: f32,

    /// Total minerals collected.
    collected_minerals: f32,
    /// Total vespene collected.
    collected_vespene: f32,

    /// Collection rate of minerals.
    collection_rate_minerals: f32,
    /// Collection rate of vespene.
    collection_rate_vespene: f32,

    /// Total minerals spent.
    spent_minerals: f32,
    /// Total vespene spent.
    spent_vespene: f32,

    /// Total food used.
    food_used: Option<CategoryScoreDetails>,

    /// TODO: Find out what this means.
    killed_minerals: Option<CategoryScoreDetails>,
    /// TODO: Find out what this means.
    killed_vespene: Option<CategoryScoreDetails>,

    /// TODO: Find out what this means.
    lost_minerals: Option<CategoryScoreDetails>,
    /// TODO: Find out what this means.
    lost_vespene: Option<CategoryScoreDetails>,

    /// TODO: Find out what this means.
    friendly_fire_minerals: Option<CategoryScoreDetails>,
    /// TODO: Find out what this means.
    friendly_fire_vespene: Option<CategoryScoreDetails>,

    /// TODO: Find out what this means.
    used_minerals: Option<CategoryScoreDetails>,
    /// TODO: Find out what this means.
    used_vespene: Option<CategoryScoreDetails>,

    /// TODO: Find out what this means.
    total_used_minerals: Option<CategoryScoreDetails>,
    /// TODO: Find out what this means.
    total_used_vespene: Option<CategoryScoreDetails>,

    /// Total damage dealt to enemies.
    total_damage_dealt: Option<VitalScoreDetails>,
    /// Total damage taken from enemies.
    total_damage_taken: Option<VitalScoreDetails>,
    /// Total damage healed.
    total_healed: Option<VitalScoreDetails>,
}

impl FromProto<ProtoScoreDetails> for ScoreDetails {
    fn from_proto(mut details: ProtoScoreDetails) -> Result<Self> {
        Ok(Self {
            idle_production_time: details.get_idle_production_time(),
            idle_worker_time: details.get_idle_worker_time(),

            total_value_units: details.get_total_value_units(),
            total_value_structures: details.get_total_value_structures(),

            killed_value_units: details.get_killed_value_units(),
            killed_value_structures: details.get_killed_value_structures(),

            collected_minerals: details.get_collected_minerals(),
            collected_vespene: details.get_collected_vespene(),

            collection_rate_minerals: details.get_collection_rate_minerals(),
            collection_rate_vespene: details.get_collection_rate_vespene(),

            spent_minerals: details.get_spent_minerals(),
            spent_vespene: details.get_spent_vespene(),

            food_used: {
                if details.has_food_used() {
                    Some(details.take_food_used().into_sc2()?)
                } else {
                    None
                }
            },

            killed_minerals: {
                if details.has_killed_minerals() {
                    Some(details.take_killed_minerals().into_sc2()?)
                } else {
                    None
                }
            },
            killed_vespene: {
                if details.has_killed_vespene() {
                    Some(details.take_killed_vespene().into_sc2()?)
                } else {
                    None
                }
            },

            lost_minerals: {
                if details.has_lost_minerals() {
                    Some(details.take_lost_minerals().into_sc2()?)
                } else {
                    None
                }
            },
            lost_vespene: {
                if details.has_lost_vespene() {
                    Some(details.take_lost_vespene().into_sc2()?)
                } else {
                    None
                }
            },

            friendly_fire_minerals: {
                if details.has_friendly_fire_minerals() {
                    Some(details
                        .take_friendly_fire_minerals()
                        .into_sc2()?)
                } else {
                    None
                }
            },
            friendly_fire_vespene: {
                if details.has_friendly_fire_vespene() {
                    Some(details.take_friendly_fire_vespene().into_sc2()?)
                } else {
                    None
                }
            },

            used_minerals: {
                if details.has_used_minerals() {
                    Some(details.take_used_minerals().into_sc2()?)
                } else {
                    None
                }
            },
            used_vespene: {
                if details.has_used_vespene() {
                    Some(details.take_used_vespene().into_sc2()?)
                } else {
                    None
                }
            },

            total_used_minerals: {
                if details.has_total_used_minerals() {
                    Some(details.take_total_used_minerals().into_sc2()?)
                } else {
                    None
                }
            },
            total_used_vespene: {
                if details.has_total_used_vespene() {
                    Some(details.take_total_used_vespene().into_sc2()?)
                } else {
                    None
                }
            },

            total_damage_dealt: {
                if details.has_total_damage_dealt() {
                    Some(details.take_total_damage_dealt().into_sc2()?)
                } else {
                    None
                }
            },
            total_damage_taken: {
                if details.has_total_damage_taken() {
                    Some(details.take_total_damage_taken().into_sc2()?)
                } else {
                    None
                }
            },
            total_healed: {
                if details.has_total_healed() {
                    Some(details.take_total_healed().into_sc2()?)
                } else {
                    None
                }
            },
        })
    }
}
