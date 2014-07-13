use std::iter::range_inclusive;

fn main() {
	for num in range_inclusive(1i, 100) {
		println!("{}", fizzbuzz(num));
	}
}

fn fizzbuzz(num: int) -> String {
	let text = format!("{}{}",
	                   if num % 3 == 0 {"Fizz"} else {""},
	                   if num % 5 == 0 {"Buzz"} else {""});
	if text.len() > 0 {text} else {format!("{}", num)}
}
