export function twoFer(otherName?: string): string {
  if (!otherName) {
    otherName = "you";
  }

  return `One for ${otherName}, one for me.`;
}
