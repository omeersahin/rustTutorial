//Defining a function
fn fn_hello1(){
    println!("hello from function fn_hello1 ");
}

fn main(){
//calling a function
    fn_hello1();
    fn_hello2();
    println!("{}",return_value());
    let mut x = 0;
    pass_by_value(x);
    println!("After Pass by Value x = {x}");
    pass_by_referance(&mut x);
    println!("After Pass by Reference x = {x}");

}
//Defining a function
fn fn_hello2(){
    println!("hello from function fn_hello2 ");
}

// Return Type

fn return_value()->&'static str {
    return "Returned String";
 }

fn pass_by_value(mut x:i32){
    x = x + 2;
    println!("Inside Pass by Value x = {x}");
}

fn pass_by_referance(x: &mut i32){
    *x = *x + 2;
    println!("Inside Pass by Reference x = {}",*x);
}


