use std::collections::btree_map::Entry;
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;

struct PriorityQueue<T> {
    heap: Vec<T>,
    map: BTreeMap<T, usize>,
}

impl<T: Copy + Ord> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T: Copy + Ord> From<&[T]> for PriorityQueue<T> {
    fn from(slice: &[T]) -> Self {
        let heap_size = slice.len();
        let mut priority_queue = Self::new(heap_size);

        for i in 0..heap_size {
            priority_queue.map.insert(slice[i].clone(), i);
            priority_queue.heap.push(slice[i]);
        }

        for i in usize::max(0, (heap_size / 2) - 1)..=0 {
            priority_queue.sink(i);
        }

        priority_queue
    }
}

impl<T: Copy + Ord> PriorityQueue<T> {
    fn new(size: usize) -> Self {
        Self {
            heap: Vec::with_capacity(size),
            map: BTreeMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn clear(&mut self) {
        self.heap.clear();
        self.map.clear();
    }

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn peel(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.heap[0])
    }

    fn poll(&mut self) -> Option<T>{
        self.remove_at(0)
    }

    fn contains(&self, elem: T) -> bool {
        self.map.contains_key(&elem)
    }

    fn add(&mut self, elem: T) {
        let heap_size = self.size();
        self.heap.push(elem);
        self.map.insert(elem, heap_size);
        self.swim(heap_size)
    }

    fn less(&self, i: usize, j: usize) -> bool
    where
        T: PartialOrd,
    {
        let node1 = &self.heap[i];
        let node2 = &self.heap[j];

        node1.le(node2)
    }

    fn swim(&mut self, mut index: usize)
    where
        T: PartialOrd,
    {
        let mut parent = Self::parent_index(index);

        while index > 0 && self.less(index, parent) {
            self.swap(parent, index);
            index = parent;
            parent = Self::parent_index(index);
        }
    }

    fn parent_index(index: usize) -> usize {
        if index > 0 { (index - 1) / 2 } else { 0 }
    }

    fn sink(&mut self, mut index: usize)
    where
        T: PartialOrd,
    {
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = left;

            if right < self.size() && self.less(right, left) {
                smallest = right;
            }

            if left >= self.size() || self.less(index, smallest) {
                break;
            }

            self.swap(smallest, index);
            index = smallest;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let i_elem = self.heap[i];
        let j_elem = self.heap[j];

        self.heap[i] = j_elem;
        self.heap[j] = i_elem;

        self.map_swap(i_elem, j_elem, i, j);
    }

    fn remove(&mut self, elem: T) -> bool {
        let possible_index = self.map.get(&elem);
        let x = possible_index.is_some();

        if let Some(index) = possible_index {
            self.remove_at(*index);
        }

        x
    }

    fn remove_at(&mut self, index: usize) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let new_heap_size = self.size() - 1; // TODO check if this should be decreased by one
        let removed_data = if let Some(removed_data_reference) = self.heap.get(index) {
            Some(removed_data_reference.clone())
        } else {
            None
        };
        // let removed_data = self.heap.get(index);
        self.swap(index, new_heap_size);

        self.heap.swap_remove(new_heap_size); // TODO check if this bamboozles everything
        self.map_remove(&removed_data.unwrap(), new_heap_size);

        if index == new_heap_size {
            return removed_data;
        }

        // let elem = self.heap.get(index);
        let elem1 = if let Some(removed_data_reference) = self.heap.get(index) {
            Some(removed_data_reference.clone())
        } else {
            None
        };

        // Try sinking element
        self.sink(index);

        let elem2 = if let Some(elem2) = self.heap.get(index) {
            Some(elem2.clone())
        } else {
            None
        };

        if elem2 == elem1 {
            self.swim(index);
        }

        removed_data
    }

    fn map_add(&mut self, value: T, index: usize) {
        // let set = self.map.get(value);
        self.map.entry(value)
            .and_modify(|x| *x = index) // TODO check if this line is needed
            .or_insert(index);
    }

    fn map_remove(&mut self, value: &T, index: usize) {
        self.map.remove(value);
    }

