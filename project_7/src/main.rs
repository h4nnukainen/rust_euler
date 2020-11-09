fn sieve() {
  let mut primes: Vec<i64> = Vec::new();
  let mut done = false;
  let mut i = 2;
  primes.push(i);

  while !done {
    let mut skip = false;
    for p in primes.clone() {
      if i % p == 0 {
        skip = true;
        break;
      }
    }
    if !skip {
      primes.push(i);
    }

    if primes.len() == 10001 {
      println!("\n\n10 001st prime number is {}", i);
      done = true;
    }

    i += 1;
  }
}

fn another() {
  let primes: [i32; 4] = [2, 3, 5, 7];
  let mut i = 2;
  let mut prime_amount = primes.len();
  let mut done = false;

  while !done {
    let mut skip = false;
    for p in &primes {
      if i % p == 0 {
        skip = true;
        break;
      }
    }
    if !skip {
      prime_amount += 1;
    }

    if prime_amount == 10001 {
      println!("\n\n{}st prime number is {}", prime_amount, i);
      done = true;
    }

    i += 1;
  }
}

fn main() {
  sieve();
  another();
}
