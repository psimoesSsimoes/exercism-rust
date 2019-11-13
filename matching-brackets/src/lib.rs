const B : char = '[';
const F : char ='{';
const D : char ='(';
const BC : char =']';
const FC : char ='}';
const DD : char =')';


pub fn brackets_are_balanced(string: &str) -> bool {
	let mut control = vec![];

	for c in string.chars(){
		if c == B || c == F || c == D{
			control.push(c);
		}

		if c == BC || c == FC || c == DD{
			let x = control.pop();
			if x.is_none(){
				return false
			}

			if !closes(x.unwrap(),c){
				return false
			}
		}
	}

	if control.len()!=0{
		return false
	}

	true
}

fn closes(open: char, close :char) -> bool{
	if open == B{
		return close == BC
	}

	if open == F{
		return close == FC
	}

	if open == D{
		return close == DD
	}

	false
}
