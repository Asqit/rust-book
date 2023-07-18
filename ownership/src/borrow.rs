// In this function we do essentially the same as we did 
// in the ownership example. We take a string and slightly modify it's value.
// But here we do that with borrowing instead of taking entire ownership
fn append_surname(name: &mut String)  {
    name.push_str(" Tuček");
}


fn main() {
    let mut my_name: String = String::from("Andrew");

    // Because we want the modify this variable's value
    // we need to pass a mutable reference. 
    // This way, the function knows the memory location of the value 
    // and is able to mutate it. 
    append_surname(&mut my_name);

    // Take note, that rust does not allow you to have multiple 
    // mutable references. 
    // But unlike mutable, you can have x number of 
    // un-mutable references.
    // But you can't have mutable reference after un-mutable
    // cuz that would completely destroy the reason of the un-mutable 
    let x: &mut String = &mut my_name;
    let y: &mut String = &mut my_name;
    let z: &mut String = &mut my_name;


    x.push_str("string");
    
    
    println!("{my_name}");    
}