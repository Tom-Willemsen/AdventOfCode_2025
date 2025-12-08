pub struct UfNode {
    items: Vec<usize>,
    sizes: Vec<usize>,
    num_sets: usize,
}

impl UfNode {
    pub fn new(items: Vec<usize>) -> UfNode {
        UfNode {
            sizes: vec![1; items.len()],
            num_sets: items.len(),
            items,
        }
    }

    pub fn find_representative(&mut self, elem: usize) -> Option<usize> {
        let parent = *self.items.get(elem)?;

        if parent == elem {
            Some(parent)
        } else {
            let v = self.find_representative(parent)?;
            *self.items.get_mut(elem)? = v;
            Some(v)
        }
    }

    pub fn union_sets(&mut self, a: usize, b: usize) -> Option<()> {
        let a = self.find_representative(a)?;
        let b = self.find_representative(b)?;

        if a != b {
            let a_size = self.sizes.get(a)?;
            let b_size = self.sizes.get(b)?;

            if a_size < b_size {
                *self.items.get_mut(a)? = b;
                *self.sizes.get_mut(b)? = a_size + b_size;
                *self.sizes.get_mut(a)? = 0;
            } else {
                *self.items.get_mut(b)? = a;
                *self.sizes.get_mut(a)? = a_size + b_size;
                *self.sizes.get_mut(b)? = 0;
            }
            self.num_sets -= 1;
        }
        Some(())
    }

    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    pub fn sizes(&self) -> Vec<usize> {
        self.sizes.iter().copied().filter(|&n| n > 0).collect()
    }
}
