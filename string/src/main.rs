fn main() {
    let company ="TUBITAK"; // default &str
    let location = "Ankara";
    println!("company is : {} location :{}",company,location);


    // String simply a vector of u8, heap-allocated and growable.
    // &str immutable called string slice
    // &mut str mutable slice
    let empty_string  = String::new();
    println!("This is Empty String : {}",empty_string);


    let notempty_string  = String::from("Not Empty String");
    println!("This is : {}",notempty_string);

    let str1: &str = "test";
    let str2: String = String::from("test");
    let condition = str1 == str2;
    println!("str1 == str2 : {}", condition);


    println!("Length : {}", str1.len());
    let int_value = 12;
    println!("intToString : {}", int_value.to_string());

    //replacing
    println!("Replaced t to T : {}", str1.replace("t","T"));


    //pushing char
    let mut str3: String = String::from("TestString"); 
    str3.push('1');
    println!("TestString and pushed '1' : {}",str3) ;

    //pushing str
    let mut str4: String = String::from("TestString"); 
    str4.push_str("1234");
    println!("TestString and pushed '1234' : {}",str4) ;

    //split
    let str5 = "string1,string2,string3";
    let tokens:Vec<&str>= str5.split(",").collect();
    println!("First String : {}",tokens[0]) ;
    println!("Second String : {}",tokens[1]) ;
    println!("Third String : {}",tokens[2]) ;

    //chars convert char array
    let str6 = "This is String";
    for n in str6.chars(){
        println!("{}",n);
    }

//Concatenation

    let str7 = "String".to_string();
    let str8 = "Concat".to_string();

    let str9 = str7 + &str8; // n2 reference is passed
    println!("{}",str9);

    


    


}
