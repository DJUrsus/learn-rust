// need that {:?} format
extern crate debug;

fn main() {
	struct BigStruct {
		i: int,
	}

	// Structs, enums, and primitives normally live on the stack.
	let mut a: BigStruct = BigStruct { i: 4 };
	println!("{:?}", a);

	// A Box is a reference to a heap value.  A Box is declared with `Box<T>` and instantiated with `box`.
	let mut b: Box<BigStruct> = box BigStruct { i: 5 };
	println!("{:?}", b);

	{
		// A reference borrows a pointer, and is declared and instantiated with `&`.
		let a2: &BigStruct = &a;
		println!("{:?}", a2);

		// Boxes are already references.
		let b2: &BigStruct = b;
		println!("{:?}", b2);

		// Because a and b have been borrowed they are frozen.  They aren't mut
		// in this context, and the following statements will fail to compile if
		// uncommented.

		// a.i += 2;
		// println!("{:?}", a);

		// b.i += 2;
		// println!("{:?}", b);
	}

	// Now the variables we lent them to have been destroyed, so they are no
	// longer frozen.

	a.i += 2;
	println!("{:?}", a);

	b.i += 2;
	println!("{:?}", b);

	// I still don't quite understand how to deal with object lifetime ending
	// during a return statement.
}
