class Squares {
  readonly squareOfSum: number;
  readonly sumOfSquares: number;
  readonly difference: number;

  constructor(maxNum: number) {
    this.squareOfSum = (maxNum ** 2 * (maxNum + 1) ** 2) / 4;
    this.sumOfSquares = (maxNum * (maxNum + 1) * (2 * maxNum + 1)) / 6;
    this.difference = this.squareOfSum - this.sumOfSquares;
  }
}

export default Squares;
