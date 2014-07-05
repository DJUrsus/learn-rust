fn main() {
	for num in range(1i, 101) {
		println!("{}", fizzbuzz(num))
	}
}

fn fizzbuzz(num: int) -> String {
	match (num % 3, num % 5) {
		(0, 0) => format!("FizzBuzz"),
		(0, _) => format!("Fizz"),
		(_, 0) => format!("Buzz"),
		(_, _) => format!("{}", num),
	}
}
