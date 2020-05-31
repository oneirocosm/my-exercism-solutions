#include "raindrops.h"

// function prototypes -------------------------------------------------------

bool divisible_by(int n, int divisor);


// function definitions ------------------------------------------------------

namespace raindrops {
    std::string convert(int n)
    {
        std::string sound = "";

        if (divisible_by(n, 3)) {
            sound.append("Pling");
        }

        if (divisible_by(n, 5)) {
            sound.append("Plang");
        }

        if (divisible_by(n, 7)) {
            sound.append("Plong");
        }

        if (sound.empty()) {
            sound = std::to_string(n);
        }

        return sound;
    }
}  // namespace raindrops

bool divisible_by(int n, int divisor)
{
    return n % divisor == 0;
}
