use std::{os, uint};

fn main() {
  let args: ~[~str] = os::args();
let y: int = 4;
assert! (y == 4);
for uint::range(1,args.len()) |x| {
  print(fmt!("%s ",args[x]));

}

println("");
}
