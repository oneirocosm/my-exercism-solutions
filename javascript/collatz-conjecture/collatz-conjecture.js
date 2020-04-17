export const steps = (n, previousNumberOfSteps = 0) => {
  if (n <= 0) {
    throw "Only positive numbers are allowed";
  }

  if (n == 1) {
    return previousNumberOfSteps;
  }

  const next = computeNextValue(n);
  return steps(next, previousNumberOfSteps + 1);
};

const computeNextValue = n => {
  if (isEven(n)) {
    return n / 2;
  } else {
    return 3 * n + 1;
  }
};

const isEven = n => {
  return n % 2 == 0;
};
