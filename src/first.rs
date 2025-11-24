//! src/first.rs

pub struct List {
    head: Link
}

pub enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    data: i32,
    next: Link
}

