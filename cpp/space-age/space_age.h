#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

#include <stdint.h>

namespace space_age {
    class space_age {
        private:
        int64_t sec;
        double earth_years;
        public:
        explicit space_age(int64_t years);
        uint64_t seconds() const;
        double on_mercury() const;
        double on_venus() const;
        double on_earth() const;
        double on_mars() const;
        double on_jupiter() const;
        double on_saturn() const;
        double on_uranus() const;
        double on_neptune() const;
    };

}  // namespace space_age

#endif // SPACE_AGE_H
