// hamming.js

export const compute = (origStrand, compStrand) => {
  verifyArguments(origStrand, compStrand);

  return zip([...origStrand], [...compStrand])
    .map(([a,b]) => a != b)
    .reduce((acc, x) => acc + x, 0);
};

const verifyArguments = (origStrand, compStrand) => {
  if (origStrand == "" && compStrand != "") {
    throw "left strand must not be empty"
  }

  if (origStrand != "" && compStrand == "") {
    throw "right strand must not be empty"
  }

  if (origStrand.length != compStrand.length) {
    throw "left and right strands must be of equal length";
  }
}

const zip = (a, b) => a.map((x, i) => [x, b[i]]);
