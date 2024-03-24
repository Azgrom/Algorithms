use core::alloc::Allocator;
use core::ptr::NonNull;
use std::alloc::Global;

struct DynamicArray {
    arr: *mut u8,
    capacity: usize,
    length: usize,
}

impl DynamicArray {
    fn new() -> Self {
        Self {
            arr: core::ptr::null_mut(),
            capacity: 0,
            length: 0,
        }
    }

    fn push(&mut self, value: u8) {
        if self.length >= self.capacity {
            self.grow();
        }

        unsafe {
            let ptr = self.arr.add(self.length);
            core::ptr::write(ptr, value);
        }

        self.length += 1;
    }

    fn get(&self, index: usize) -> Option<&u8> {
        if index < self.length {
            unsafe { Some(&*self.arr.add(index)) }
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let size_of_t = core::mem::size_of::<u8>();
        let align_of_t = core::mem::align_of::<u8>();
        let layout =
            core::alloc::Layout::from_size_align(size_of_t * new_capacity, align_of_t).unwrap();
        let global = Global::default();
        let new_data = global.allocate(layout).unwrap().as_ptr() as *mut u8;

        for i in 0..self.length {
            unsafe {
                let ptr = new_data.add(i);
                core::ptr::write(ptr, core::ptr::read(self.arr.add(i)));
            }
        }

        if !self.arr.is_null() {
            let layout =
                core::alloc::Layout::from_size_align(size_of_t * self.capacity, align_of_t)
                    .unwrap();

            unsafe { global.deallocate(NonNull::new(self.arr).unwrap(), layout) }
        }

        self.arr = new_data;
        self.capacity = new_capacity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        use std::any::{Any, TypeId};

        assert_eq!(DynamicArray::new().type_id(), TypeId::of::<DynamicArray>());
    }

    #[test]
    fn default_length_and_capacity() {
        let dynamic_array = DynamicArray::new();

        assert_eq!(dynamic_array.length, 0);
        assert_eq!(dynamic_array.len(), 0);
        assert_eq!(dynamic_array.capacity, 0);
        assert_eq!(dynamic_array.capacity(), 0);
        assert_eq!(dynamic_array.arr, core::ptr::null_mut());
        assert_eq!(dynamic_array.get(0), None);
        assert_eq!(NonNull::new(dynamic_array.arr), None);
    }

    #[test]
    fn first_grow_instruction() {
        let mut dynamic_array = DynamicArray::new();

        dynamic_array.grow();
        let non_null_pointer_state = NonNull::new(dynamic_array.arr);

        assert_eq!(dynamic_array.length, 0);
        assert_eq!(dynamic_array.len(), 0);
        assert_eq!(dynamic_array.capacity, 1);
        assert_eq!(dynamic_array.capacity(), 1);
        assert_eq!(dynamic_array.get(0), None);
        assert!(non_null_pointer_state.is_some());
        assert_eq!(
            unsafe {
                NonNull::slice_from_raw_parts(
                    non_null_pointer_state.unwrap(),
                    dynamic_array.capacity(),
                )
                .as_ref()
                .len()
            },
            1
        );
    }

    #[test]
    fn second_grow_instruction() {
        let mut dynamic_array = DynamicArray::new();

        dynamic_array.grow();
        dynamic_array.grow();
        let non_null_pointer_state = NonNull::new(dynamic_array.arr);

        assert_eq!(dynamic_array.length, 0);
        assert_eq!(dynamic_array.len(), 0);
        assert_eq!(dynamic_array.capacity, 2);
        assert_eq!(dynamic_array.capacity(), 2);
        assert_eq!(dynamic_array.get(0), None);
        assert!(non_null_pointer_state.is_some());
        assert_eq!(
            unsafe {
                NonNull::slice_from_raw_parts(
                    non_null_pointer_state.unwrap(),
                    dynamic_array.capacity(),
                )
                .as_ref()
                .len()
            },
            2
        );
    }

    #[test]
    fn third_grow_instruction() {
        let mut dynamic_array = DynamicArray::new();

        dynamic_array.grow();
        dynamic_array.grow();
        dynamic_array.grow();
        let non_null_pointer_state = NonNull::new(dynamic_array.arr);

        assert_eq!(dynamic_array.length, 0);
        assert_eq!(dynamic_array.len(), 0);
        assert_eq!(dynamic_array.capacity, 4);
        assert_eq!(dynamic_array.capacity(), 4);
        assert_eq!(dynamic_array.get(0), None);
        assert!(non_null_pointer_state.is_some());
        assert_eq!(
            unsafe {
                NonNull::slice_from_raw_parts(
                    non_null_pointer_state.unwrap(),
                    dynamic_array.capacity(),
                )
                .as_ref()
                .len()
            },
            4
        );
    }

    #[test]
    fn first_push_instruction() {
        let mut dynamic_array = DynamicArray::new();

        dynamic_array.push(128);
        let non_null_pointer_state = NonNull::new(dynamic_array.arr);

        assert_eq!(dynamic_array.length, 1);
        assert_eq!(dynamic_array.len(), 1);
        assert_eq!(dynamic_array.capacity, 1);
        assert_eq!(dynamic_array.capacity(), 1);
        assert_eq!(dynamic_array.get(0), Some(&128));
        assert!(non_null_pointer_state.is_some());
        assert_eq!(
            unsafe {
                NonNull::slice_from_raw_parts(non_null_pointer_state.unwrap(), dynamic_array.len())
                    .as_ref()
                    .len()
            },
            1
        );
    }

    #[test]
    fn second_push_instruction() {
        let mut dynamic_array = DynamicArray::new();

        dynamic_array.push(128);
        dynamic_array.push(255);
        let non_null_pointer_state = NonNull::new(dynamic_array.arr);

        assert_eq!(dynamic_array.length, 2);
        assert_eq!(dynamic_array.len(), 2);
        assert_eq!(dynamic_array.capacity, 2);
        assert_eq!(dynamic_array.capacity(), 2);
        assert_eq!(dynamic_array.get(0), Some(&128));
        assert_eq!(dynamic_array.get(1), Some(&255));
        assert_eq!(
            unsafe {
                NonNull::slice_from_raw_parts(non_null_pointer_state.unwrap(), dynamic_array.len())
                    .as_ref()
                    .len()
            },
            2
        );
    }

    #[test]
    fn third_push_instruction() {
        let mut dynamic_array = DynamicArray::new();

        dynamic_array.push(128);
        dynamic_array.push(255);
        dynamic_array.push(128);
        let non_null_pointer_state = NonNull::new(dynamic_array.arr);

        assert_eq!(dynamic_array.length, 3);
        assert_eq!(dynamic_array.len(), 3);
        assert_eq!(dynamic_array.capacity, 4);
        assert_eq!(dynamic_array.capacity(), 4);
        assert_eq!(dynamic_array.get(0), Some(&128));
        assert_eq!(dynamic_array.get(1), Some(&255));
        assert_eq!(dynamic_array.get(2), Some(&128));
        assert_eq!(
            unsafe {
                NonNull::slice_from_raw_parts(non_null_pointer_state.unwrap(), dynamic_array.len())
                    .as_ref()
                    .len()
            },
            3
        );
    }
}
