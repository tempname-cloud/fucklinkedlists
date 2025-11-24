### Linked List Definition

A linked list is a structure that contains an element followed by another list.

### Purpose of Using `Box<T>`

If we try to define a list like this:

```rust
pub enum List {
    Empty,
    Elem(i32, List)
}
```

Rust cannot determine the size of `List` at compile time, because this definition implies *infinite size*.  
A `Box<T>` solves this by allocating the element on the heap and giving the enum a known fixed size (the size of a pointer).

It also ensures ownership of the data and allows storing up to `isize::MAX` bytes.

### Correct Linked List Definition

To fix the infinite-size issue, we define:

```rust
pub enum List<T> {
    Empty,
    Elem(T, Box<List<T>>)
}
```

This way, each node contains a value and a boxed pointer to the next node.

This still has some issues however. The *first* element is an enum that begins, being allocated on the stack, and the following values are on the heap. Eventually one of the following values (nested Box<T>) will contain an Empty enum and junk. The reason for this is because the List enum needs to have enough space to contain either List::Empty or List::Elem(some_value). When its empty, the remaining space is unused. So one node isn't heap allocated and the List contains a fake node. 

In order to avoid that, we can set up the Linked List with a Node using the classic C/C++ setup.

```rust
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
```

We have a Singly Linked List (SLL) with one variable containing a Link enum. The Link enum has 2 states, either Empty or More(Box<Node>) which contains a pointer to a Node struct. This might sound confusing, but under the hood the Link enum data is stored as one pointer value. When this pointer value is null, rust represents it as Link::Empty and when it is non-null, rust represents it with Link::More(Box<Node>).