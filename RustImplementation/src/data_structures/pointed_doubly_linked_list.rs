use std::alloc::{Allocator, Global};

#[derive(Clone, Debug, PartialEq)]
struct PointedNode<T> {
    data: T,
    prev: *mut PointedNode<T>,
    next: *mut PointedNode<T>,
}

impl<T> PointedNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        }
    }
}

struct PointedDoublyLinkedList<T> {
    size: usize,
    head: *mut PointedNode<T>,
    tail: *mut PointedNode<T>,
}

impl<T: PartialEq> PointedDoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
        }
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add(&mut self, data: T) {
        let size_of_t_node = core::mem::size_of::<PointedNode<T>>();
        let align_of_t_node = core::mem::align_of::<PointedNode<T>>();
        let layout = core::alloc::Layout::from_size_align(size_of_t_node, align_of_t_node).unwrap();
        let new_node = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<T>>()
            .as_ptr();

        self.size += 1;

        if self.head.is_null() {
            self.head = new_node;
            self.tail = new_node;
            unsafe {
                core::ptr::write(self.head, PointedNode::new(data));
            }
            return;
        }

        unsafe {
            let previous_tail = self.tail;
            (*(*(&self.tail))).next = new_node;
            self.tail = new_node;
            self.tail.write(PointedNode::new(data));
            (*(*(&self.tail))).prev = previous_tail;
        }
    }

    fn peek(&self) -> Option<&T> {
        return if self.head.is_null() {
            None
        } else {
            Some(&(unsafe { self.head.as_mut() }.unwrap().data))
        };
    }

    fn remove(&mut self) -> Option<T> {
        let current_head = self.head;

        if current_head.is_null() {
            return None;
        }

        unsafe {
            self.size -= 1;
            let current_node = current_head;

            if !(*(*(&current_node))).next.is_null() {
                let new_head = (*(*(&current_node))).next;
                self.head = new_head;
                (*(*(&self.head))).prev = core::ptr::null_mut();
            } else {
                self.head = core::ptr::null_mut();
                self.tail = core::ptr::null_mut();
            }

            Some(current_node.read().data)
        }
    }

    fn contains(&self, data: T) -> bool {
        unsafe {
            let mut current = self.head;
            while !current.is_null() {
                if (*(*(&current))).data == data {
                    return true;
                }
                current = (*(*(&current))).next;
            }
        }

        false
    }

    fn index_of(&self, data: T) -> Option<usize> {
        let mut current = self.head;
        let mut i = 0;

        unsafe {
            while !current.is_null() {
                if (*(*(&current))).data == data {
                    return Some(i);
                }

                current = (*(*(&current))).next;
                i += 1;
            }
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
            PointedDoublyLinkedList::<u8>::new().type_id(),
            TypeId::of::<PointedDoublyLinkedList<u8>>()
        );
        assert_ne!(
            PointedDoublyLinkedList::<u8>::new().type_id(),
            TypeId::of::<PointedDoublyLinkedList<u16>>()
        );
    }

    #[test]
    fn default_instantiation() {
        let doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        assert_eq!(doubly_u8_linked_list.size, 0);
        assert_eq!(doubly_u8_linked_list.size(), 0);
        assert!(doubly_u8_linked_list.head.is_null());
        assert!(doubly_u8_linked_list.tail.is_null());
        assert_eq!(doubly_u8_linked_list.peek(), None);
    }

    #[test]
    fn first_peek_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(0);

        assert_eq!(doubly_u8_linked_list.size, 1);
        assert_eq!(doubly_u8_linked_list.size(), 1);
        assert!(!doubly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read() },
            PointedNode::<u8>::new(0)
        );
        assert_eq!(doubly_u8_linked_list.peek(), Some(&0));
    }

    #[test]
    fn first_add_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(0);

        assert_eq!(doubly_u8_linked_list.size, 1);
        assert_eq!(doubly_u8_linked_list.size(), 1);
        assert!(!doubly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read() },
            PointedNode::<u8>::new(0)
        );
        assert_eq!(doubly_u8_linked_list.peek(), Some(&0));
        assert_eq!(unsafe { doubly_u8_linked_list.head.read() }.data, 0);
        assert!(unsafe { doubly_u8_linked_list.head.read() }.next.is_null());
        assert!(unsafe { doubly_u8_linked_list.head.read() }.prev.is_null());
    }

    #[test]
    fn second_add_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(128);
        doubly_u8_linked_list.add(255);

        assert_eq!(doubly_u8_linked_list.size, 2);
        assert_eq!(doubly_u8_linked_list.size(), 2);
        assert!(!doubly_u8_linked_list.head.is_null());
        assert_eq!(doubly_u8_linked_list.peek(), Some(&128));
        assert_eq!(unsafe { doubly_u8_linked_list.head.read() }.data, 128);
        assert!(!unsafe { doubly_u8_linked_list.head.read() }.next.is_null());
        assert!(unsafe { doubly_u8_linked_list.head.read() }.prev.is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read() }.data,
            255
        );
        assert!(unsafe { doubly_u8_linked_list.head.read().next.read() }
            .next
            .is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read() }.prev,
            doubly_u8_linked_list.head
        );
    }

    #[test]
    fn third_add_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(128);
        doubly_u8_linked_list.add(255);
        doubly_u8_linked_list.add(0);

        assert_eq!(doubly_u8_linked_list.size, 3);
        assert_eq!(doubly_u8_linked_list.size(), 3);
        assert!(!doubly_u8_linked_list.head.is_null());
        assert_eq!(doubly_u8_linked_list.peek(), Some(&128));
        assert_eq!(unsafe { doubly_u8_linked_list.head.read() }.data, 128);
        assert!(!unsafe { doubly_u8_linked_list.head.read() }.next.is_null());
        assert!(unsafe { doubly_u8_linked_list.head.read() }.prev.is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read() }.data,
            255
        );
        assert!(!unsafe { doubly_u8_linked_list.head.read().next.read() }
            .next
            .is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read().next.read() }.data,
            0
        );
        assert!(
            unsafe { doubly_u8_linked_list.head.read().next.read().next.read() }
                .next
                .is_null()
        );
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read().next.read() }.prev,
            unsafe { doubly_u8_linked_list.head.read().next }
        );
    }

    #[test]
    fn clear_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(128);
        doubly_u8_linked_list.add(255);
        doubly_u8_linked_list.add(0);

        assert_eq!(doubly_u8_linked_list.size, 3);
        assert_eq!(doubly_u8_linked_list.peek(), Some(&128));

        doubly_u8_linked_list.clear();

        assert_eq!(doubly_u8_linked_list.size, 0);
        assert_eq!(doubly_u8_linked_list.size(), 0);
        assert!(doubly_u8_linked_list.head.is_null());
        assert!(doubly_u8_linked_list.tail.is_null());
    }

    #[test]
    fn remove_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(128);
        doubly_u8_linked_list.add(255);
        doubly_u8_linked_list.add(0);
        let removed_item = doubly_u8_linked_list.remove();

        assert_eq!(removed_item, Some(128));
        assert_eq!(doubly_u8_linked_list.size, 2);
        assert_eq!(doubly_u8_linked_list.size(), 2);
        assert!(!doubly_u8_linked_list.head.is_null());
        assert_eq!(doubly_u8_linked_list.peek(), Some(&255));
        assert_eq!(unsafe { doubly_u8_linked_list.head.read() }.data, 255);
        assert!(!unsafe { doubly_u8_linked_list.head.read() }.next.is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read() }.data,
            0
        );
        assert!(unsafe { doubly_u8_linked_list.head.read().next.read() }
            .next
            .is_null());
        assert_eq!(
            unsafe { doubly_u8_linked_list.head.read().next.read() }.prev,
            doubly_u8_linked_list.head
        );

        assert_eq!(doubly_u8_linked_list.remove(), Some(255));
        assert_eq!(doubly_u8_linked_list.remove(), Some(0));
        assert_eq!(doubly_u8_linked_list.remove(), None);
    }

    #[test]
    fn contains_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(128);
        doubly_u8_linked_list.add(255);
        doubly_u8_linked_list.add(0);

        assert!(doubly_u8_linked_list.contains(128));
        assert!(doubly_u8_linked_list.contains(255));
        assert!(doubly_u8_linked_list.contains(0));
        assert!(!doubly_u8_linked_list.contains(1));
    }

    #[test]
    fn index_of_instruction() {
        let mut doubly_u8_linked_list = PointedDoublyLinkedList::<u8>::new();

        doubly_u8_linked_list.add(128);
        doubly_u8_linked_list.add(255);
        doubly_u8_linked_list.add(0);

        assert_eq!(doubly_u8_linked_list.index_of(128), Some(0));
        assert_eq!(doubly_u8_linked_list.index_of(255), Some(1));
        assert_eq!(doubly_u8_linked_list.index_of(0), Some(2));
        assert_eq!(doubly_u8_linked_list.index_of(1), None);
    }
}
