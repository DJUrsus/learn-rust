use std::iter::range_inclusive;
use std::num::FromPrimitive;
use std::fmt::Show;

fn main() {
	for num in range_inclusive(1u, 100) {
		println!("{}", fizzbuzz(num))
	}
}

fn fizzbuzz<T: Int + FromPrimitive + Show>(num: T) -> String {
	let text = format!("{}{}",
	                   if num % u(3) == u(0) {"Fizz"} else {""},
	                   if num % u(5) == u(0) {"Buzz"} else {""});
	return if text.len() > 0 {text} else {format!("{}", num)};
}

fn u<T: FromPrimitive>(num: uint) -> T {
	FromPrimitive::from_uint(num).unwrap()
}
