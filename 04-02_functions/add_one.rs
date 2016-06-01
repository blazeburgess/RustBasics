fn main() {
    add_one(32);
}

fn add_one(x: i32) -> i32 {
    x + 1
    // using a semi-colon would raise an error:
    //   error: not all control paths return a value
    //   help: consider removing this seicolon:
    //       x + 1;
    //            ^
}
