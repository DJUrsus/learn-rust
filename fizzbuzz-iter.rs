struct FizzBuzzer {
	num: uint
}

impl Iterator<String> for FizzBuzzer {
	fn next(&mut self) -> Option<String> {
		self.num += 1;
		if self.num > 100 {None} else {
			let text = format!("{}{}",
			                   if self.num % 3 == 0 {"Fizz"} else {""},
			                   if self.num % 5 == 0 {"Buzz"} else {""});
			Some(if text.len() > 0 {text} else {format!("{}", self.num)})
		}
	}
}

// I don't like this name, and neither does Rust.
fn new_FizzBuzzer() -> FizzBuzzer { FizzBuzzer { num: 0 } }

fn main() {
	let mut i = new_FizzBuzzer();
	for fb in i {
		print!("{}\n", fb);
	}
}
