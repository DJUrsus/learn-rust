fn main() {

	#[deriving(Show)]
	enum Direction {
		North,
		East,
		South,
		West,
	}

	println!( "{} => {}", North, North as int );

	enum Color {
		Red = 0xff0000,
		Green = 0x00ff00,
		Blue = 0x0000ff,
	}

	struct Point {
		x: f64,
		y: f64,
	}

	enum Shape {
		Circle(Point, f64),
		Rectangle(Point, Point)
	}

	fn area(sh: Shape) -> f64 {
		use std::f64;
		match sh {
			Circle(_, size) => f64::consts::PI * size * size,
			Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
		}
	}

	println!("green: {}", Green as int)

	let c1 = Circle(Point{x:0.0, y:1.0}, 2.0);
	println!("area: {}", area(c1));

	let r1 = Rectangle(Point{x:0.0, y:0.0}, Point{x:4.0, y:3.0});
	println!("area: {}", area(r1));

	let mytup: (int, int, f64) = (10, 20, 30.0);
	match mytup {
		(a, b, c) => println!("tuple {}", a + b + (c as int))
	}

    struct MyTup(int, int, f64);
    let mytup2: MyTup = MyTup(10, 20, 30.0);
    match mytup2 {
        MyTup(a, b, c) => println!("tuple struct {}", a + b + (c as int))
    }
}
