fn main(){
	{
		let mut a: i8 = 10;
		let mut b: i8 = 45;
		
		println!("Before the Swapping (XOR/BITWISE)");
		println!("a: {a}");
		println!("b: {b}");

		a = a ^ b; // new value for a is -> 0010 0111
		b = a ^ b; // new value for b is -> 0000 1010
		a = a ^ b; // new value for a is -> 0010 1101

		println!("After the Swapping (XOR/BITWISE)");
		println!("a: {a}");
		println!("b: {b}");
	}
	// Using Arithmetics (bitwise)
	println!("\n==================\n");
	{
		let mut a: i8 = 10;
		let mut b: i8 = 45;
		
		println!("Before the Swapping (Arithmetics)");
		println!("a: {a}");
		println!("b: {b}");

		a = a + b; // new value for a is -> 55
		// a needs to be first, because is the higher number
		b = a - b; // new value for b is -> 10
		a = a - b; // new value for a is -> 45

		println!("After the Swapping (Arithmetics)");
		println!("a: {a}");
		println!("b: {b}");
	}
}
