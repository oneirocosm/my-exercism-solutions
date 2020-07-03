// pangram.js

const ALPHABET = "abcdefghijklmnopqrstuvwxyz".split("");

export const isPangram = (text = "") => {
  if (text == "") {
    return false;
  }

  return ALPHABET.every(letter => text.toLowerCase().includes(letter));
};
