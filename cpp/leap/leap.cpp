#include "leap.h"

// static function declarations
static bool divisible_by(int32_t year, int32_t divisor);

// function definitions
namespace leap {
    bool is_leap_year(int32_t year) {
        return divisible_by(year, 400) || (divisible_by(year, 4) && !divisible_by(year, 100));
    }
}  // namespace leap

bool divisible_by(int32_t year, int32_t divisor)
{
    return year % divisor == 0;
}
