struct MinStack {
    stk: Vec<i32>,
    min_stk: Vec<(usize, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stk: Vec::new(),
            min_stk: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        match self.min_stk.last() {
            None => self.min_stk.push((self.stk.len(), val)),
            Some(&(_, minval)) if minval >= val => {
                self.min_stk.push((self.stk.len(), val));
            }
            _ => {}
        }
        self.stk.push(val);
    }

    fn pop(&mut self) {
        self.stk.pop();
        match self.min_stk.last() {
            Some(&(i, _)) if i == self.stk.len() => {
                self.min_stk.pop();
            }
            _ => {}
        }
    }

    fn top(&self) -> i32 {
        *self.stk.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min_stk.last().map(|&(_, v)| v).unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
struct PH;