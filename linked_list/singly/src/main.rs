use std::borrow::{BorrowMut};

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_front(&mut self, el: T) {
        let new_node = Box::new(
            Node {
                elem: el,
                next: self.head.take(),
            }
        );
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        return self.last(index)
            .as_mut()
            .map(|n| &mut n.elem);
    }

    pub fn get(&mut self, index: usize) -> Option<&T> {
        return self.last(index)
            .as_ref()
            .map(|n| &n.elem);
    }

    fn last(&mut self, index: usize) -> &mut Link<T> {
        let mut cur = &mut self.head;
        for _i in 0..=index {
            if cur.is_none() {
                return cur;
            } else {
                // let a = cur.as_mut().map(|n| &mut n.next);
                // cur = cur.as_mut().map(|n| &mut n.next);
                // cur = cur.map(|n| n.next);
            }
        }
        return cur;
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn check_empty_list() {
        let mut list: List<i32> = List::new();

        assert_eq!(list.get(0), None);
    }

    #[test]
    fn add_front() {
        let mut list: List<i32> = List::new();
        list.push_front(1);

        assert_eq!(list.get(0), Some(&1));
    }
}
