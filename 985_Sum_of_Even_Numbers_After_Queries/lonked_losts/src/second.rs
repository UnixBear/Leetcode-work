

//pub means we want people outside this module to be able to use List..
//We turn List into a List of type <T> to make it generic
pub struct List<T> {
    head: Link<T>,
}


//We're implementing a tuple struct here
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //Access fields of a struct numerically
        self.0.pop()
    }
}

//Iter is generic so we have to specify a lifetime
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

//Since we only use lifetimes for functions and types
//we don't don't need it for this declaration of our
//implementation
impl<T> List<T> {

    //We need a lifetime for this function because we have
    //a borrow of self that creates the iter. &self  needs
    //to be valid for as long as Iter is around
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref().map(|node| &*node)
        }
    }   
}

impl<'a, T> Iterator for Iter<'a, T> {
    //Because our type includes a lifetime, we don't need
    //it in our next definition because it's presumed in
    //the type declaration.
    type Item = &'a T;


    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| &*node);
            &node.elem
        })
    }
}


//Since mutable references are not to be shared, a mutable iterable
//is going to be more complicated than the shared references of
//Iter. 

//Setting up our struct so we can handle a mutable list and
//explicitly linking the lifetimes to the reference
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

//iter_mut return type needs an anonymous lifetime for elision
//since it's an implementation for a struct
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_,T> {
        IterMut { next: self.head.as_deref_mut()}
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node | {
            self.next=node.next.as_deref_mut();
            &mut node.elem
        })
    }

}



//We'll replace our Link with a type alias that is an Option<Box<Node>>,
//since Link is an enum that either has Empty or Box<Node>>
//enum Link {Empty, More(Box<Node>),}

//We'll replace mem::replace(x,y) with Option's take method, since
//it does the same thing

//We'll swap out our match call in pop() with a map() so we have an 
//inline implementation which is easier to debug

//Originally we had elem by a signed 32-bit integer, but now we'll
//have it be generic over types by making Link an option of a box of
//of node which we'll add a generic type T to and replace references
//i32 with <T>
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }
    pub fn push(&mut self, elem:T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        //Also replacing Link::More(new_node) with Some(new_node)
        self.head = Some(new_node);
    }

    //swapping out match for a closure here in pop()
    pub fn pop(&mut self) -> Option<T> {
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
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
            *value = 42;
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn into_iter() {
        use super::List;
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None); 
    }

    #[test]
    fn iter() {
        use super::List;
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
        
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        use super::List;
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut     iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));

    }
}