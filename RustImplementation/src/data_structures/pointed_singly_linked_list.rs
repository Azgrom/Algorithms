use std::alloc::{Allocator, Global};

#[derive(Clone, Debug, PartialEq)]
struct PointedNode<T> {
    data: T,
    next: *mut PointedNode<T>,
}

impl<T> PointedNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: core::ptr::null_mut(),
        }
    }
}

struct PointedSinglyLinkedList<T> {
    size: usize,
    head: *mut PointedNode<T>,
}

impl<T: PartialEq> PointedSinglyLinkedList<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: core::ptr::null_mut(),
        }
    }

    fn clear(&mut self) {
        self.head = core::ptr::null_mut();
        self.size = 0;
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
            unsafe {
                core::ptr::write(self.head, PointedNode::new(data));
            }
            return;
        }

        let mut current = self.head;
        unsafe {
            while !current.read().next.is_null() {
                let x = current.read().next;
                current = x;
            }
        }

        unsafe {
            (*(*(&current))).next = new_node;
            (*(*(&current))).next.write(PointedNode::new(data));
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
            let current_node = current_head.read();

            if !current_node.next.is_null() {
                let next_node = current_node.next.read();
                self.head.write(next_node);
            } else {
                self.head = core::ptr::null_mut();
            }

            Some(current_node.data)
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
            PointedSinglyLinkedList::<u8>::new().type_id(),
            TypeId::of::<PointedSinglyLinkedList<u8>>()
        );
        assert_ne!(
            PointedSinglyLinkedList::<u8>::new().type_id(),
            TypeId::of::<PointedSinglyLinkedList<u16>>()
        );
    }

    #[test]
    fn default_instantiation() {
        let singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();

        assert_eq!(singly_u8_linked_list.size, 0);
        assert_eq!(singly_u8_linked_list.size(), 0);
        assert!(singly_u8_linked_list.head.is_null());
        assert_eq!(singly_u8_linked_list.peek(), None);
    }

    #[test]
    fn first_peek_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();

        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.size, 1);
        assert_eq!(singly_u8_linked_list.size(), 1);
        assert!(!singly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read() },
            PointedNode::<u8>::new(0)
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&0));
    }

    #[test]
    fn first_add_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();

        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.size, 1);
        assert_eq!(singly_u8_linked_list.size(), 1);
        assert!(!singly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read() },
            PointedNode::<u8>::new(0)
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&0));
        assert_eq!(unsafe { singly_u8_linked_list.head.read() }.data, 0);
        assert!(unsafe { singly_u8_linked_list.head.read() }.next.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read() }.next,
            core::ptr::null_mut()
        );
    }

    #[test]
    fn second_add_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();
        let size_of_t_node = core::mem::size_of::<PointedNode<u8>>();
        let align_of_t_node = core::mem::align_of::<PointedNode<u8>>();
        let layout = core::alloc::Layout::from_size_align(size_of_t_node, align_of_t_node).unwrap();
        let mut first_node = PointedNode::<u8>::new(128);
        first_node.next = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<u8>>()
            .as_ptr();
        let second_node = PointedNode::<u8>::new(255);
        unsafe {
            first_node.next.write(second_node.clone());
        }

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);

        assert_eq!(singly_u8_linked_list.size, 2);
        assert_eq!(singly_u8_linked_list.size(), 2);
        assert!(!singly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read() }.data,
            first_node.data
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&128));
        assert_eq!(unsafe { singly_u8_linked_list.head.read() }.data, 128);
        assert!(!unsafe { singly_u8_linked_list.head.read() }.next.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read().next.read() }.data,
            second_node.data
        );
        assert!(unsafe { singly_u8_linked_list.head.read().next.read() }
            .next
            .is_null());
    }

    #[test]
    fn third_add_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();
        let size_of_t_node = core::mem::size_of::<PointedNode<u8>>();
        let align_of_t_node = core::mem::align_of::<PointedNode<u8>>();
        let layout = core::alloc::Layout::from_size_align(size_of_t_node, align_of_t_node).unwrap();
        let mut first_node = PointedNode::<u8>::new(128);
        first_node.next = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<u8>>()
            .as_ptr();
        let mut second_node = PointedNode::<u8>::new(255);
        second_node.next = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<u8>>()
            .as_ptr();
        unsafe {
            second_node.next.write(PointedNode::<u8>::new(0));
            first_node.next.write(second_node.clone());
        }

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.size, 3);
        assert_eq!(singly_u8_linked_list.size(), 3);
        assert!(!singly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read() }.data,
            first_node.data
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&128));
        assert_eq!(unsafe { singly_u8_linked_list.head.read() }.data, 128);
        assert!(!unsafe { singly_u8_linked_list.head.read() }.next.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read().next.read() }.data,
            second_node.data
        );
        assert!(!unsafe { singly_u8_linked_list.head.read().next.read() }
            .next
            .is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read().next.read().next.read() }.data,
            unsafe { second_node.next.read() }.data
        );
    }

    #[test]
    fn clear_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();
        let size_of_t_node = core::mem::size_of::<PointedNode<u8>>();
        let align_of_t_node = core::mem::align_of::<PointedNode<u8>>();
        let layout = core::alloc::Layout::from_size_align(size_of_t_node, align_of_t_node).unwrap();
        let mut first_node = PointedNode::<u8>::new(128);
        first_node.next = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<u8>>()
            .as_ptr();
        let mut second_node = PointedNode::<u8>::new(255);
        second_node.next = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<u8>>()
            .as_ptr();
        unsafe {
            second_node.next.write(PointedNode::<u8>::new(0));
            first_node.next.write(second_node.clone());
        }

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);
        singly_u8_linked_list.clear();

        assert_eq!(singly_u8_linked_list.size, 0);
        assert_eq!(singly_u8_linked_list.size(), 0);
        assert!(singly_u8_linked_list.head.is_null());
    }

    #[test]
    fn remove_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();
        let size_of_t_node = core::mem::size_of::<PointedNode<u8>>();
        let align_of_t_node = core::mem::align_of::<PointedNode<u8>>();
        let layout = core::alloc::Layout::from_size_align(size_of_t_node, align_of_t_node).unwrap();
        let mut first_node = PointedNode::<u8>::new(255);
        first_node.next = Global::default()
            .allocate(layout)
            .unwrap()
            .cast::<PointedNode<u8>>()
            .as_ptr();
        let second_node = PointedNode::<u8>::new(0);
        unsafe {
            first_node.next.write(second_node.clone());
        }

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);
        let removed_item = singly_u8_linked_list.remove();

        assert_eq!(removed_item, Some(128));
        assert_eq!(singly_u8_linked_list.size, 2);
        assert_eq!(singly_u8_linked_list.size(), 2);
        assert!(!singly_u8_linked_list.head.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read() }.data,
            first_node.data
        );
        assert_eq!(singly_u8_linked_list.peek(), Some(&255));
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read().next.read() }.data,
            second_node.data
        );
        assert!(!unsafe { singly_u8_linked_list.head.read() }.next.is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read().next.read() },
            second_node
        );
        assert!(unsafe { singly_u8_linked_list.head.read().next.read() }
            .next
            .is_null());
        assert_eq!(
            unsafe { singly_u8_linked_list.head.read().next.read() }.data,
            0
        );
        assert_eq!(singly_u8_linked_list.remove(), Some(255));
        assert_eq!(singly_u8_linked_list.remove(), Some(0));
    }

    #[test]
    fn contains_instruction() {
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();

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
        let mut singly_u8_linked_list = PointedSinglyLinkedList::<u8>::new();

        singly_u8_linked_list.add(128);
        singly_u8_linked_list.add(255);
        singly_u8_linked_list.add(0);

        assert_eq!(singly_u8_linked_list.index_of(128), Some(0));
        assert_eq!(singly_u8_linked_list.index_of(255), Some(1));
        assert_eq!(singly_u8_linked_list.index_of(0), Some(2));
        assert_eq!(singly_u8_linked_list.index_of(1), None);
    }
}
