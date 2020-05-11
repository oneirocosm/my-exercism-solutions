#include "leap.h"

namespace leap {
    bool is_leap_year(uint32_t year) {
        bool meets_leap_criteria;

        if ((year % 400) == 0) {
            meets_leap_criteria = true;
        } else if ((year % 100) == 0) {
            meets_leap_criteria = false;
        } else if ((year % 4) == 0) {
            meets_leap_criteria = true;
        } else {
            meets_leap_criteria = false;
        }

        return meets_leap_criteria;
    }
}  // namespace leap
