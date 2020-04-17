const romanNumeralBasis = {
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

export const toRoman = decimalNumber => {
  var romanRepr = "";
  for (const romanNumeralBasisMember in romanNumeralBasis) {
    const basisValue = romanNumeralBasis[romanNumeralBasisMember];
    const multiplesOfBasisMember = Math.trunc(decimalNumber / basisValue);
    decimalNumber %= basisValue;
    romanRepr += romanNumeralBasisMember.repeat(multiplesOfBasisMember);
  }
  return romanRepr;
};
