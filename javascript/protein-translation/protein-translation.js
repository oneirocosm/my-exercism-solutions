const protein_decoding = {
  AUG: "Methionine",
  UUU: "Phenylalanine",
  UUC: "Phenylalanine",
  UUA: "Leucine",
  UUG: "Leucine",
  UCU: "Serine",
  UCC: "Serine",
  UCA: "Serine",
  UCG: "Serine",
  UAU: "Tyrosine",
  UAC: "Tyrosine",
  UGU: "Cysteine",
  UGC: "Cysteine",
  UGG: "Tryptophan"
};

const stop_codons = ["UAA", "UAG", "UGA"];

export const translate = (chain = "") => {
  const codons = parseRnaIntoCodons(chain);

  var proteins = [];
  for (const codon of codons) {
    if (stop_codons.includes(codon)) {
      break;
    }
    proteins.push(convertCodonToProtein(codon));
  }
  return proteins;
};

const parseRnaIntoCodons = (rna = "") => {
  return rna.match(/.{1,3}/g) || [];
};

const convertCodonToProtein = codon => {
  const protein = protein_decoding[codon];
  if (protein === undefined) {
    throw "Invalid codon";
  }
  return protein;
};
