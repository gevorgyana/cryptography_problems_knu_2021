use rand::Rng;
use gcd::Gcd;

fn sieve_of_eratosthenes(upto: usize) -> Vec<u128> {
    let mut primes = vec![true; upto as usize + 1];
    let mut cur_prime = 2;
    while cur_prime < upto {
	for j in (cur_prime * 2..upto).step_by(cur_prime) {
	    primes[j] = false;
	}
	cur_prime += 1;
	while primes[cur_prime] == false {
	    cur_prime += 1;
	}
    }
    let mut rv = vec![];
    for (i, v) in primes.iter().enumerate() {
	if i < 2 || i == primes.len() - 1 {
	    continue;
	}
	if primes[i] == true {
	    rv.push(i as u128);
	}
    }
    rv
}

fn gen_random_num(from: u128, to: u128) -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen_range(from..to + 1)
}

fn power_modulus(x: u128, y: u128, p: u128) -> u128 {
    let (mut x, mut y, mut p) = (x, y, p);
    x = x % p;
    if x == 0 {
	return 0;
    }
    let mut res = 1;
    while y > 0 {
	if y & 1 > 0 {
	    res = (res * x) % p;
	}
	y = y >> 1;
	x = (x * x) % p;
    }
    res
}

fn is_prime_using_pocklington_criteria(r: u128, n: u128) -> bool {
    loop {
	let a = gen_random_num(2, n - 1);
	if power_modulus(a, n - 1, n) != 1 {
	    return false;
	}
	let d = ((power_modulus(a, r, n) - 1) % n).gcd(n);
	if d != n {
	    if d == 1 {
		return true;
	    } else {
		return false;
	    }
	}
    }
}

fn grow_prime_number(initial_prime: u128, small_primes: &Vec<u128>) -> u128 {
    loop {
	let mut n: u128;
	let mut r: u128;
	loop {
	    r = gen_random_num(initial_prime, 2 * initial_prime + 1) * 2;
	    n = r * initial_prime + 1;
	    let mut n_is_prime = true;
	    for p in small_primes {
		if n % p == 0 {
		    n_is_prime = false;
		    break;
		}
	    }
	    if n_is_prime == true {
		break;
	    }
	}
	// let's prove that n is indeed prime now
	let n_is_prime = is_prime_using_pocklington_criteria(r, n);
	if n_is_prime == true {
	    return n;
	}
    }
}

fn main() {
    let small_primes = sieve_of_eratosthenes(1000);
    let initial_prime = *small_primes.last().unwrap();
    let initial_prime = grow_prime_number(initial_prime, &small_primes);
    // let initial_prime = grow_prime_number(initial_prime, &small_primes);
    println!("what is next? {}", grow_prime_number(initial_prime, &small_primes));
}
