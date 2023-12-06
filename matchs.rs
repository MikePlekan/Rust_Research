//Michael Plekan
#[allow(unreachable_patterns)]
fn main(){
    match [1,2,3,4,5,6,7,8,9,10] {
        [a, b, c, ..] if a < 0 => println!("a = {}, b = {}, c = {}", a, b, c),
        [a, b, .., c] if c == a => println!("a = {}, b = {}, c = {}", a, b, c),
        [a, b, ..] if a > 0 => println!("a = {}, b = {}", a, b),

        _ => println!("anything"),
    }
    let b = match 'a' {
        n @ 'a' ..= 'z' => format!("im the lowercase letter {}", n),
        n @ 'A' ..= 'Z' => format!("IM THE UPPERCASE LETTER {}", n),
        n => format!("im not a letter {}", n),
    };
    println!("b = {:?}", b);
}