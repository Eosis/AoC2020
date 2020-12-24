use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Display;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
    pub prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }

    pub fn set_next(&mut self, to_set: &Rc<RefCell<Node<T>>>) {
        if self.next.is_some() {
            panic!("Oh dear");
        } else {
            self.next = Some(Rc::clone(&to_set));
        }
    }

    /// Returns the start of the removed list.
    pub fn take_three_out(to_remove_after: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let removed_start: Rc<RefCell<Node<T>>> = Rc::clone(to_remove_after.borrow().next.as_ref().unwrap());
        let removed_end = Rc::clone(
            removed_start
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap(),
        );
        {
            let mut changing = to_remove_after.borrow_mut();
            changing.next = Some(Rc::clone(removed_end.borrow().next.as_ref().unwrap()))
        }
        removed_start
    }

    pub fn insert_after(&mut self, to_insert: Rc<RefCell<Node<T>>>) {
        let for_after_inserted = self.next.take().unwrap();
        self.next = Some(Rc::clone(&to_insert));
        let newly_inserteds_tail = Rc::clone(
            to_insert
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap(),
        );
        let mut newly_inserteds_tail = newly_inserteds_tail.borrow_mut();
        newly_inserteds_tail.next = Some(for_after_inserted);
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) -> Rc<RefCell<Node<T>>> {
        let new_head = Node::new(elem);
        let to_return = Rc::clone(&new_head);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
        to_return
    }

    pub fn push_back(&mut self, elem: T) -> Rc<RefCell<Node<T>>> {
        let new_tail = Node::new(elem);
        let to_return = Rc::clone(&new_tail);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        to_return
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn tail_node(&self) -> Link<T> {
        if self.tail.is_some() {
            Some(Rc::clone(self.tail.as_ref().unwrap()))
        } else {
            None
        }
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         while self.pop_front().is_some() {}
//     }
// }

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

pub fn print_list_items<T: Display>(node: Rc<RefCell<Node<T>>>, items: usize) {
    let mut current_node = node;
    for _ in 0..items {
        println!("{}", current_node.borrow().elem);
        let next_node = {
            let borrowed_current = current_node.borrow();
            Rc::clone(borrowed_current.next.as_ref().unwrap())
        };
        current_node = next_node;
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use crate::day_23::linked_list::{print_list_items, Node};
    use hashbrown::HashMap;
    use std::cell::RefCell;
    use std::fmt::Display;
    use std::rc::Rc;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    fn setup_map() -> (List<u32>, HashMap<u32, Rc<RefCell<Node<u32>>>>) {
        let mut list: List<u32> = List::new();
        let mut map: HashMap<u32, Rc<RefCell<Node<u32>>>> = HashMap::new();
        let head = list.push_back(1);
        map.insert(1, Rc::clone(&head));
        for n in 2..=10 {
            let inserted = list.push_back(n);
            map.insert(n, Rc::clone(&inserted));
        }
        let inserted = list.push_back(11);
        map.insert(11, Rc::clone(&inserted));
        inserted.borrow_mut().set_next(&head);

        (list, map)
    }

    #[test]
    fn print_with_map() {
        let (mut list, mut map) = setup_map();
        let from_map = map.get(&5).unwrap();
        let next = from_map.borrow().elem;
        let next_again = from_map.borrow().next.as_ref().unwrap().borrow().elem;

        print_list_items(Rc::clone(list.head.as_ref().unwrap()), 11);
        println!("From 5");
        print_list_items(Rc::clone(map.get(&5).as_ref().unwrap()), 11);
    }

    #[test]
    fn take_center_out() {
        let (mut list, mut map) = setup_map();
        let to_remove_after = Rc::clone(map.get(&4).as_ref().unwrap());
        let removed_head = Node::take_three_out(to_remove_after);
        println!("With 3 removed");
        print_list_items(Rc::clone(map.get(&1).as_ref().unwrap()), 20);
        println!("Just the 3 that were removed");
        print_list_items(Rc::clone(&removed_head), 3);

        // Let's put them back in after 11.
        let to_insert_after = Rc::clone(map.get(&11).as_ref().unwrap());
        to_insert_after.borrow_mut().insert_after(removed_head);
        println!("After the re-insertion");
        print_list_items(Rc::clone(map.get(&1).as_ref().unwrap()), 11);
    }
}
