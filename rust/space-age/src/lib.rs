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
    ///
    /// let earth_year = Duration::from(31_557_600);
    /// assert_float_absolute_eq!(Earth::years_during(&earth_year), 1.0);
    ///
    /// let mercury_year = Duration::from(7_600_544);
    /// assert_float_absolute_eq!(Mercury::years_during(&mercury_year), 1.0);
    /// ```
    fn years_during(d: &Duration) -> f64 {
        let earth_years = d.sec / Self::SECONDS_PER_EARTH_YR;
        let planet_years =  earth_years / Self::EARTH_YRS_PER_PLANET_YR;
        planet_years
    }
}

/// Creates a struct that implements the Planet trait and assigns
/// an associated constant named EARTH_YRS_PER_PLANET_YR
///
/// # Arguments
///
/// * `name` - The name of the struct
/// * `earth_yrs` - The number of earth years in a planet year
macro_rules! new_planet {
    ($name:ident, $earth_yrs:expr) => {
        pub struct $name;
        impl Planet for $name {
            const EARTH_YRS_PER_PLANET_YR: f64 = $earth_yrs;
        }
    }
}

new_planet!(Mercury, 0.240_846_700);
new_planet!(Venus, 0.615_197_260);
new_planet!(Earth, 1.0);
new_planet!(Mars, 1.880_815_800);
new_planet!(Jupiter, 11.862_615);
new_planet!(Saturn, 29.447_498);
new_planet!(Uranus, 84.016_846);
new_planet!(Neptune, 164.791_320);
