    use std::collections::BTreeMap;

struct PriorityQueue<T> {
    heap: Vec<T>,
    map: BTreeMap<T, usize>,
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T> From<&[T]> for PriorityQueue<T> {
    fn from(slice: &[T]) -> Self {
        let heap_size = slice.len();
        let mut priority_queue = Self::new(heap_size);

        for i in 0..heap_size {
            priority_queue.map.insert(&slice[i], i);
            priority_queue.heap.push(&slice[i]);
        }

        for i in usize::max(0, (heap_size / 2) - 1)..=0 {
            priority_queue.sink(i);
        }

        priority_queue
    }
}

impl<T> PriorityQueue<T> {
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

    fn poll(&self) {
        self.remove_at(0)
    }

    fn contains(&self, elem: T) -> bool {
        self.map.contains_key(elem)
    }

    fn add(&mut self, elem: T) {
        let heap_size = self.size();
        self.heap.push(&elem);
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
        let mut parent = (index - 1) / 2;

        while index > 0 && self.less(index, parent) {
            self.swap(parent, index);
            index = parent;
            parent = (index - 1) / 2;
        }
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
        let i_elem = &self.heap[i];
        let j_elem = &self.heap[j];

        self.heap[i] = j_elem;
        self.heap[j] = i_elem;

        self.map_swap(i_elem, j_elem, i, j);
    }

    fn remove(&mut self, elem: T) -> bool {
        let possible_index = self.map.get(elem);

        if let Some(index) = possible_index {
            self.remove_at(index);
        }

        possible_index.is_some()
    }
}
