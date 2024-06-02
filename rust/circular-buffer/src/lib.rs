pub struct CircularBuffer<T> {
    head: usize,
    tail: usize,
    size: usize,
    buffer: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            buffer.push(None);
        }

        Self {
            head: 0,
            tail: 0,
            size: 0,
            buffer,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.tail] = Some(element);
        self.tail = (self.tail + 1) % self.buffer.capacity();
        self.size += 1;

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            return Err(Error::EmptyBuffer);
        }

        let Some(last_item) = self.buffer[self.head].take() else {
            panic!("Out of sync buffer handling");
        };
        self.head = (self.head + 1) % self.buffer.capacity();
        self.size -= 1;

        Ok(last_item)
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.head = 0;
        self.tail = 0;
        self.buffer.fill_with(|| None);
    }

    pub fn overwrite(&mut self, element: T) {
        if !self.is_full() {
            self.write(element).expect("Out of sync buffer handling");
            return;
        }
        self.buffer[self.tail] = Some(element);
        self.head = (self.head + 1) % self.buffer.capacity();
        self.tail = (self.tail + 1) % self.buffer.capacity();
    }

    fn is_full(&self) -> bool {
        self.size >= self.buffer.capacity()
    }
}
