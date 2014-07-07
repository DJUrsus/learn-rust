use std::iter::range_inclusive;

fn main() {
	for num in range_inclusive(1i, 100) {
		println!("{}\t{}", fizzbuzz_match(num), fizzbuzz_concat(num))
	}
}

fn fizzbuzz_match(num: int) -> String {
	match (num % 3, num % 5) {
		(0, 0) => format!("FizzBuzz"),
		(0, _) => format!("Fizz"),
		(_, 0) => format!("Buzz"),
		(_, _) => format!("{}", num),
	}
}

fn fizzbuzz_concat(num: int) -> String {
	let text = format!("{}{}",
	                   if num % 3 == 0 {"Fizz"} else {""},
	                   if num % 5 == 0 {"Buzz"} else {""});
	if text.len() > 0 {text} else {format!("{}", num)}
}
