//Michael Plekan
#[allow(unreachable_patterns)]
fn main(){
    //this match is showing how to use the .. operator
    //it matches the rest of the values in the array
    match [1,2,3,4,5,6,7,8,9,10] {
        [a, b, c, ..] if a < 0 => println!("a = {}, b = {}, c = {}", a, b, c),
        [a, b, .., c] if c == a => println!("a = {}, b = {}, c = {}", a, b, c),
        [a, b, ..] if a > 0 => println!("a = {}, b = {}", a, b),

        _ => println!("anything"),
    }

    // this match is showing how to use the @ operator
    // it binds the value to the variable on the left
    // in this case, the variable is l for lowercase
    // and u for uppercase
    let b = match 'a' {
        l @ 'a' ..= 'z' => format!("im the lowercase letter {}", l),
        u @ 'A' ..= 'Z' => format!("IM THE UPPERCASE LETTER {}", u),
        n => format!("im not a letter {}", n),
    };
    println!("b = {:?}", b);
}