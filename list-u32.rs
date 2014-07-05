fn main() {

	#[deriving(Clone)]
	enum List {
		Cons(u32, Box<List>),
		Nil,
	}

	fn prepend(xs: List, value: u32) -> List {
		Cons(value, box xs)
	}

	fn eq(xs: &List, ys: &List) -> bool {
		match (xs, ys) {
			(&Nil, &Nil)
			    => true,

			(&Cons(x, box ref next_xs), &Cons(y, box ref next_ys))
			if x == y
			    => eq(next_xs, next_ys),

			_   => false,
		}
	}

	let xs = Cons(5, box Cons(10, box Nil));
	let ys = Cons(5, box Cons(10, box Nil));
	let mut zs = ys.clone();

	assert!(eq(&xs, &ys));
	assert!(eq(&ys, &zs));

	zs = prepend(zs, 0);
	assert!(!eq(&ys, &zs));
	match zs {
		Cons(_, box ref next) =>
			assert!(eq(&ys, next)),
		_ => (),
	}

}
