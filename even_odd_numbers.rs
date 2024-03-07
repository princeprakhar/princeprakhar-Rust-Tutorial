use std::io;
fn main()
{
    let mut m=String::new();
    println!("Enter the number that you want to check: ");
    io::stdin().read_line(&mut m).expect("");
    let m:u32= m.trim().parse().expect("Invalid input");
    if m%2==0 {
        println!("{m} is Even number!");
    }
    else{
        println!("{m} is Odd number!");
    }
}