fn main() {
    //if-else
    let num = 10;
    if num > 0{
        println!("Number is higher than 0");
    } else if num < 0 {
        println!("Number is less than 0");
    } else {
        println!("Number is 0");
    }


    //match
    let match_code = "case1";
    let result = match match_code {
        "case1" => {    "Case1 printed"     },
        "case2" => {    "Case2 printed"     },
        _       => {    "Default printed"   }
        };
    println!("{result}");
}
