fn fib(mut a: i32, b: i32, mut sum: i32) {
  let limit = 4000000;
  if a <= limit {
    if a % 2 == 0 {
      sum += a;
    }
    a = a + b;
    fib(b, a, sum);
  } else {
    println!("The answer is: {}", sum);
  }
}

fn main() {
  fib(1, 2, 0);

  let limit = 4000000;
  let (mut sum, mut term, mut next) = (0, 1, 2);

  while term <= limit {
    let b = next;
    if term % 2 == 0 {
      sum += term;
    }
    next = term + next;
    term = b;
  }
  println!("The answer is: {}", sum);
}
