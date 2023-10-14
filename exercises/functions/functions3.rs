// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(10);
}

fn call_me(num: u32) {
    for i in 0..num {//for(i=0;i<num;i++)
        println!("Ring! Call number {}", i + 1);
    }
}
