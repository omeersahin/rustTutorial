fn main() {
    let company ="TUBITAK"; // default &str
    let location = "Ankara";
    println!("company is : {} location :{}",company,location);


    let empty_string  = String::new();
    println!("This is Empty String : {}",empty_string);
    let notempty_string  = String::from("Not Empty String");
    println!("This is : {}",notempty_string);
}
