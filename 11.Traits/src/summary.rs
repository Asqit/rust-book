pub trait Summary {    
    fn summarize(&self) -> String;
    
    // Rust also allows us to define a default implementation
    fn summarize_default(&self) -> String {
        String::from("(read more...)")
    }
}