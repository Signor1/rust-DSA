/*
 * Wrapper that uses array under the hood
 * Push/pop/ access the 0(1)
 * enqueue/dequeue/ has 0(N)
 * Constructor specifies the initial size
 */

type Array<T> = Vec<T>;

struct ArrayList<T> {
    pub length: usize,
    inner: Array<T>,
    tail: usize,
}

impl<T: Default + Clone> ArrayList<T> {
    fn new() -> Self {
        Self {
            length: 0,
            inner: vec![T::default(); 5],
            tail: 0,
        }
    }

    fn grow_inner(&mut self) {
        // let prev = self.inner;
        let new = vec![T::default(); self.inner.len() * 2];
    }

    pub fn prepend(item: T) {}

    // adding item to the end of the array
    pub fn append(&mut self, item: T) {
        self.length = self.length + 1;

        if self.inner.len() < self.length {
            // new inner of greater length
        } else {
            let i = self.tail + 1;
            self.inner[i] = item;
            self.tail = i;
        }
    }
}
