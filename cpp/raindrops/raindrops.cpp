#include "raindrops.h"


// function definitions ------------------------------------------------------

namespace raindrops {
    std::string convert(int n)
    {
        std::string sound = "";

        if (n % 3 == 0) {
            sound.append("Pling");
        }

        if (n % 5 == 0) {
            sound.append("Plang");
        }

        if (n % 7 == 0) {
            sound.append("Plong");
        }

        if (sound.empty()) {
            sound = std::to_string(n);
        }

        return sound;
    }
}  // namespace raindrops
