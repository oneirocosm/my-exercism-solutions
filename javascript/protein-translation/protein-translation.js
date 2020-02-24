var decoding = {
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
  UGG: "Tryptophan",
  UAA: "STOP",
  UAG: "STOP",
  UGA: "STOP"
};

export const translate = (chain = "") => {
  // split into chunks of 3
  var codons = chain.match(/.{1,3}/g) || [];

  // convert codons
  var proteins = [];
  for (var i = 0; i < codons.length; i++) {
    var codon = codons[i];

    if (decoding[codon] === undefined) {
      throw "Invalid codon";
    } else {
      proteins.push(decoding[codon]);
    }
  }

  // truncate if stop appears
  var stop_point = proteins.indexOf("STOP");
  if (stop_point != -1) {
    proteins.length = stop_point;
  }
  return proteins;
};
