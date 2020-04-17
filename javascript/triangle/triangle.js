export class Triangle {
  constructor(a, b, c) {
    this.sides = [a, b, c];
  }

  isEquilateral() {
    return this.numberOfEqualSidePairs() == 3 && this.isValidTriangle();
  }

  isIsosceles() {
    return this.numberOfEqualSidePairs() >= 1 && this.isValidTriangle();
  }

  isScalene() {
    return this.numberOfEqualSidePairs() == 0 && this.isValidTriangle();
  }

  isValidTriangle() {
    return this.sideLengthsValid() && this.triangleInequalityValid();
  }

  sideLengthsValid() {
    return this.sides.every(side => side >= 0);
  }

  triangleInequalityValid() {
    return 2 * this.longestSide() < this.perimeter();
  }

  longestSide() {
    return Math.max(...this.sides);
  }

  perimeter() {
    return this.sides.reduce((acc, x) => acc + x, 0);
  }

  numberOfEqualSidePairs() {
    const [a, b, c] = this.sides;
    const diffBetweenFirstTwoSides = Math.abs(a - b);
    const diffBetweenSecondTwoSides = Math.abs(b - c);
    const diffBetweenThirdTwoSides = Math.abs(c - a);

    var numberOfEqualSidePairs = 0;
    numberOfEqualSidePairs += diffBetweenFirstTwoSides == 0;
    numberOfEqualSidePairs += diffBetweenSecondTwoSides == 0;
    numberOfEqualSidePairs += diffBetweenThirdTwoSides == 0;
    return numberOfEqualSidePairs;
  }
}
