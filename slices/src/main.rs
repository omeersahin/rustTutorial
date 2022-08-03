fn main() {
    let str1 = "This is String";
    let slice1 = &str1[0..7];
    println!("{slice1}");


    let mut num_array = [11,22,33,44,55,66,77,88,99];
    let slice2 = &mut num_array[3..6];
    println!("{:?}",slice2);

    slice2[2] = 100;
    println!("{:?}",num_array);
}
