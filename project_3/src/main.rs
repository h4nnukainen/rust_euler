fn sieve() {
  let mut primes: Vec<i64> = Vec::new();
  for i in 2..100 {
    if i % i == 0 {
      let mut skip = false;
      if primes.len() == 0 {
        primes.push(i);
        print!("{} ", i);
      }
      for p in primes.clone() {
        if i % p == 0 {
          skip = true;
          break;
        }
      }
      if !skip {
        primes.push(i);
        print!("{} ", i);
      }
    }
  }
}

fn main() {
  sieve();

  // let mut n: u64 = 600851475143;
  // let mut div: u64 = 2;
  // let mut max_fact = 0;

  // while n != 0 {
  //   println!("{}", n);
  //   if n % div != 0 {
  //     div = div + 1;
  //   } else {
  //     max_fact = n;
  //     n = n / div;
  //     if n == 1 {
  //       println!("Isoin alkuluku on {}", max_fact);
  //       break;
  //     }
  //   }
  // }
}
