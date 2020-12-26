use std::cell::RefCell;
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
            elem,
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
    use crate::day_23::LabelNodeMap;
    use hashbrown::HashMap;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn setup_map() -> (List<u32>, LabelNodeMap) {
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
        let (list, map) = setup_map();
        print_list_items(Rc::clone(list.head.as_ref().unwrap()), 11);
        println!("From 5");
        print_list_items(Rc::clone(map.get(&5).as_ref().unwrap()), 11);
    }

    #[test]
    fn take_center_out() {
        let (_, map) = setup_map();
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
