pub fn is_armstrong_number(num: u32) -> bool {
	let mut count = 0;
	let mut digits = vec![];
	let mut x = num;
	while x !=0 {
		count+=1;
		digits.push(x % 10);
		x/=10
	}

	num == digits.iter().
		fold(0, |_acc, x| _acc + x.pow(count))
}
