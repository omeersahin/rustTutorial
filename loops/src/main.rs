fn main() {
    //Definite Loop
    for i in 1..101{ // 101 is not inclusive
        if i%5==0 {
            println!("{i} is divisible by 5");
        } 
    }
    //Indefinite Loops ( While & Loop)
    // While
    let mut x = 0;
    while x < 101{
        x+=1;
        if x%6==0{
            println!("{x} is divisible by 6");
        }
    }

    // Loop
    let mut y = 0;
    loop {
        y+=1;
        if y%7==0{
            println!("{y} is divisible by 7");
        }

        if y==101 {
            break;
        }
    }

}
