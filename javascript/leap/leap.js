export const isLeap = year => {
  var isLeap;
  if (divisible(year, 400)) {
    isLeap = true;
  } else if (divisible(year, 100)) {
    isLeap = false;
  } else if (divisible(year, 4)) {
    isLeap = true;
  } else {
    isLeap = false;
  }
  return isLeap;
};

const divisible = (dividend, divisor) => {
  return dividend % divisor == 0;
};
