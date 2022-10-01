// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct Iter<T> {
    next: Option<&Node<T>>,
}

pub struct List<T> {
    head: Link<T>,
}

//type alias
type Link<T> = Option<Box<Node<T>>>;
// enum Link {
//     Empty,
//     More(Box<Node>),
// }


struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.map(|node| &node)
        }
    }
    pub fn new() -> Self {
        List { head: None}
    }

    pub fn push(&mut self, elem:T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
/*        match self.head.take(){
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        } */
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        }) 
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        } 
    }
}
impl<T> Iterator for Iter<T> {
    type Item = &T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple () numerically
        self.next.map(|node| {
            self.next = node.next.map(|node| &node);
            &node.elem
        })
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
    #[test]
    fn peek() {
        use super::List;
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);
    
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });
    
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn into_iter() {
        use super::List;
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        
    }
}
