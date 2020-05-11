#include "space_age.h"

namespace space_age {
    space_age::space_age(int64_t input_in_secs) {
        sec = input_in_secs;
    }

    uint64_t space_age::seconds() const {
        return sec;
    }

    double space_age::on_mercury() const {
        const double earth_yrs_per_mercury_yr = 0.2408467;
        return on_earth() / earth_yrs_per_mercury_yr;
    }

    double space_age::on_venus() const {
        const double earth_yrs_per_venus_yr = 0.61519726;
        return on_earth() / earth_yrs_per_venus_yr;
    }

    double space_age::on_earth() const {
        const double secs_per_earth_year = 31557600.0;
        return sec / secs_per_earth_year;
    }

    double space_age::on_mars() const {
        const double earth_yrs_per_mars_yr = 1.8808158;
        return on_earth() / earth_yrs_per_mars_yr;
    }

    double space_age::on_jupiter() const {
        const double earth_yrs_per_jupiter_yr = 11.862615;
        return on_earth() / earth_yrs_per_jupiter_yr;
    }

    double space_age::on_saturn() const {
        const double earth_yrs_per_saturn_yr = 29.447498;
        return on_earth() / earth_yrs_per_saturn_yr;
    }

    double space_age::on_uranus() const {
        const double earth_yrs_per_uranus_yr = 84.016846;
        return on_earth() / earth_yrs_per_uranus_yr;
    }

    double space_age::on_neptune() const {
        const double earth_yrs_per_neptune_yr = 164.79132;
        return on_earth() / earth_yrs_per_neptune_yr;
    }

}  // namespace space_age
