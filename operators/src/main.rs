fn main() {
    // Arithmetic Operators

    let x = 3;
    let y = 2;
    // Addition +
    let sum = x + y;
    println!("x + y = {sum}");
    // Subtraction -
    let diff = x - y;
    println!("x - y = {diff}");
    // Multiplication *
    let mult = x * y;
    println!("x * y = {mult}");
    // Division /
    let div = x / y;
    println!("x / y = {div}");
    // Modulus %
    let modulus = y % x;
    println!("x % y = {modulus}");
    

    // Condition Operators

    // Greater than >
    let bool1 = 3 > 2;
    println!("3 > 2 = {bool1}");
    // Lesser than <
    let bool2 = 3 < 2;
    println!("3 < 2 = {bool2}");
    // Greater than or equal to >=
    let bool3 = 3 >= 2;
    println!("3 >= 2 = {bool3}");
    // Lesser than or equal to <=
    let bool4 = 3 <= 2;
    println!("3 <= 2 = {bool4}");
    // Equality ==
    let bool5 = 3 == 2;
    println!("3 == 2 = {bool5}");
    // Not equal !=
    let bool6 = 3 != 2;
    println!("3 != 2 = {bool6}");

    // Logical Operators

    // AND &&
    println!("AND Operator");
    println!("1 AND 1 =   {}",true &&true );
    println!("1 AND 0 =   {}",true &&false);
    println!("0 AND 1 =   {}",false&&true );
    println!("0 AND 0 =   {}",false&&false);
    // OR  ||
    println!("OR Operator");
    println!("1  OR 1 =   {}",true ||true );
    println!("1  OR 0 =   {}",true ||false);
    println!("0  OR 1 =   {}",false||true );
    println!("0  OR 0 =   {}",false||false);
    // NOT !
    println!("NOT Operator");
    println!("NOT 1   =   {}",!true );
    println!("NOT 0   =   {}",!false);


    // Bitwise Operators
    // 7 -> 0111
    // 8 -> 1000 
    // AND &
    // OR |
    // XOR ^
    // NOT !
    // Left Shift <<
    // Right Shift >>
    // Right Shift with Zero >>>
    println!("Bitwise Operator");
    println!("Bitwise 14-> 1110");
    println!("Bitwise 8 -> 1000");

    println!("8 AND 14=   {}",8 & 14);
    println!("8 OR  14=   {}",8 | 14);
    println!("NOT 8   =   {}",!8    );//32 bit 11111111111111111111111111110111
    println!("8 XOR 14=   {}",8 ^ 14);
    println!("<<8     =   {}",8<<1  );
    println!(">>8     =   {}",8>>1  );



}
