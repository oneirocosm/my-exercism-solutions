let genex = require("genex");
let shuffle = require("shuffle-array");

export class Robot {
  #name;
  static #namesInUse = new Set();
  static #namesFree = shuffle(genex(/[A-Z]{2}\d{1,3}/).generate());

  static releaseNames() {
    return Robot.#namesInUse;
  }

  constructor() {
    this.reset();
  }

  reset() {
    let name = Robot.#namesFree.pop();
    Robot.#namesInUse.add(name);
    this.#name = name;
  }

  get name() {
    return this.#name;
  }
}
