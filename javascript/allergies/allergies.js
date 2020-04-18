// allergies.js

export class Allergies {
  constructor(allergyCode) {
    this.code = allergyCode;
  }

  list() {
    var allergens = [];
    for (const allergen in AllergyBitmasks) {
      if (this.allergicTo(allergen)) {
        allergens.push(allergen);
      }
    }
    return allergens;
  }

  allergicTo(allergen) {
    return !!(this.code & AllergyBitmasks[allergen]);
  }
}

const AllergyBitmasks = {
  eggs: 1,
  peanuts: 2,
  shellfish: 4,
  strawberries: 8,
  tomatoes: 16,
  chocolate: 32,
  pollen: 64,
  cats: 128
};
