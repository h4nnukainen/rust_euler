fn fib(mut a: u32, b: u32) {
  let limit: u32 = 128;
  if a < limit {
    println!("{}", a);
    a = a + b;
  }

  fib(b, a);
}

fn main() {
  fib(0, 1);
}
