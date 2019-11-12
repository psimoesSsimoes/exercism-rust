pub fn reply(message: &str) -> &str {
	let msg = message.trim_end();
	if msg.is_empty() {
		return "Fine. Be that way!"
	}

	let x = msg.to_ascii_lowercase();
	if msg == msg.to_uppercase() && msg.ends_with("?") && !(msg.to_ascii_uppercase() == x){
		return "Calm down, I know what I'm doing!"
	}
	if msg.ends_with("?"){
		return "Sure."
	}
	if msg == msg.to_uppercase() && !(msg.to_ascii_uppercase() == x)  {
		return "Whoa, chill out!"
	}

	"Whatever."
}

