#[derive(Debug)]
struct HeapTree {
    tree: Vec<i32>,
}

impl HeapTree {
    
    fn new(values: &[i32]) -> HeapTree {
        HeapTree {
            tree: values.to_vec(),
        }
    }

    fn build_max_heapify(&mut self) {
        let mut counter = Some(self.size() / 2);
        while let Some(i) = counter {
            self.max_heapify(Some(i));
            match i > 0 {
                true => counter = Some(i - 1),
                false => break,
            }
        }
    }

    fn max_heapify(&mut self, index: Option<usize>) {
        let left_child = self.left_child(index);
        let right_child = self.right_child(index);

        let mut largest = index;

        if largest.is_none() {
            return;
        }

        if left_child.is_some() && self.tree[largest.unwrap()] < self.tree[left_child.unwrap()] {
            largest = left_child;
        }
        
        if right_child.is_some() && self.tree[largest.unwrap()] < self.tree[right_child.unwrap()] {
            largest = right_child;
        }

        if largest.ne(&index) {
            self.tree.swap(largest.unwrap(), index.unwrap());
            self.max_heapify(largest);
        }
    }

    fn left_child(&self, index: Option<usize>) -> Option<usize> {
        let index = match index {
            None => return None,
            Some(u) => u,
        };
        match (2 * index + 1).le(&(self.size() - 1)) {
            true => Some(2 * index + 1),
            false => None,
        }
    }

    fn right_child(&self, index: Option<usize>) -> Option<usize> {
        let index = match index {
            None => return None,
            Some(u) => u,
        };
        match (2 * index + 2).le(&(self.size() - 1)) {
            true => Some(2 * index + 2),
            false => None,
        }
    }

    fn size(&self) -> usize {
        self.tree.len()
    } 
}

fn main() {

    let y = [11, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut x = HeapTree::new(&y);
    x.tree.push(10);
    x.build_max_heapify();
    println!("{:?}", x);
}
