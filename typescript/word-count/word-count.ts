class Words {
  count(input: string): Map<string, number> {
    const words: string[] = input
      .trim()
      .toLowerCase()
      .split(/\s+/);
    let counter: Map<string, number> = new Map();
    for (const word of words) {
      const oldCount = counter.get(word) || 0;
      counter.set(word, oldCount + 1);
    }

    return counter;
  }
}

export default Words;
