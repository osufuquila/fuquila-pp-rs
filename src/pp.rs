use crate::{
    fruits::FruitsDifficultyAttributes, mania::ManiaDifficultyAttributes,
    osu::OsuDifficultyAttributes, taiko::TaikoDifficultyAttributes, Beatmap, DifficultyAttributes,
    GameMode, PerformanceAttributes,
};

#[cfg(feature = "fruits")]
use crate::FruitsPP;

#[cfg(feature = "mania")]
use crate::ManiaPP;

#[cfg(feature = "osu")]
use crate::OsuPP;

#[cfg(feature = "taiko")]
use crate::TaikoPP;

/// Performance calculator on maps of any mode.
#[allow(clippy::upper_case_acronyms)]
pub enum AnyPP<'m> {
    #[cfg(feature = "fruits")]
    Fruits(FruitsPP<'m>),
    #[cfg(feature = "mania")]
    Mania(ManiaPP<'m>),
    #[cfg(feature = "osu")]
    Osu(OsuPP<'m>),
    #[cfg(feature = "taiko")]
    Taiko(TaikoPP<'m>),
}

impl<'m> AnyPP<'m> {
    #[inline]
    pub fn new(map: &'m Beatmap) -> Self {
        match map.mode {
            #[cfg(feature = "fruits")]
            GameMode::CTB => Self::Fruits(FruitsPP::new(map)),
            #[cfg(feature = "mania")]
            GameMode::MNA => Self::Mania(ManiaPP::new(map)),
            #[cfg(feature = "osu")]
            GameMode::STD => Self::Osu(OsuPP::new(map)),
            #[cfg(feature = "taiko")]
            GameMode::TKO => Self::Taiko(TaikoPP::new(map)),
            #[allow(unreachable_patterns)]
            _ => panic!("feature for mode {:?} is not enabled", map.mode),
        }
    }

    #[inline]
    pub fn calculate(self) -> PerformanceAttributes {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => PerformanceAttributes::Fruits(f.calculate()),
            #[cfg(feature = "mania")]
            Self::Mania(m) => PerformanceAttributes::Mania(m.calculate()),
            #[cfg(feature = "osu")]
            Self::Osu(o) => PerformanceAttributes::Osu(o.calculate()),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => PerformanceAttributes::Taiko(t.calculate()),
        }
    }

    /// Provide the result of previous a difficulty or performance calculation.
    /// If you already calculated the attributes for the current map-mod combination,
    /// be sure to put them in here so that they don't have to be recalculated.
    #[inline]
    pub fn attributes(self, attributes: impl AttributeProvider) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.attributes(attributes.attributes())),
            #[cfg(feature = "mania")]
            Self::Mania(m) => Self::Mania(m.attributes(attributes.attributes())),
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.attributes(attributes.attributes())),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.attributes(attributes.attributes())),
        }
    }

    /// Specify mods through their bit values.
    ///
    /// See [https://github.com/ppy/osu-api/wiki#mods](https://github.com/ppy/osu-api/wiki#mods)
    #[inline]
    pub fn mods(self, mods: u32) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.mods(mods)),
            #[cfg(feature = "mania")]
            Self::Mania(m) => Self::Mania(m.mods(mods)),
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.mods(mods)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.mods(mods)),
        }
    }

    /// Amount of passed objects for partial plays, e.g. a fail.
    #[inline]
    pub fn passed_objects(self, passed_objects: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.passed_objects(passed_objects)),
            #[cfg(feature = "mania")]
            Self::Mania(m) => Self::Mania(m.passed_objects(passed_objects)),
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.passed_objects(passed_objects)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.passed_objects(passed_objects)),
        }
    }

    /// Set the accuracy between 0.0 and 100.0.
    ///
    /// For some modes this method depends on previously set values
    /// be sure to call this last before calling `calculate`.
    ///
    /// Irrelevant for osu!mania.
    #[allow(unused_variables)]
    #[inline]
    pub fn accuracy(self, acc: f64) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.accuracy(acc)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.accuracy(acc)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.accuracy(acc)),
        }
    }

    /// Specify the amount of misses of a play.
    ///
    /// Irrelevant for osu!mania.
    #[allow(unused_variables)]
    #[inline]
    pub fn misses(self, misses: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.misses(misses)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.misses(misses)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.misses(misses)),
        }
    }

    /// Specify the max combo of the play.
    ///
    /// Irrelevant for osu!mania.
    #[allow(unused_variables)]
    #[inline]
    pub fn combo(self, combo: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.combo(combo)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.combo(combo)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.combo(combo)),
        }
    }

    /// Specify the amount of 300s of a play.
    ///
    /// Irrelevant for osu!mania.
    #[allow(unused_variables)]
    #[inline]
    pub fn n300(self, n300: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.fruits(n300)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.n300(n300)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.n300(n300)),
        }
    }

    /// Specify the amount of 100s of a play.
    ///
    /// Irrelevant for osu!mania.
    #[allow(unused_variables)]
    #[inline]
    pub fn n100(self, n100: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.droplets(n100)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.n100(n100)),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => Self::Taiko(t.n100(n100)),
        }
    }

    /// Specify the amount of 50s of a play.
    ///
    /// Irrelevant for osu!mania and osu!taiko.
    #[allow(unused_variables)]
    #[inline]
    pub fn n50(self, n50: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.tiny_droplets(n50)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(o) => Self::Osu(o.n50(n50)),
            #[cfg(feature = "taiko")]
            Self::Taiko(_) => self,
        }
    }

    /// Specify the amount of katus of a play.
    ///
    /// This value is only relevant for osu!ctb for which it represent
    /// the amount of tiny droplet misses.
    #[allow(unused_variables)]
    #[inline]
    pub fn n_katu(self, n_katu: usize) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => Self::Fruits(f.tiny_droplet_misses(n_katu)),
            #[cfg(feature = "mania")]
            Self::Mania(_) => self,
            #[cfg(feature = "osu")]
            Self::Osu(_) => self,
            #[cfg(feature = "taiko")]
            Self::Taiko(_) => self,
        }
    }

    /// Specify the score of a play.
    ///
    /// This value is only relevant for osu!mania.
    ///
    /// On `NoMod` its between 0 and 1,000,000, on `Easy` between 0 and 500,000, etc.
    #[allow(unused_variables)]
    #[inline]
    pub fn score(self, score: u32) -> Self {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(_) => self,
            #[cfg(feature = "mania")]
            Self::Mania(m) => Self::Mania(m.score(score)),
            #[cfg(feature = "osu")]
            Self::Osu(_) => self,
            #[cfg(feature = "taiko")]
            Self::Taiko(_) => self,
        }
    }
}

