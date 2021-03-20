export const answer = input => {
  return new Wordy(input).runToCompletion();
};

class Wordy {
  constructor(sequence) {
    this.instructions = Wordy.splitInstructions(sequence);
    this.next = "";
    this.buffer = "";
    this.state = new AwaitingOperation();
    this.op = undefined;
    this.value = 0;
  }

  static splitInstructions(sequence) {
    return sequence
      .split(" ")
      .map(instruction => instruction.split(/(\?)/))
      .flat();
  }

  parse() {
    this.next = this.instructions.shift();
  }

  step() {
    this.state.run(this);
    this.state.transition(this);
  }

  runToCompletion() {
    while (!(this.state instanceof Complete)) {
      this.step();
    }
    return this.value;
  }
}

class AwaitingOperation {
  static operations = {
    plus: (x, y) => x + y,
    minus: (x, y) => x - y,
    "multiplied by": (x, y) => x * y,
    "divided by": (x, y) => x / y,
    "What is": (x, y) => y,
    "?": undefined
  };

  static couldMatch(potentialOp) {
    return Object.keys(AwaitingOperation.operations).some(opName =>
      opName.startsWith(potentialOp)
    );
  }

  run(ctx) {
    ctx.parse();
    if (ctx.buffer === "") {
      ctx.buffer = ctx.next;
    } else {
      ctx.buffer += " " + ctx.next;
    }

    if (!isNaN(Number(ctx.buffer))) {
      throw "Syntax error";
    }

    if (!AwaitingOperation.couldMatch(ctx.buffer)) {
      throw "Unknown operation";
    }

    const op = AwaitingOperation.operations[ctx.buffer];
    if (op) {
      ctx.op = op;
    }
  }

  transition(ctx) {
    if (ctx.buffer == "?") {
      ctx.buffer = "";
      ctx.state = new Complete();
    } else if (ctx.op) {
      ctx.buffer = "";
      ctx.state = new AwaitingNumber();
    }
  }
}

class AwaitingNumber {
  run(ctx) {
    ctx.parse();
    const num = Number(ctx.next);

    if (isNaN(num)) {
      throw "Syntax error";
    } else {
      ctx.value = ctx.op(ctx.value, num);
    }
  }

  transition(ctx) {
    ctx.op = undefined;
    ctx.state = new AwaitingOperation();
  }
}

class Complete {}
