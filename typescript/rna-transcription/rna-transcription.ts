export type Opaque<K, T> = T & { __TYPE__: K };

type RnaValid = Opaque<string, "RnaValid">;
type DnaValid = Opaque<string, "RnaValid">;

function createRnaValid(str: string): RnaValid {
  const RNA_LETTERS = new Set<string>(["C", "G", "A", "U"]);

  if (str.split("").some(ch => !RNA_LETTERS.has(ch))) {
    throw new Error("Invalid output RNA.");
  }

  return str.toString() as RnaValid;
}

function createDnaValid(str: string): DnaValid {
  const DNA_LETTERS = new Set<string>(["C", "G", "A", "T"]);

  if (str.split("").some(ch => !DNA_LETTERS.has(ch))) {
    throw new Error("Invalid input DNA.");
  }

  return str.toString() as DnaValid;
}

class Transcriptor {
  static DNA_TO_RNA: { [key: string]: string } = {
    G: "C",
    C: "G",
    T: "A",
    A: "U"
  };

  toRna(dnaStrand: string): RnaValid {
    let dnaStrandChecked: DnaValid = createDnaValid(dnaStrand);
    let rnaStrand = Array.from(dnaStrandChecked).reduce(
      (acc_strand, nucleotide) => {
        const rna = Transcriptor.DNA_TO_RNA[nucleotide];
        return acc_strand + rna;
      },
      ""
    );

    return createRnaValid(rnaStrand);
  }
}

export default Transcriptor;