/// Abstract type to provide flexibility when passing difficulty attributes to a performance calculation.
pub trait AttributeProvider {
    fn attributes(self) -> DifficultyAttributes;
}

impl AttributeProvider for DifficultyAttributes {
    #[inline]
    fn attributes(self) -> DifficultyAttributes {
        self
    }
}

impl AttributeProvider for PerformanceAttributes {
    #[inline]
    fn attributes(self) -> DifficultyAttributes {
        match self {
            #[cfg(feature = "fruits")]
            Self::Fruits(f) => DifficultyAttributes::Fruits(f.attributes),
            #[cfg(feature = "mania")]
            Self::Mania(m) => DifficultyAttributes::Mania(m.attributes),
            #[cfg(feature = "osu")]
            Self::Osu(o) => DifficultyAttributes::Osu(o.attributes),
            #[cfg(feature = "taiko")]
            Self::Taiko(t) => DifficultyAttributes::Taiko(t.attributes),
        }
    }
}

#[cfg(feature = "fruits")]
impl AttributeProvider for FruitsDifficultyAttributes {
    fn attributes(self) -> DifficultyAttributes {
        DifficultyAttributes::Fruits(self)
    }
}

#[cfg(feature = "mania")]
impl AttributeProvider for ManiaDifficultyAttributes {
    fn attributes(self) -> DifficultyAttributes {
        DifficultyAttributes::Mania(self)
    }
}

#[cfg(feature = "osu")]
impl AttributeProvider for OsuDifficultyAttributes {
    fn attributes(self) -> DifficultyAttributes {
        DifficultyAttributes::Osu(self)
    }
}

#[cfg(feature = "taiko")]
impl AttributeProvider for TaikoDifficultyAttributes {
    fn attributes(self) -> DifficultyAttributes {
        DifficultyAttributes::Taiko(self)
    }
}
