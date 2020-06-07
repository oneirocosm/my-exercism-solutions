// pangram.js

const BASE_COUNT = {
  a: 0,
  b: 0,
  c: 0,
  d: 0,
  e: 0,
  f: 0,
  g: 0,
  h: 0,
  i: 0,
  j: 0,
  k: 0,
  l: 0,
  m: 0,
  n: 0,
  o: 0,
  p: 0,
  q: 0,
  r: 0,
  s: 0,
  t: 0,
  u: 0,
  v: 0,
  w: 0,
  x: 0,
  y: 0,
  z: 0
};

export const isPangram = (text = "") => {
  if (text == "") {
    return false;
  }

  const lc_text_array = [...text.toLowerCase()];
  const alphabet_count = lc_text_array.reduce(update_count, { ...BASE_COUNT });

  return Object.values(alphabet_count).every(letter_count => letter_count > 0);
};

const update_count = (alphabet_count, letter) => {
  if (alphabet_count[letter] !== undefined) {
    alphabet_count[letter] += 1;
  }
  return alphabet_count;
};
