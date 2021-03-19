export class Cipher {
  constructor(key = "") {
    if (!key) {
      key = Cipher.randomKey();
    }
    this.key = key.toLowerCase();
  }

  encode(message) {
    const offsets = this.keyOffsetGenerator();
    return message
      .split("")
      .map(character => {
        const shift = offsets.next().value;
        return Cipher.translateChar(character, shift);
      })
      .join("");
  }

  decode(message) {
    const offsets = this.keyOffsetGenerator();
    return message
      .split("")
      .map(character => {
        const shift = 26 - offsets.next().value;
        return Cipher.translateChar(character, shift);
      })
      .join("");
  }

  *keyOffsetGenerator() {
    const length = this.key.length;
    const ordBase = "a".charCodeAt(0);
    let i = 0;
    while (true) {
      const ord = this.key[i].charCodeAt(0) - ordBase;
      yield ord;
      i = (i + 1) % length;
    }
  }

  static randomKey() {
    const ALPHABET = "abcdefghijklmnopqrstuvwxyz";
    let key = "";

    for (let i = 0; i < 10; i++) {
      key += ALPHABET.charAt(Math.floor(Math.random() * ALPHABET.length));
    }
    return key;
  }

  static translateChar(character, shift) {
    if (!Cipher.isLetter(character)) {
      return character;
    }

    const baseLetter = Cipher.isLowerCase(character) ? "a" : "A";
    const baseOrd = baseLetter.charCodeAt(0);
    const relativeInputOrd = character.charCodeAt(0) - baseOrd;
    const relativeShiftOrd = (relativeInputOrd + shift) % 26;
    return String.fromCharCode(baseOrd + relativeShiftOrd);
  }

  static isLetter(string) {
    return string.toLowerCase() != string.toUpperCase();
  }

  static isLowerCase(string) {
    return Cipher.isLetter(string) && string == string.toLowerCase();
  }
}
