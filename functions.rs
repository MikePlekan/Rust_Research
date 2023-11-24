fn main(){
    //closure that prints the sum of 2 numbers
    let print_sum = |x: i32, y: i32| println!("Sum: {} + {} = {}", x, y, x+y);

    print_sum(13, 37);
    
    //function prints out the first n number of fibonacci numbers
    fn fibonacci(n: i32){
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            print!("{}, ", a);
            let c = a + b;
            a = b;
            b = c;
        }
        println!();
    }
    //call the function
    fibonacci(10);

    let hello: &'static str = "Hello, world!";

    //closure that prints hello
    let print_hello = || println!("{}", hello);

    print_hello();
}