use std::io;

fn main() {
	/*
	###############################
		variables and mutability
	###############################
	
	let x = 5;
	let mut y = 5;
	const MAX_POINTS:u32 = 10_000; //must decalre tye of const

	//x = 6; will give error as vars are immutable by default
	y = 6;
    println!("x is: {} and y is: {}", x, y);
    println!("MAX_POINTS is: {}", MAX_POINTS);

    let x = x*2;
    println!("x is: {}", x);
    */

    /*
	###############################
			 Data Types
	###############################
	
	let guess: u32 = "42".parse().expect("Not a number");

	//Rust has 4 scalar (single value) types : integers, floating-point, Booleans, and characters
	//ints => i8-i128, u8-u128, isize, usize
	/*Literals:
			Decimal: 98_222
			Hex: 0xff
			Octal: 0o77
			Binary: 0b1111_0000
			Byte(u8 only): b'A'
	*/
	//floats => f32, f64
	//bool => true or false
	//chars (4bytes)

	//Compound types: tuples and arrays

	//tuples: immutable with fixed length and allow different data types
	let tup = (500, 3.3, 'a');
	let (x, y, z) = tup;
	let i = tup.0;

	//arrays: immutable with fixed size and dont allow different types
	let months = ["January", "February", "..."];
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	let b = [3; 5]; //[3, 3, 3, 3, 3]
	let i = b[0];

    println!("i is: {}", i);
    */
    /*
	###############################
			 Functions
	###############################
	

	let _b = {
		let a = 3;
		a+1
	};

	println!("Please input first number:");
	let mut x = String::new();
	io::stdin().read_line(&mut x).expect("Failed to read line");

	println!("Please input second number:");
	let mut y = String::new();
	io::stdin().read_line(&mut y).expect("Failed to read line");

	let x: i32 = match x.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	let y: i32 = match y.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	let z : i32 = sum(x,y);

	println!("z is: {}", z);
	*/

	/*
	###############################
			Flow Control
	###############################
	*/

	println!("Please input your age:");
	let mut age = String::new();
	io::stdin().read_line(&mut age).expect("Failed to read line");

	let age: u32 = match age.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	if age < 18 {
		println!("You are a minor");
	} else if age > 18{
		println!("You are not a minor");
	}else{
		println!("You just became an adult");
	}

	//note: both if and else returns must be of same type so Rust knows during compile time var type
	let minor = if age < 18 {
		true
	} else {
		false
	};

	println!("This person is a minor: {}", minor);
	
	let mut counter = 0;
	let result = loop {
		counter+=1;

		if counter == 10 {
			break counter*2;
		}
	};

	println!("The result is: {}", result);
	
	let mut number = 3;
	while number != 0 {
		println!("{}!", number);

		number += -1;
	}

	println!("The number is: {}", number);

	let a = [10,20,30,40,50];
	let mut index = 0;
	//might cause program to panic, use iter instead
	while index < 5 {
		println!("The vallue is {}", a[index]);
		index+=1;
	}

	for element in a.iter(){
		println!("the value is {}", element);
	}

	for number in (1..4).rev() {
		println!("{}", );
	}
	/*
	match some_pattern {
		diff_pattern_1 => println!("1"),
		some_pattern => println!("2"),
		diff_pattern_2 => println!("3"),
	};
	*/
}

/*
fn sum(x:i32, y:i32) -> i32{
	//return x+y
	x+y
}*/