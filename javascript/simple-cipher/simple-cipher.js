export class Cipher {
  constructor(key = "") {
    if (!key) {
      const ALPHABET = "abcdefghijklmnopqrstuvwxyz";
      for (let i = 0; i < 10; i++) {
        key += ALPHABET.charAt(Math.floor(Math.random() * ALPHABET.length));
      }
    }
    this.key = key.toLowerCase();
  }

  encode(message) {
    const offsets = keyValue(this.key);
    return message.split("").reduce((acc, character) => {
      const shift = offsets.next().value;
      const newCharacter = translateChar(character, shift);
      return acc + newCharacter;
    }, "");
  }

  decode(message) {
    const offsets = keyValue(this.key);
    return message.split("").reduce((acc, character) => {
      const shift = 26 - offsets.next().value;
      const newCharacter = translateChar(character, shift);
      return acc + newCharacter;
    }, "");
  }
}

function* keyValue(key) {
  const chars = key.split("");
  const length = chars.length;
  const ordBase = "a".charCodeAt(0);
  let i = 0;
  while (true) {
    const ord = chars[i].charCodeAt(0) - ordBase;
    yield ord;
    i = (i + 1) % length;
  }
}

function translateChar(character, shift) {
  if (!isLetter) {
    return character;
  }

  const ordLetter = isLowerCase(character) ? "a" : "A";
  const ordBase = ordLetter.charCodeAt(0);
  const ord = character.charCodeAt(0) - ordBase;
  const ordShift = (ord + shift) % 26;
  return String.fromCharCode(ordBase + ordShift);
}

function isLetter(string) {
  return string.toLowerCase() != string.toUpperCase();
}

function isLowerCase(string) {
  return isLetter(string) && string == string.toLowerCase();
}
