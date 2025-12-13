use std::cmp::min;

const SMALL_PRIMES: &[u64] = &[
	2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,
	101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,
	211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,
	307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,
	401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,
	503,509,521,523,541,547,557,563,569,571,577,587,593,599,
	601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,
	701,709,719,727,733,739,743,751,757,761,769,773,787,797,
	809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,
	907,911,919,929,937,941,947,953,967,971,977,983,991,997
];

const MR_BASES: &[u64] = &[
	2u64, 325u64, 9375u64, 28178u64, 450775u64, 9780504u64, 1795265022u64
];

#[inline(always)]
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
	((a as u128 * b as u128) % (m as u128)) as u64
}

#[inline(always)]
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
	let mut res: u64 = 1 % m;
	base %= m;
	while exp > 0 {
		if (exp & 1) != 0 {
			res = mod_mul(res, base, m);
		}
		base = mod_mul(base, base, m);
		exp >>= 1;
	}
	res
}

fn is_prime(n: u64) -> bool {
	if n < 2 {
		return false;
	}
	for &p in SMALL_PRIMES {
		if n == p {
			return true;
		}
		if n % p == 0 {
			return false;
		}
	}
	let mut d = n - 1;
	let mut s = 0u32;
	while (d & 1) == 0 {
		d >>= 1;
		s += 1;
	}

	'outer: for &a in MR_BASES {
		if a % n == 0 {
			continue;
		}
		let mut x = mod_pow(a % n, d, n);
		if x == 1 || x == n - 1 {
			continue;
		}
		for _ in 1..s {
			x = mod_mul(x, x, n);
			if x == n - 1 {
				continue 'outer;
			}
		}
		return false;
	}

	true
}

#[derive(Clone)]
struct SplitMix64 {
	state: u64,
}

impl SplitMix64 {
	#[inline(always)]
	fn new(seed: u64) -> Self { Self { state: seed } }

	#[inline(always)]
	fn next_u64(&mut self) -> u64 {
		let mut z = self.state.wrapping_add(0x9E3779B97f4A7C15);
		self.state = z;
		z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
		z ^ (z >> 31)
	}

	#[inline(always)]
	fn gen_range_1_to_n_minus_1(&mut self, n: u64) -> u64 {
		if n <= 2 { return 1; }
		(self.next_u64() % (n - 1)) + 1
	}
}

fn pollard_brent(n: u64, rng: &mut SplitMix64) -> u64 {
	if n % 2 == 0 { return 2; }
	if n % 3 == 0 { return 3; }

	let mut y = rng.gen_range_1_to_n_minus_1(n);
	let c = rng.gen_range_1_to_n_minus_1(n);
	let m: u64 = 128;
	let mut g: u64 = 1;
	let mut r: u64 = 1;
	let mut q: u64 = 1;
	let mut x: u64 = 0;
	let mut ys: u64 = 0;

	let f = |x: u64| -> u64 {
		(mod_mul(x, x, n) + c) % n
	};

	while g == 1 {
		x = y;
		for _ in 0..r {
			y = f(y);
		}

		let mut k = 0u64;
		while k < r && g == 1 {
			ys = y;
			let limit = min(m, r - k);
			for _ in 0..limit {
				y = f(y);
				let diff = if x > y { x - y } else { y - x };
				if diff != 0 {
					q = mod_mul(q, diff, n);
				}
			}
			g = gcd(u128_to_u64(q as u128), n);
			k = k.wrapping_add(limit);
		}
		r = r.wrapping_mul(2);
	}

	if g == n {
		loop {
			ys = f(ys);
			let diff = if x > ys { x - ys } else { ys - x };
			g = gcd(diff, n);
			if g > 1 {
				break;
			}
		}
	}
	g
}

#[inline(always)]
fn u128_to_u64(x: u128) -> u64 { (x % (u64::MAX as u128 + 1)) as u64 }

#[inline(always)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		let t = a % b;
		a = b;
		b = t;
	}
	a
}

pub fn factor(mut n: u64) -> Vec<u64> {
	if n <= 1 {
		return Vec::new();
	}

	let mut res: Vec<u64> = Vec::new();

	for &p in SMALL_PRIMES {
		if (p as u128) * (p as u128) > n as u128 {
			break;
		}
		while n % p == 0 {
			res.push(p);
			n /= p;
		}
	}

	if n == 1 {
		res.sort_unstable();
		return res;
	}

	let mut stack = vec![n];
	let mut rng = SplitMix64::new(0x9E3779B97f4A7C15_u64 ^ n);

	while let Some(m) = stack.pop() {
		if m == 1 {
			continue;
		}
		if is_prime(m) {
			res.push(m);
			continue;
		}
		let mut d = pollard_brent(m, &mut rng);
		let mut tries = 0;
		while d == 0 || d == 1 || d == m {
			rng.next_u64();
			d = pollard_brent(m, &mut rng);
			tries += 1;
			if tries > 8 {
				let mut candidate = 3u64;
				while candidate * candidate <= m {
					if m % candidate == 0 {
						d = candidate;
						break;
					}
					candidate += 2;
				}
				if d == m || d == 0 {
					res.push(m);
					d = 1;
					break;
				}
			}
		}
		if d == 1 {
			continue;
		}
		let other = m / d;
		stack.push(d.max(other));
		stack.push(d.min(other));
	}
	res.sort_unstable();
	res
}