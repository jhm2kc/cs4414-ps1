use std::{os, float, uint};
fn main() {
	let args: ~[~str] =os::args();
	let mut sum: float = 0.0;
	let mut count: int = 0;
	let mut failed: bool = false;
	
	for uint::range(1,args.len()) |idx| {
		match float::from_str(args[idx]) {
			Some(x) => {
				sum += x;
				count +=1;
			}
			None => {
				println(fmt!("Not a valid input! %s", 						args[idx]));
				failed = true;
			}
		}
	}
	if failed == false {
		println(fmt!("Average: %f", sum/(count as float) ) )
	}
}
