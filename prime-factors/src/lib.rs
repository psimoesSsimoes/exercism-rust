pub fn factors(n: u64) -> Vec<u64> {
	let mut x = n;
	let mut result =vec![];
	while x %2 == 0{
		result.push(2);
		x= x / 2;
	}

	let mut f = 3;
	while f * f <= n{
		if x % f == 0 {
			result.push(f);
			x/= f;
		}else{
			f+=2;
		}
	}

	if x != 1 {
		result.push(n);
	}
	result
}
