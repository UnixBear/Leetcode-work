use std::mem;
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }
    pub fn push(&mut self, elem:i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    }

#[cfg(test)]
mod test {
    #[test]
    fn wurld() {
        use super::List;
        let mut list = List::new();

        //check empty list behaves right
        assert_eq!(list.pop(), None);

        //populate list
        list.push(1);
        list.push(3);
        list.push(5);

        //check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(3));
        
        //push more to see if we get our remaining 1 and our new pushes
        list.push(8);
        list.push(13);

        //now check
        assert_eq!(list.pop(), Some(13));
        assert_eq!(list.pop(), Some(8));

        //check rest
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}