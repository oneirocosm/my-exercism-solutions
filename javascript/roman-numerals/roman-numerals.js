// These numbers aren't prime per se, but they are
// the basic components to building any roman numeral
const primeNumeralToValue = {
  M: 1000,
  CM: 900,
  D: 500,
  CD: 400,
  C: 100,
  XC: 90,
  L: 50,
  XL: 40,
  X: 10,
  IX: 9,
  V: 5,
  IV: 4,
  I: 1
};

export const toRoman = remainder => {
  var romanRepr = "";
  for (const primeNumeral in primeNumeralToValue) {
    const primeValue = primeNumeralToValue[primeNumeral];
    const multiplesOfPrime = Math.trunc(remainder / primeValue);
    remainder %= primeValue;
    romanRepr += primeNumeral.repeat(multiplesOfPrime);
  }
  return romanRepr;
};
