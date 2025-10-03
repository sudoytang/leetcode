use std::collections::{BTreeSet, HashMap};

#[allow(unused)]

struct NumberContainers {
    container: HashMap<i32, i32>,
    index: HashMap<i32, BTreeSet<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl NumberContainers {

    fn new() -> Self {
        Self {
            container: HashMap::new(),
            index: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        use std::collections::hash_map::Entry;
        match self.container.entry(index) {
            Entry::Occupied(mut o) => {
                let old = *o.get();
                self.index.entry(old).and_modify(|v| {v.remove(&index);});
                *o.get_mut() = number;
                self.index.entry(number).or_default().insert(index);
            }
            Entry::Vacant(v) => {
                v.insert(number);
                self.index.entry(number).or_default().insert(index);
            }
        }
    }

    fn find(&self, number: i32) -> i32 {
        self.index
            .get(&number)
            .map_or(-1, |v| v.first().map_or(-1, |v| *v))
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
#[allow(unused)]
struct PlaceHolder;