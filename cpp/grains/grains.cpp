#include "grains.h"
#include <limits>

// function prototypes -------------------------------------------------------

namespace grains {
    unsigned long long total(unsigned int num_square);
} // namespace grains


// function definitions ------------------------------------------------------

namespace grains {
    unsigned long long square(unsigned int num_square)
    {
        return 1ULL << (num_square - 1);
    }

    unsigned long long total()
    {
        return total(64);
    }

    unsigned long long total(unsigned int num_square)
    {
        unsigned int shift = std::numeric_limits<unsigned long long>::digits - num_square;

        return (-1ULL << shift) >> shift;
    }
}  // namespace grains
