const RESP_TO_Q: &str = "Sure.";
const RESP_TO_YELL: &str = "Whoa, chill out!";
const RESP_TO_YELL_Q: &str = "Calm down, I know what I'm doing!";
const RESP_TO_NOTHING: &str = "Fine. Be that way!";
const RESP_TO_OTHER: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let msg: &str = message.trim();

    if msg.is_empty() {
        return RESP_TO_NOTHING;
    }

    match (msg.ends_with('?'), is_uppercase(msg)) {
        (true, true) => RESP_TO_YELL_Q,
        (true, false) => RESP_TO_Q,
        (false, true) => RESP_TO_YELL,
        (false, false) => RESP_TO_OTHER,
    }
}

fn is_uppercase(msg: &str) -> bool {
    // requires at least one uppercase letter
    // and no lowercase letters to be true
    msg.chars().any(char::is_uppercase) && !msg.chars().any(char::is_lowercase)
}
