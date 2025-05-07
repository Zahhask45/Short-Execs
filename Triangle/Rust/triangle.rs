fn main(){
	let size = 10; // size of the rows
	for i in 0..size{
		let stars = i + 1; //amount of stars to print
		for _ in 0..stars{
			print!("*");
		}
		println!("");
	}
}
