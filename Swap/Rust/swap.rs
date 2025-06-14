fn main(){
	{
		// Using Bitwise (XOR)
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
	println!("\n==================\n");
	// Using Arithmetics (+, -)
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
	// Simplier way
	println!("\n==================\n");
	{
		let mut a: i8 = 10;
		let mut b: i8 = 45;
		
		println!("Before the Swapping (Simplier way)");
		println!("a: {a}");
		println!("b: {b}");

		(a, b) = (b, a);

		println!("After the Swapping (Simplier way)");
		println!("a: {a}");
		println!("b: {b}");
	}
}
