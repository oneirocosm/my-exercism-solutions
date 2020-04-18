// etl.js

export const transform = legacyMapping => {
  var newMapping = {};
  for (const [score, letters] of Object.entries(legacyMapping)) {
    updateMapping(newMapping, score, letters);
  }

  return newMapping;
};

const updateMapping = (mapping, score, letters) => {
  for (const letter of letters) {
    mapping[letter.toLowerCase()] = parseInt(score);
  }
};
