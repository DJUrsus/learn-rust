use std::iter::range_inclusive;

fn main() {
	for num in range_inclusive(1i, 100) {
		let text = format!("{}{}",
		                   if num % 3 == 0 {"Fizz"} else {""},
		                   if num % 5 == 0 {"Buzz"} else {""});
		if text.len() > 0 {
			println!("{}", text);
		} else {
			println!("{}", num);
		}
	}
}
