fn main() {
	for num in range(1i, 101) {
		println!("{}", fizzbuzz(num))
	}
}

fn fizzbuzz(num: int) -> String {
	if num % 15 == 0 {
		format!("FizzBuzz")
	} else if num % 5 == 0 {
		format!("Buzz")
	} else if num % 3 == 0 {
		format!("Fizz")
	} else {
		format!("{}", num)
	}
}
