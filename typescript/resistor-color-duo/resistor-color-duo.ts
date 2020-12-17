enum ColorVal {
  black,
  brown,
  red,
  orange,
  yellow,
  green,
  blue,
  violet,
  grey,
  white
}

export class ResistorColor {
  private colors: string[];

  constructor(colors: string[]) {
    if (colors.length < 2) {
      throw Error("At least two colors need to be present");
    } else {
      this.colors = colors;
    }
  }
  value = (): number => {
    const mostSigVal = ColorVal[this.colors[0] as keyof typeof ColorVal];
    const leastSigVal = ColorVal[this.colors[1] as keyof typeof ColorVal];
    return mostSigVal * 10 + leastSigVal;
  };
}
