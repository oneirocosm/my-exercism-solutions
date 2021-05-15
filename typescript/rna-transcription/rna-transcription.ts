const RNA_LETTERS = ["C", "G", "A", "U"];
const DNA_LETTERS = ["C", "G", "A", "U"];
export type RnaValid = typeof RNA_LETTERS[number];
export type DnaValid = typeof DNA_LETTERS[number];

class Transcriptor {
  static DNA_TO_RNA: { [key: string]: string } = {
    G: "C",
    C: "G",
    T: "A",
    A: "U"
  };

  toRna(dnaStrand: DnaValid): RnaValid {
    return Array.from(dnaStrand).reduce((acc_strand, nucleotide) => {
      const rna: RnaValid | undefined = Transcriptor.DNA_TO_RNA[nucleotide];
      if (rna === undefined) {
        throw "Invalid input DNA.";
      }
      return acc_strand + rna;
    }, "");
  }
}

export default Transcriptor;
