extern crate regex;

use regex::Regex;

pub fn reply(message: &str) -> &str {
	  let shouting = Regex::new(r"[[:alpha:]]")
		  .unwrap()
		  .is_match(message)&&
		  message == message.to_uppercase();

	  let question = Regex::new(r"\?\s*\z")
		  .unwrap()
		  .is_match(message);

	  let silence  = !Regex::new(r"\S")
		  .unwrap()
		  .is_match(message);

    match (shouting, question, silence) {
        (_,    _,    true) => &"Fine. Be that way!",
        (true, true, _   ) => &"Calm down, I know what I'm doing!",
        (_,    true, _   ) => &"Sure.",
        (true, _,    _   ) => &"Whoa, chill out!",
        _                  => &"Whatever.",
    }
}
