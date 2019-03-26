use std::collections::HashMap;

struct Sieve {
	composites: HashMap<u32,Vec<u32>>,
	current: u32,
}

impl Default for Sieve {
    fn default() -> Self {
        Sieve {
            composites: HashMap::new(),
            current: 2,
        }
    }
}

impl Sieve {
    pub fn new() -> Sieve {
        Default::default()
    }
}

impl Iterator for Sieve {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        fn next_prime(composites: &mut HashMap<u32, Vec<u32>>, x: u32) -> u32 {
            match composites.get(&x) {
                Some(numbers) => {
                    for num in numbers.to_owned() {
                        composites
                            .entry(x + num)
                            .and_modify(|v| v.push(num))
                            .or_insert_with(|| vec![num]);
                    }
                    composites.remove(&x);
                    next_prime(composites, x + 1)
                }
                None => {
                    composites.insert(x * x, vec![x]);
                    x
                }
            }
        }

        let prime = next_prime(&mut self.composites, self.current);
        self.current = prime + 1; // This number will be the next to be tested

        Some(prime)
    }
}
pub fn nth(n: u32) -> u32 {
	let mut sieve = Sieve::new();
	let mut result :u32 =sieve.next().unwrap_or(0);
	for _x in 0..n {
		result = sieve.next().unwrap_or(0)
	}
	result
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
