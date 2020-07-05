#include <ctype.h>
#include <unordered_set>
#include "pangram.h"

namespace pangram {

    bool is_pangram(std::string const &input) {

        std::unordered_set<char> letters_in_input;
        for (char const letter: input) {
            if (isalpha(letter)) {
                letters_in_input.insert(tolower(letter));
            }
        }
        return letters_in_input.size() == 26;
    }

}  // namespace pangram
