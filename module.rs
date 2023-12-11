//Michael Plekan

//module for fruit
#[allow(dead_code)]

pub mod fruit {
    pub mod seed {
        pub static NUM:i32 = 10;
        pub fn eat() {
            println!("Eating seed");
        }
    
    }
    fn eat() {
        println!("Eating fruit");
    }
    pub fn peel() {
        println!("Peeling fruit");
    }
}


//using the module
fn main() {
    fruit::peel();
    fruit::seed::eat();
    println!("Number of seeds: {}", fruit::seed::NUM);
    //fruit::eat(); //error: function private
}