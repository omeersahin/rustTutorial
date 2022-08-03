fn main(){
    let v = vec![1,2,3];       // vector v owns the object in heap
    let v2 = v;                // moves ownership to v2
    let v2_return = printing(v2);    //returned the ownership to v2_return
    println!("After return {:?}",v2_return);


    // Primitive variables needs less resources than an object so no ownership move happenning
    let u1 = 10;
    let u2 = u1;  // u1 value copied(not moved) to u2
 
    println!("u1 = {}",u1);


    //Borrowing with &(reference), after function used the variable it gives the ownership back.
    let vec1 = vec![10,20,30];
    print_vector(&vec1); // passing reference
    println!("After borrowed vec1[0]={}",vec1[0]);


    let mut number = 3;
    increment(&mut number);
    println!("{}", number);

}
 fn printing(v:Vec<i32>)->Vec<i32> { 
    // returning same vector
    println!("inside printing function {:?}",v);
    v
 }

 fn print_vector(x:&Vec<i32>){
    println!("Inside print_vector function {:?}",x);
 }

 fn increment(x: &mut i32) {
    *x+= 1;
 }