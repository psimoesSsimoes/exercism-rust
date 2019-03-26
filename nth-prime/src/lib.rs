// Credit to christophefarges's
// his solution is really fast
pub fn nth(n: u32) -> u32 {

    let mut count = 0;
    let mut current = 1;

    while count <= n {
        current +=1;
        if is_prime(current){
            count +=1;
        }
    }
    current
}

fn is_prime(n: u32) -> bool {
    // Algorithm taken from https://en.wikipedia.org/wiki/Primality_test
    if n <= 3 {
        return n > 1
    }
    if n % 2 == 0  || n % 3 == 0 {
        return false
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i+2) == 0{
            return false
        }
        i += 6;
    }
    true
}
