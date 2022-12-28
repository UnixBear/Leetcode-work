//our initial layout where we swap out Box with RC
//becuase we want to have shared references where we
//drop one of the references to the referent without
//having to drop the whole list and invalidate all
//the other lists.  RC lets us copy, unlike Box, and
//the allocated memory doesn't drop until there are 
//no more references to that segment of memory

use std::rc::Rc;
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
        elem: T,
        next: Link<T>,
}

//constructor 
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None}
    }
    //we replace push with a new function called `prepend`.
    //Instead of having a mutable List where we manually
    //change our references, we can implicitly use the Copy
    //trait given as part of Rc.  Clone in Rc is also used to 
    //count references, which serves as a very primitive 
    //Garbage collector
    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    //We replace pop with `tail` because it's easier, given
    //we are using Rcs instead of Boxes, to just clone the second
    //element (which contains the rest of the list) than having
    //to manage the removing of the head and changing the references
    //manually.  Note: we are using and_then() instead of map()
    //because head.as_ref() is giving us an Option<Option<Rc...>>
    //and map wants the direct value
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }

    //We should have a function that gives us the head element
    //of a list for completeness sake. We'll return an 
    //Option in case we have an empty list.
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        //note: we need the prepend statement below
        //for the rust compile to infer the type of
        //the list to be i32
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(),Some(&3))
    }
}

