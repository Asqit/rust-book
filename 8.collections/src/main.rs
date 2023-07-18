// Collections are data types, that differs from primitives 
// because their data are stored on heap, meaning they can grow / shrink
// in the runtime. You may know these already, one of the we used in rust already.
// ------------------------------------------------------------------------------
// Common collections
// - String: Is collection of characters
// - Vector: Allows you to dynamically store any amount of items next to each other (dynamic array)
// - HashMaps: Allows you to pair a value with a particular key 

fn main() {

    // 1) Vectors
    // Vectors takes a generic type
    // and basically are dynamic arrays (they grow to infinity and shrink to 0)
    let mut vec: Vec<i32> = Vec::new();

    for i in 1..= 100 {
        vec.push(i);
    }

    // Although this method exists, it is recommended to use the '.get()' method
    // as the first approach causes panic because it references a nonexistent element
    let first_slice: &i32 = &vec[0];

    // This approach is safer as the '.get()' method returns Option enum
    // which we can easily match every case of and thus safely handle all possibilities.
    println!("First:{first_slice}..Last:{}", match vec.get(99) {
        None => 0,
        Some(i) => *i
    });


    // 2) String
    // Strings are sequence of characters encoded in UTF-8 standard
    // Strings can grow and shrink on runtime. You can also conveniently 
    // contact strings with + or the 'format!' macro
    let mut foo: String = String::new();

    foo.push('a');
    foo.push('h');
    foo.push('o');
    foo.push('j');
    foo.push_str(", Světe!");

    let mut bar: String = String::from("\nJak se máš?");

    // We have now taken ownership of 'string'
    // and can no longer be accessed. 
    // Also we had to use reference to 'str' variable
    // in order to get it's content and
    // because each variable can have at most one owner
    // we can't do "string + str" that would mean, that we would 
    // try to get ownership of two strings.
    let concat = foo + &bar;

    // Also notice, that even if we change the value of bar
    // value of concat will still be the same as it was at the 
    // definition. 
    bar.clear();
    bar.push_str("\nJak se vede?");

    println!("{concat} {bar}");



}
