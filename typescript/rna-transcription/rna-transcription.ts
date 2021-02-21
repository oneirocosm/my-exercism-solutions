class Transcriptor {
  static DNA_TO_RNA: { [key: string]: string } = {
    G: "C",
    C: "G",
    T: "A",
    A: "U"
  };

  toRna(dnaStrand: string): string {
    return Array.from(dnaStrand)
      .map(nucleotide => {
        const rna = Transcriptor.DNA_TO_RNA[nucleotide];
        if (rna === undefined) {
          throw "Invalid input DNA.";
        }
        return rna;
      })
      .join("");
  }
}

export default Transcriptor;
