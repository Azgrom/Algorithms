use super::stack_pointed_singly_linked_list::StackPointedSinglyLinkedList;

#[derive(Clone, PartialEq)]
struct Node<T>
where
    T: Clone,
{
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>, elem: T) -> Box<Node<T>> {
        Self {
            data: elem,
            left,
            right,
        }
        .into()
    }
}

struct BinarySearchTree<T>
where
    T: Clone,
{
    node_count: usize,
    root: Option<Box<Node<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Clone + PartialOrd,
{
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.node_count
    }

    fn add(&mut self, elem: T) -> bool {
        return if self.contains(&elem) {
            false
        } else {
            let moved_root = self.root.take();
            self.root = Some(Self::add_node(moved_root, elem));
            self.node_count += 1;

            true
        };
    }

    fn add_node(mut node: Option<Box<Node<T>>>, elem: T) -> Box<Node<T>> {
        return if let Some(mut data) = node {
            if elem.lt(&data.data) {
                let moved_left = data.left.take();
                data.left = Some(Self::add_node(moved_left, elem));
            } else {
                let moved_left = data.right.take();
                data.right = Some(Self::add_node(moved_left, elem));
            }

            data
        } else {
            Node::<T>::new(None, None, elem)
        };
    }

    fn remove(&mut self, elem: T) -> bool {
        if self.contains(&elem) {
            self.root = Self::remove_node(&elem, &mut self.root);
            self.node_count -= 1;

            return true;
        }

        return false;
    }

    fn remove_node(elem: &T, option_node: &mut Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        option_node.as_mut().and_then(|boxed_node| {
            if elem.lt(&boxed_node.data) {
                boxed_node.left = Self::remove_node(elem, &mut boxed_node.left);
            } else if elem.gt(&boxed_node.data) {
                boxed_node.right = Self::remove_node(elem, &mut boxed_node.right);
            } else {
                if boxed_node.left.is_none() {
                    return boxed_node.right.take();
                } else if boxed_node.right.is_none() {
                    return boxed_node.left.take();
                } else if let Some(min_right_node) =
                    Self::find_min(boxed_node.right.as_ref().map(|right| &**right))
                {
                    boxed_node.data = min_right_node.data.clone();
                    boxed_node.right = Self::remove_node(&boxed_node.data, &mut boxed_node.right);
                }
            }

            Some(boxed_node.clone())
        })
    }

    fn find_min(node: Option<&Node<T>>) -> Option<&Node<T>> {
        node.and_then(|mut x| {
            while let Some(left_node) = x.left.as_ref().map(|left| &**left) {
                x = &*left_node;
            }

            Some(x)
        })
    }

    fn find_max(node: Option<&Node<T>>) -> Option<&Node<T>> {
        node.and_then(|mut x| {
            while let Some(right_node) = x.right.as_ref().map(|right| &**right) {
                x = &*right_node;
            }

            Some(x)
        })
    }

    fn contains(&self, elem: &T) -> bool {
        Self::contains_node(elem, self.root.as_ref().map(|root| &**root))
    }

    fn contains_node(elem: &T, node: Option<&Node<T>>) -> bool {
        return match node {
            None => false,
            Some(node) => {
                return match &node.data {
                    data if elem.lt(data) => {
                        Self::contains_node(elem, node.left.as_ref().map(|left| &**left))
                    }
                    data if elem.gt(data) => {
                        Self::contains_node(elem, node.right.as_ref().map(|right| &**right))
                    }
                    _ => true,
                }
            }
        };
    }

    fn height(&self) -> usize {
        Self::height_node(self.root.as_ref().map(|root_node| &**root_node))
    }

    fn height_node(node: Option<&Node<T>>) -> usize {
        return match node {
            None => 0,
            Some(node) => {
                let right_height = Self::height_node(node.right.as_ref().map(|x| &**x));
                let left_height = Self::height_node(node.left.as_ref().map(|x| &**x));
                usize::max(left_height, right_height) + 1
            }
        };
    }
}

struct PreOrderTreeTraversal<'a, T>
where T: Clone + PartialOrd
{
    tree: &'a BinarySearchTree<T>,
    stack: StackPointedSinglyLinkedList<&'a Node<T>>
}

impl<'a, T> PreOrderTreeTraversal<'a, T>
where T: Clone + PartialOrd
{
    fn new(tree: &'a BinarySearchTree<T>) -> PreOrderTreeTraversal<T> {
        if tree.is_empty() {
            panic!("Cannot create Iterator from an empty Tree")
        }

        let mut stack_pointed_singly_linked_list = StackPointedSinglyLinkedList::<&Node<T>>::new();
        stack_pointed_singly_linked_list.push(tree.root.as_ref().map(|x| &**x).unwrap());

        Self {
            tree,
            stack: stack_pointed_singly_linked_list
        }
    }
}

impl<T> Iterator for PreOrderTreeTraversal<'_, T>
where T: Clone + PartialOrd
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop();

        if node.is_none() || self.stack.is_empty() {
           return None;
        }

        node.and_then(|x| {
            if let Some(right_node) = &x.right {
                self.stack.push(&**right_node);
            }
            if let Some(left_node) = &x.left {
                self.stack.push(&**left_node);
            }

            Some(&x.data)
        }).cloned()
    }
}
