use std::io;
use crossterm::{
	terminal::{ClearType, Clear},
	execute,
	cursor
};

struct Resistor {
	v1: f64,
	v2: f64,
}

impl Resistor {
	fn new() -> Self {
		Resistor {
			v1: 0.0,
			v2: 0.0,
		}
	}

	fn parallel(&self) {
		let sum: f64 = (1.0 / self.v1) + (1.0 /self.v2);
		let sum_inverse: f64 = 1.0 / sum;

		println!("R_Parallel [{}, {}]: {}", self.v1, self.v2, sum_inverse);
	}
}

fn input(prompt: &str) -> String {
	println!("{}", prompt);
	let mut buf: String = String::new();
	io::stdin().read_line(&mut buf).unwrap();
	String::from(buf.trim())
}

fn clear_terminal() {
	execute!(
		io::stdout(),
		Clear(ClearType::FromCursorUp),
		cursor::MoveTo(0,0)
	).unwrap();
}

fn main() {
	clear_terminal();
	let mut resistor: Resistor = Resistor::new();
	loop {
		resistor.v1 = input("Resistor 1: ").parse().unwrap();
		println!("");
		resistor.v2 = input("Resistor 2: ").parse().unwrap();
		clear_terminal();
		resistor.parallel();
		println!("");
	}
}
