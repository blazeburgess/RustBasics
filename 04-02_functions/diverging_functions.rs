// functions that do not return a value have their own syntax
fn main() {
    diverges();
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
// panic! is a macro that crashes the execution thread it is a part of
// with the message given as parameter.
// the main function begins the thread that diverges() ends.
//
// a diverging function works as any type, so any of thiese work:
let x: i32 = diverges();
let x: String = diverges();
