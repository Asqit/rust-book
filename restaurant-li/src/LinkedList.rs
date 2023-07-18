struct Link<T> {
    item: T,
    link: &T,
}

pub struct LinkedList<T> {
    capacity: i32,
    list: [Link], 
}

impl LinkedList {
    pub fn push(&self, item: &T) {
        self.capacity.push(item);
    }
}