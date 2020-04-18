// raindrops.js

export const convert = number => {
  var sound = "";

  if (isDivisible(number, 3)) {
    sound += "Pling";
  }

  if (isDivisible(number, 5)) {
    sound += "Plang";
  }

  if (isDivisible(number, 7)) {
    sound += "Plong";
  }

  if (sound == "") {
    sound = number.toString();
  }

  return sound;
};

const isDivisible = (number, divisor) => number % divisor == 0;
