// pangram.js

export const isPangram = (text = "") => {
  if (text == "") {
    return false;
  }

  let alphabet_count = {
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

  for (const letter of text.toLowerCase()) {
    const old_count = alphabet_count[letter];
    if (old_count !== undefined) {
      alphabet_count[letter] += 1;
    }
  }

  return Object.values(alphabet_count).every(letter_count => letter_count > 0);
};
