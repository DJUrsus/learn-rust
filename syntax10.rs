/**************************\
*  "Weird" things in Rust  *
\**************************/


// Compiler directives:

#[allow(unused_variable)]

// Normally, the compiler complains about unused variables, as well as unused
// functions and unreachable code (dead_code and unreachable_code).  See
// `rustc -W help`.

// Compiler directives can apply to statements; see List<T> in list.rs.

// No file-level executable statements; only declarations


// Function declarations use `fn`.
// If a file has a `main` function, it can be compiled to executable.  Any file
// can be compiled into a library.
fn main() {
	use std::string::String;


	// Whitespace-agnostic (mostly?)


	// Macro invocation/syntax extension:
	println!("foo");


	// Semicolons convert expressions to statements.
	// * A bare expression at the end of a function is an implicit return.

	// Unit/void is (), which is also its type, and "can be considered" a 0-tuple.

	// Combining the last 2 pieces of information:

	fn void1() -> () {
		return();
	}

	fn void2() -> () {
		return;
	}

	fn void3() -> () {
		()
	}

	fn void4() -> () {
	}

	assert!(void1() == void2() && void2() == void3() && void3() == void4());


	// `loop` is superior to `while true`.
	loop {
		if universe::recalibrate() {
			break;
		}
	}


	// `let` to declare variables.  `let mut` for mutability.
	let hi = "hi";
	let mut count: int = 0;

	// No parentheses are needed for `if`, `match`, `while`, and `return`.
	while count < 10 {
		println!("count is {}", count);
		count += 1;
	}


	// By convention, variables are worm case.  Types are camel case except
	// primitives.
	type MyType = int;


	// Variable types are declared with colons.
	let a: int  = 1;
	// Literal types are indicated with type suffixes.
	let b: int  = 10i;
	let c: uint = 100u;
	let d: i32  = 1000i32;


	// Characters use single quotes; strings use double.
	let mut buf: String = String::from_str("Hello!");
	buf.push_char('\n');
	print!("{}", buf);

	// Raw strings:
	println!(r"words \\\wor\0ds \!words\n");
	print!(r###"He said, "What?"
"###); // any number of hashes, but must match


	// `match` for type/value patterns.
	for num in range(-1i, 13) {
		print!("{}\t", num);
		match num {
			0     => println!("zero"),
			1 | 2 => println!("one or two"),
			3..10 => println!("three to ten"),
			_     => println!("something else"),
		}
	}
}

// THESE ARE HELPER DECLS
mod universe { pub fn recalibrate() -> bool { true } }
