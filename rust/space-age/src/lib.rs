//! # space-age
//! space-age is used to convert a duration of seconds into
//! years as defined by various planets in our solar system
//! These include:
//!     - Mercury-years
//!     - Venus-years
//!     - Earth-years (regular years to us humans)
//!     - Mars-years
//!     - Jupiter-years
//!     - Saturn-years
//!     - Uranus-years
//!     - Neptune-years

/// A struct that holds the duration of time in seconds
///
/// # Members
///
/// * `sec` - an f64 that represents time in seconds
#[derive(Debug)]
pub struct Duration {
    sec: f64,
}

/// Block defining the conversion of u64 to Durations
///
/// # Methods
///
/// * `from` - converts a u64 to a Duration struct
impl From<u64> for Duration {
    /// Converts a u64 to Duration
    ///
    /// # Arguments
    ///
    /// * `s` - a u64 representing the amount of seconds
    ///
    /// # Returns
    ///
    /// * `Duration`
    fn from(s: u64) -> Self {
        Duration { sec: s as f64 }
    }
}

/// Has the quality of being a planet
///
/// # Associated Consts
///
/// * `SECONDS_PER_EARTH_YR` - default set to 31_557_600.0
/// * `EARTH_YRS_PER_PLANET_YR` - default not set
///
/// # Associated Methods
///
/// * `years_during` - converts a Duration to a span of planet years
pub trait Planet {
    const SECONDS_PER_EARTH_YR: f64 = 31_557_600.0;
    const EARTH_YRS_PER_PLANET_YR: f64;
    /// Gives the planet years that span the given duration
    ///
    /// # Arguments
    ///
    /// * `d` - the Duration in seconds
    ///
    /// # Returns
    ///
    /// * `f64` - the corresponding amount of planet years
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg_attr(test, macro_use)]
    /// extern crate assert_float_eq;
    /// use assert_float_eq::*;
    /// use space_age::*;
    //
    /// let earth_year = Duration::from(31_557_600);
    /// assert_float_absolute_eq!(Earth::years_during(&earth_year), 1.0);
    ///
    /// let mercury_year = Duration::from(7_600_544);
    /// assert_float_absolute_eq!(Mercury::years_during(&mercury_year), 1.0);
    /// ```
    fn years_during(d: &Duration) -> f64 {
        d.sec / Self::SECONDS_PER_EARTH_YR / Self::EARTH_YRS_PER_PLANET_YR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const EARTH_YRS_PER_PLANET_YR: f64 = 0.240_846_7;
}

impl Planet for Venus {
    const EARTH_YRS_PER_PLANET_YR: f64 = 0.615_197_26;
}

impl Planet for Earth {
    const EARTH_YRS_PER_PLANET_YR: f64 = 1.0;
}

impl Planet for Mars {
    const EARTH_YRS_PER_PLANET_YR: f64 = 1.880_815_8;
}

impl Planet for Jupiter {
    const EARTH_YRS_PER_PLANET_YR: f64 = 11.862_615;
}

impl Planet for Saturn {
    const EARTH_YRS_PER_PLANET_YR: f64 = 29.447_498;
}

impl Planet for Uranus {
    const EARTH_YRS_PER_PLANET_YR: f64 = 84.016_846;
}

impl Planet for Neptune {
    const EARTH_YRS_PER_PLANET_YR: f64 = 164.79132;
}
