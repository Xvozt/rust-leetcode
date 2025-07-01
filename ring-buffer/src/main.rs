#[derive(Debug)]
struct RingBuffer<T> {
    storage: Vec<T>,
    in_use_size: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Full;

impl<T: Copy> RingBuffer<T> {
    fn new(capacity: usize) -> Self
    where
        T: Default + Copy,
    {
        RingBuffer {
            storage: vec![Default::default(); capacity],
            in_use_size: 0,
            start: 0,
            end: 0,
        }
    }

    fn push(&mut self, item: T) -> Result<(), Full> {
        if self.is_full() {
            return Err(Full);
        }
        self.storage[self.end] = item;
        self.end = (self.end + 1) % self.storage.len();
        self.in_use_size += 1;
        Ok(())
    }

    fn pull(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.storage[self.start];
        self.start = (self.start + 1) % self.storage.len();
        self.in_use_size -= 1;
        Some(item)
    }

    fn is_full(&self) -> bool {
        self.in_use_size == self.storage.len()
    }

    fn is_empty(&self) -> bool {
        self.in_use_size == 0
    }
}

fn main() {
    println!("Let's learn about ring buffer");

    let mut b = RingBuffer::<i32>::new(3);
    let _ = b.push(5).unwrap();
    let _ = b.push(6).unwrap();
    let pl1 = b.pull().unwrap();
    //here expect 5
    println!("{pl1:?}");
    let _ = b.push(1).unwrap();
    //here expect [3, 6, 1]
    let _ = b.push(3).unwrap();
    println!("{b:?}");
    //here expect Error Full
    let _ = b.push(9).unwrap();
    println!("{b:?}");
}
