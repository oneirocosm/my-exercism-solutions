// pangram.js

const ALPHABET = "abcdefghijklmnopqrstuvwxyz".split("");

export const isPangram = (text = "") => {
  return ALPHABET.every(letter => text.toLowerCase().includes(letter));
};
