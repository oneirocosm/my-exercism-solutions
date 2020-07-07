// bob.js

export const hey = message => {
  const trimmed_message = message.trim();

  const shouting =
    trimmed_message.split("").some(is_letter) &&
    trimmed_message == trimmed_message.toUpperCase();
  const question = trimmed_message.slice(-1) == "?";

  let response;
  if (shouting && question) {
    response = "Calm down, I know what I'm doing!";
  } else if (shouting) {
    response = "Whoa, chill out!";
  } else if (question) {
    response = "Sure.";
  } else if (trimmed_message.length == 0) {
    response = "Fine. Be that way!";
  } else {
    response = "Whatever.";
  }

  return response;
};

const is_letter = candidate => {
  return candidate.toUpperCase() != candidate.toLowerCase();
};
