#include <ctype.h>
#include <unordered_set>
#include "pangram.h"

namespace pangram {
    std::unordered_set<char> const alphabet = {'a', 'b', 'c', 'd', 'e', 'f',
        'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z'};

    bool is_pangram(std::string const &input) {

        std::unordered_set<char> letters_in_string;
        for (char const &letter: input) {
            if (isalpha(letter)) {
                letters_in_string.insert(tolower(letter));
            }
        }
        return alphabet == letters_in_string;
    }

}  // namespace pangram
