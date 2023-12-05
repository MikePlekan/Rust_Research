//Micahel Plekan

//Starter code found: https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html

#[allow(unused_labels)]//allows the labels to not be used, normally the compiler would give a warning
fn main() {
    let mut x:i32=0;
    'outer: while x<10 {//doesn't reach 10 because the outer loop gets broken out of during the first iteration
        println!("Entered the outer loop");
        x+=1;
        'inner: loop {
            println!("Entered the inner loop");
            'innerinner: loop {
                println!("Entered the innerinner loop");
                // This breaks the outer loop
                break 'outer;
            }
        }
    }
    println!("Exited the outer loop");
    println!("X is {}",x);
}