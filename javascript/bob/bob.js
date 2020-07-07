// bob.js

export const hey = message => {
  const trimmed_message = message.trim();

  const shouting =
    trimmed_message.split("").some(is_letter) &&
    trimmed_message == trimmed_message.toUpperCase();
  const question = trimmed_message.endsWith("?");

  let response;
  if (shouting && question) {
    return "Calm down, I know what I'm doing!";
  } else if (shouting) {
    return "Whoa, chill out!";
  } else if (question) {
    return "Sure.";
  } else if (trimmed_message.length == 0) {
    return "Fine. Be that way!";
  }

  return "Whatever.";
};

const is_letter = candidate => {
  return candidate.toUpperCase() != candidate.toLowerCase();
};
