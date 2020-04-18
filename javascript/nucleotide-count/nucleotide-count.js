// nucleotide-count.js

export class NucleotideCounts {
  static parse(dnaStrand = "") {
    var count = {};
    for (const nucleotide of [...dnaStrand]) {
      this.incrementCount(count, nucleotide);
    }
    return this.toString(count);
  }

  static incrementCount(count, nucleotide) {
    if (!validNucleotides.includes(nucleotide)) {
      throw "Invalid nucleotide in strand";
    }

    if (count[nucleotide] === undefined) {
      count[nucleotide] = 1;
    } else {
      count[nucleotide] += 1;
    }
  }

  static toString(count) {
    return validNucleotides
      .map(nucleotide => (count[nucleotide] || 0).toString())
      .join(" ");
  }
}

const validNucleotides = ["A", "C", "G", "T"];
