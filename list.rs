fn main() {

	#[deriving(Clone)]
	enum List<T> {
		Cons(T, Box<List<T>>),
		Nil,
	}

	fn prepend<T>(xs: List<T>, value: T) -> List<T> {
		Cons(value, box xs)
	}

	fn eq<T: PartialEq>(xs: &List<T>, ys: &List<T>) -> bool {
		match (xs, ys) {
			(&Nil, &Nil)
			    => true,

			(&Cons(ref x, box ref next_xs), &Cons(ref y, box ref next_ys))
			if x == y
			    => eq(next_xs, next_ys),

			_   => false,
		}
	}

	let xs = Cons('c', box Cons('a', box Cons('t', box Nil)));
	let ys = Cons('c', box Cons('a', box Cons('t', box Nil)));
	let mut zs = ys.clone();

	assert!(eq(&xs, &ys));
	assert!(eq(&ys, &zs));

	zs = prepend(zs, 's');
	assert!(!eq(&ys, &zs));
	match zs {
		Cons(_, box ref next) =>
			assert!(eq(&ys, next)),
		_ => (),
	}

}
