pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
	let mut result = 0;
	for x in 1..limit{
		let mut processed = false;
		for f in factors{
			if *f != 0 && x%f==0 && !processed{
				result+=x;
				processed=true;
			}
		}
	}
	result
}
