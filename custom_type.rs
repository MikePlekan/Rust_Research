#[repr(i32)]
enum Coins{
    Penny=1,
    Nickel=5,
    Dime=10,
    Quater=25,
    Dollar(i32)
}

impl Coins{
    fn to_dollars(&self)->f64{
        match self{
            Coins::Penny => return 0.01,
            Coins::Nickel => return 0.05,
            Coins::Dime => return 0.1,
            Coins::Quater => return 0.25,
            Coins::Dollar(v) => return *v as f64, 
        }
    }
}

fn main(){
    let p=Coins::Penny;
    println!("p is worth {}",p.to_dollars());
    let n=Coins::Nickel;
    println!("p is worth {}",n.to_dollars());
    let d=Coins::Dime;
    println!("p is worth {}",d.to_dollars());
    let q=Coins::Quater;
    println!("p is worth {}",q.to_dollars());
    let dol=Coins::Dollar(10);
    println!("p is worth {}",dol.to_dollars());
}