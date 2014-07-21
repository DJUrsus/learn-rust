
use std::os;
use std::vec;
use std::fmt;

#[deriving(Clone, Show)]
enum BaggageSlot {
	A, // baggage bin, destined for city A
	B, // baggage bin, destined for city B
	X, // empty slot
}

// impl Clone for BaggageSlot {
// 	// TODO: Should this be clone self.a?
// 	fn clone(&self) -> BaggageSlot { *self }
// }

// impl fmt::Show for BaggageSlot {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
// 		match self {&A => write!(f, "A"),
// 		            &B => write!(f, "B"),
// 		            &X => write!(f, "_")}
// 	}
// }

struct BaggageArea {
	a: Vec<BaggageSlot>,
}

impl BaggageArea {
	fn new(num: uint) -> BaggageArea {
		BaggageArea{
			a: vec::Vec::from_fn(num * 4, |i| {if      i < num * 2 { X }
			                                   else if i % 2 == 1  { A }
			                                   else                { B }
			})
		}
	}

	fn move(&mut self, from: uint, to: uint) -> Result<(), String> {
		let ref mut area = self.a;
		match (area[to], area[to + 1]) {
			(X, X) => {
				let left = area[from];
				let right = area[from + 1];
				let source_err = Err(format!("a source space is unoccupied ({} to {})", from, to));
				match (left, right) {
					(X, _) => source_err,
					(_, X) => source_err,
					_      => {
						area[to] = left;
						area[to + 1] = right;
						Ok(())
					}
				}
			},
			_ => {
				Err(format!("destination spaces are occupied ({} to {})", from, to))
			}
		}
		// let mut fs = self.a.slice(from, from + 2);
		// let mut ts = self.a.slice(to, to + 2);
		// let fo = vec::Vec::from_slice(fs);
		// let to = vec::Vec::from_slice(ts);
		// let fv = fo.as_slice();
		// let tv = to.as_slice();
		// match tv {
		// 	[X, X] => {ts = fv;
		// 	           fs = tv;
		// 	           Ok(())},
		// 	_      => {Err(format!("destination spaces are occupied ({} to {})",
		// 	                       from,
		// 	                       to))}
		// }
	}
}

impl fmt::Show for BaggageArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
		for s in self.a.iter() {
			// s.fmt(f);
			let r = s.fmt(f);
			match r {
				Err(_) => return r,
				_                => (),
			}
		}
		Ok(())
	}
}

fn run(num: uint) {
	let mut area = BaggageArea::new(num);
	println!("{}", area);
	area.move(num, num - 1).unwrap();
	println!("{}", area);
}


fn main() {
	let args = os::args();
	let num: uint = match validate_args(args) {
		Ok(number) => number,
		Err(err)   => return println!("{}", err),
	};
	run(num);
}

fn validate_args(args: Vec<String>) -> Result<uint, String> {
	let name = args[0].as_slice().split('/').last().unwrap();
	if args.len() == 1 { // no args
		return Err(format!("usage: {} bins", name));
	}
	let number = args[1].as_slice();
	let num: uint = match from_str(number) {
		Some(num) => num,
		None      => return Err(format!("{}: '{}' is not a natural number", name, number)),
	};
	if num < 3 {
		return Err(format!("{}: at least 3 bins required; {} specified", name, num));
	}
	if num > 100 {
		return Err(format!("{}: no more than 100 bins allowed; {} specified", name, num));
	}
	return Ok(num);
}
