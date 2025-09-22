pub struct CircularBuffer<T> {
    write_pointer: usize,
    read_pointer: usize,
    capacity: usize,
    buffer: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + std::fmt::Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            read_pointer: 0,
            write_pointer: 0,
            capacity,
            buffer: vec![None; capacity],
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer.iter().all(|x| x.is_some()) {
            return Err(Error::FullBuffer);
        }

        self.buffer[self.write_pointer] = Some(element);
        self.increment_write_pointer();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.len() == 0 || self.buffer[self.read_pointer].is_none() {
            return Err(Error::EmptyBuffer);
        }

        let element = self.buffer[self.read_pointer].take().unwrap();
        self.increment_read_pointer();
        Ok(element)
    }

    pub fn clear(&mut self) {
        self.buffer = vec![None; self.capacity];
        self.read_pointer = 0;
        self.write_pointer = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        self.write(element.clone()).unwrap_or_else(|_| {
            self.buffer[self.write_pointer] = Some(element);
            self.increment_write_pointer();
            self.increment_read_pointer();
        });
    }

    fn increment_read_pointer(&mut self) {
        self.read_pointer = (self.read_pointer + 1) % self.capacity;
    }

    fn increment_write_pointer(&mut self) {
        self.write_pointer = (self.write_pointer + 1) % self.capacity;
    }
}
