extern crate guerrilla;
extern crate lib;

use lib::*;

fn main() {
    println!("fun1:{}, fun2:{}", fun1(), fun2());
    let _guard = guerrilla::patch0(fun1, replacement);
    println!(
        "fun1:{}, fun2 (will crash because it was trampled!):{}",
        fun1(),
        fun2()
    );
    println!(
        "Need to use padding function to avoid it being optimized away {}:",
        padding(1)
    );
}
