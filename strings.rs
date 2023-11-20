fn main() {
    //create a mutable string
    let mut string:String = "hello world!".to_string();
    println!("{}",string);

    //edit the string
    string = string+" \nbye!";
    println!("{}",string);

    //get each char from the string and iterate through it
    for x in string.chars(){
        print!(":{}:",x);
    }
    //get the list of chars, filter out everything but 'l's, then count how many
    println!("\nThe number of 'l': {}",string.chars().filter(|x|*x=='l').count());
    
    //get each word seperated by whitespace from the string and iterate through it
    for (x,y) in string.split_whitespace().enumerate(){
        print!("{}:{}, ",x,y);
    }
    println!();
}