    fn map_get(&mut self, value: T) -> T {
        *self.map.entry(value).key()
    }

    fn map_swap(&mut self, i_elem: T, j_elem: T, i: usize, j: usize) {
        self.map.entry(i_elem).and_modify(|x| *x = j);
        self.map.entry(j_elem).and_modify(|x| *x = i);
    }
}

mod tests{
    use super::*;

    // called this method with k = 0 to start at the root
    fn is_min_heap<T: Copy + Ord>(k: usize, heap: &PriorityQueue<T>) -> bool {

        // if we are outside the bounds of the heap return true
        if k >= heap.size() {
            return true;
        }

        let left = 2 * k + 1;
        let right = 2 * k + 2;

        // make sure that the current node k is less than
        // both of its children left, and right if they exist
        // return false otherwise to indicate an invalid heap
        if left < heap.size() && !heap.less(k, left) {
            return false;
        }
        if right < heap.size() && !heap.less(k, right){
            return false;
        }

        return is_min_heap(left, heap) && is_min_heap(right, heap);
    }

    #[test]
    fn instantiation() {
        use std::any::{Any, TypeId};
        assert_eq!(
            PriorityQueue::<u8>::new(1).type_id(),
            TypeId::of::<PriorityQueue<u8>>()
        );
        assert_eq!(
            PriorityQueue::<u16>::new(1).type_id(),
            TypeId::of::<PriorityQueue<u16>>()
        );
    }

    #[test]
    fn default_instantiation(){
        let priority_queue = PriorityQueue::<u8>::default();

        assert_eq!(priority_queue.map.len(), 0);
        assert_eq!(priority_queue.heap.len(), 0);
        assert_eq!(priority_queue.size(), 0);
        assert!(priority_queue.is_empty());
    }

    #[test]
    fn size_after_operations() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new(1);
        assert_eq!(pq.size(), 0);
        pq.add(42);
        assert_eq!(pq.size(), 1);
        pq.add(7);
        assert_eq!(pq.size(), 2);
        pq.poll();
        assert_eq!(pq.size(), 1);
        pq.clear();
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn instantiation_from_slice() {
        let slice = &[3, 1, 4, 1, 5];
        let pq: PriorityQueue<i32> = slice[..].into();

        assert_eq!(pq.size(), 5);
        assert!(pq.contains(3));
        assert!(pq.contains(1));
        assert!(pq.contains(4));
        assert!(pq.contains(5));
    }

    #[test]
    fn poll_empty_queue() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new(1);
        assert_eq!(pq.poll(), None);
    }

    #[test]
    fn poll_single_element() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new(1);
        pq.add(42);
        assert_eq!(pq.size(), 1);
        assert_eq!(pq.poll(), Some(42));
        assert!(pq.is_empty());
    }

    #[test]
    fn poll_instruction() {
        let mut pq = PriorityQueue::new(10);
        pq.add(10);
        pq.add(20);
        pq.add(5);

        assert_eq!(pq.poll(), Some(5));
        assert_eq!(pq.size(), 2);
        assert!(!pq.contains(5));
    }

    #[test]
    fn size_check_after_operations() {
        let mut pq = PriorityQueue::new(10);
        pq.add(10);
        pq.add(20);
        pq.add(5);

        assert_eq!(pq.size(), 3);

        pq.poll();
        assert_eq!(pq.size(), 2);

        pq.clear();
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn remove_nonexistent_element() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new(1);
        pq.add(42);
        assert!(!pq.remove(99));
    }

    #[test]
    fn empty_check() {
        let mut pq = PriorityQueue::new(10);
        assert!(pq.is_empty());

        pq.add(10);
        assert!(!pq.is_empty());

        pq.clear();
        assert!(pq.is_empty());
    }

    #[test]
    fn is_empty_after_operations() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new(1);
        assert!(pq.is_empty());
        pq.add(42);
        assert!(!pq.is_empty());
        pq.poll();
        assert!(pq.is_empty());
        pq.add(7);
        pq.clear();
        assert!(pq.is_empty());
    }
}
