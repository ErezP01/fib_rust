use std::io;
fn main() {
    println!("what number of the sequcence would you like to print");
    let mut nth = String::new();
    io::stdin().read_line(&mut nth).expect("failed to read");
    let nth: u32 = match nth.trim().parse() { 
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut counter = 1;
    let mut curr = 1;
    let mut prev = 1;
    let mut fib;
    while counter <= nth{
        if counter < 2{
            println!("current is 1");
            counter = counter + 1;
        }
        else{
        fib = curr + prev;
        println!("in the {} place the current is {}",counter, fib);
        prev = curr;
        curr = fib;
        counter = counter + 1;
        }
    }
        
}


