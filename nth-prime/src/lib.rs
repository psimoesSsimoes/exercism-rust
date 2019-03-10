pub fn nth(n: u32) -> u32 {


}

fn sieve_of_erasosthenes(number: u32)-> Vec<u32>{
	let mut vec = Vec::new();
	let mut np = Vec<bool>::new();
	vec.push(2);
	vec.push(3);
	for i in 3..number {
		if !boolvec[i]{
			vec.push(i);
			let mut y = i*i;
			while y < number{
				np[y] = true;
				y+=1;
			}
		}
		i+=2;
	}
	vec
}

// func sieveOfEratosthenes(m int) []int {
// 	p := []int{2}
// 	np := make([]bool, m)
// 	for i := 3; i < m; i += 2 {
// 		if !np[i] {
// 			p = append(p, i)
// 			for y := i * i; y < m; y += i {
// 				np[y] = true
// 			}
// 		}
// 	}
// 	return p
// }

// static bool IsPrime(int number) {
//     for (int i = 2; i < number; i++) {
//         if (number % i == 0 && i != number) return false;
//     }
//     return true;
// }
