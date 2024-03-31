#[derive(Clone, Debug, PartialEq)]
struct BoxedNode<T> {
    data: T,
    next: Option<Box<BoxedNode<T>>>,
}

impl<T> BoxedNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

struct BoxedSinglyLinkedList<T> {
    size: usize,
    head: Option<Box<BoxedNode<T>>>,
}

impl<T: PartialEq> BoxedSinglyLinkedList<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, data: T) {
        let new_node = Box::new(BoxedNode::new(data));

        if let Some(ref mut head) = self.head {
            let mut current = head;
            while let Some(ref mut next) = current.next {
                current = next;
            }
            current.next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }

        self.size += 1;
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn remove(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    fn contains(&self, data: T) -> bool {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = &node.next;
        }

        false
    }

    fn index_of(&self, data: T) -> Option<usize> {
        let mut current = &self.head;
        let mut i = 0;

        while let Some(node) = current {
            if node.data == data {
                return Some(i);
            }

            current = &node.next;
            i += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        use std::any::{Any, TypeId};

        assert_eq!(
            BoxedSinglyLinkedList::<u8>::new().type_id(),
            TypeId::of::<BoxedSinglyLinkedList<u8>>()
        );
        assert_ne!(
            BoxedSinglyLinkedList::<u8>::new().type_id(),
            TypeId::of::<BoxedSinglyLinkedList<u16>>()
        );
    }

    #[test]
    fn default_instantiation() {
        let singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();

        assert_eq!(singly_u8_linked_list.size, 0);
        assert_eq!(singly_u8_linked_list.size(), 0);
        assert_eq!(singly_u8_linked_list.head, None);
        assert_eq!(singly_u8_linked_list.peek(), None);
    }

    #[test]
    fn first_peek_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();

        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.size, 1);
        assert_eq!(singly_u8_linked_list.size(), 1);
        assert!(singly_u8_linked_list.head.is_some());
        assert_eq!(
            singly_u8_linked_list.head,
            Some(Box::new(BoxedNode::<u8>::new(0)))
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&0));
    }

    #[test]
    fn first_add_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();

        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.size, 1);
        assert_eq!(singly_u8_linked_list.size(), 1);
        assert!(singly_u8_linked_list.head.is_some());
        assert_eq!(
            singly_u8_linked_list.head,
            Some(Box::new(BoxedNode::<u8>::new(0)))
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&0));
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().data, 0);
        assert!(singly_u8_linked_list.head.as_ref().unwrap().next.is_none());
        assert_eq!(singly_u8_linked_list.head.unwrap().next, None);
    }

    #[test]
    fn second_add_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();
        let mut first_node = BoxedNode::<u8>::new(128);
        let second_node = BoxedNode::<u8>::new(255);
        first_node.next = Some(Box::new(second_node));

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);

        assert_eq!(singly_u8_linked_list.size, 2);
        assert_eq!(singly_u8_linked_list.size(), 2);
        assert!(singly_u8_linked_list.head.is_some());
        assert_eq!(singly_u8_linked_list.head, Some(Box::new(first_node)));
        assert_eq!(singly_u8_linked_list.peek(), Some(&128));
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().data, 128);
        assert!(singly_u8_linked_list.head.as_ref().unwrap().next.is_some());
        assert_eq!(
            singly_u8_linked_list.head.as_ref().unwrap().next,
            Some(Box::new(BoxedNode::<u8>::new(255)))
        );
        assert!(singly_u8_linked_list.head.unwrap().next.unwrap().next.is_none());
    }

    #[test]
    fn third_add_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();
        let mut first_node = BoxedNode::<u8>::new(128);
        let mut second_node = BoxedNode::<u8>::new(255);
        second_node.next = Some(Box::new(BoxedNode::<u8>::new(0)));
        first_node.next = Some(Box::new(second_node.clone()));

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.size, 3);
        assert_eq!(singly_u8_linked_list.size(), 3);
        assert!(singly_u8_linked_list.head.is_some());
        assert_eq!(singly_u8_linked_list.head, Some(Box::new(first_node)));
        assert_eq!(singly_u8_linked_list.peek(), Some(&128));
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().data, 128);
        assert!(singly_u8_linked_list.head.as_ref().unwrap().next.is_some());
        assert_eq!(
            singly_u8_linked_list.head.as_ref().unwrap().next,
            Some(Box::new(second_node.clone()))
        );
        assert!(singly_u8_linked_list.head.as_ref().unwrap().next.as_ref().unwrap().next.is_some());
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().next.as_ref().unwrap().next, second_node.next);
    }

    #[test]
    fn clear_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();
        let mut first_node = BoxedNode::<u8>::new(128);
        let mut second_node = BoxedNode::<u8>::new(255);
        second_node.next = Some(Box::new(BoxedNode::<u8>::new(0)));
        first_node.next = Some(Box::new(second_node.clone()));

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);
        singly_u8_linked_list.clear();

        assert_eq!(singly_u8_linked_list.size, 0);
        assert_eq!(singly_u8_linked_list.size(), 0);
        assert!(singly_u8_linked_list.head.is_none());
    }

    #[test]
    fn remove_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();
        let mut first_node = BoxedNode::<u8>::new(255);
        let second_node = BoxedNode::<u8>::new(0);
        first_node.next = Some(Box::new(second_node.clone()));

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);
        singly_u8_linked_list.remove();

        assert_eq!(singly_u8_linked_list.size, 2);
        assert_eq!(singly_u8_linked_list.size(), 2);
        assert!(singly_u8_linked_list.head.is_some());
        assert_eq!(singly_u8_linked_list.head, Some(Box::new(first_node)));
        assert_eq!(singly_u8_linked_list.peek(), Some(&255));
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().data, 255);
        assert!(singly_u8_linked_list.head.as_ref().unwrap().next.is_some());
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().next, Some(Box::new(second_node)));
        assert!(singly_u8_linked_list.head.as_ref().unwrap().next.as_ref().unwrap().next.is_none());
        assert_eq!(singly_u8_linked_list.head.as_ref().unwrap().next.as_ref().unwrap().data, 0);
    }

    #[test]
    fn contains_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();
        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);

        assert!(singly_u8_linked_list.contains(128));
        assert!(singly_u8_linked_list.contains(255));
        assert!(singly_u8_linked_list.contains(0));
        assert!(!singly_u8_linked_list.contains(1));
    }

    #[test]
    fn index_of_instruction() {
        let mut singly_u8_linked_list = BoxedSinglyLinkedList::<u8>::new();
        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.index_of(128), Some(0));
        assert_eq!(singly_u8_linked_list.index_of(255), Some(1));
        assert_eq!(singly_u8_linked_list.index_of(0), Some(2));
        assert_eq!(singly_u8_linked_list.index_of(1), None);
    }

}
