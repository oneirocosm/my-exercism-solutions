// rna-transcription.js

const dnaToRnaEncoding = {
  G: "C",
  C: "G",
  T: "A",
  A: "U"
};

export const toRna = (dnaString = "") => {
  const dna = [...dnaString];
  return dna.map(nucleotide => dnaToRnaEncoding[nucleotide]).join("");
};
