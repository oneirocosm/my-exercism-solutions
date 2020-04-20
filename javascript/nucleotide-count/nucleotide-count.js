// nucleotide-count.js

export class NucleotideCounts {
  static parse(dnaStrand = "") {
    var nucleotideCount = { A: 0, C: 0, G: 0, T: 0 };
    return this.toString(
      [...dnaStrand].reduce(this.incrementCount, nucleotideCount)
    );
  }

  static incrementCount(nucleotideCount, nucleotide) {
    if (!Object.keys(nucleotideCount).includes(nucleotide)) {
      throw "Invalid nucleotide in strand";
    }

    nucleotideCount[nucleotide]++;
    return nucleotideCount;
  }

  static toString(nucleotideCount) {
    return Object.values(nucleotideCount).join(" ");
  }
}
