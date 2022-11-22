pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    start_cursor: usize,
    end_cursor: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: (0..capacity).map(|_| None).collect(),
            start_cursor: 0,
            end_cursor: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.start_cursor == self.end_cursor {
            if self.buffer[self.end_cursor].is_none() {
                self.buffer[self.end_cursor] = Some(element);
                self.end_cursor = self.increase_index(self.end_cursor);
            } else {
                return Err(Error::FullBuffer);
            }
        } else {
            self.buffer[self.end_cursor] = Some(element);
            self.end_cursor = self.increase_index(self.end_cursor);
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.buffer[self.start_cursor]
            .take()
            .ok_or(Error::EmptyBuffer)
            .map(|value| {
                self.start_cursor = self.increase_index(self.start_cursor);
                value
            })
    }

    pub fn clear(&mut self) {
        if self.start_cursor == self.end_cursor {
            if self.buffer[self.end_cursor].is_some() {
                self.buffer[self.start_cursor] = None;
                self.start_cursor = self.increase_index(self.start_cursor);
                self.end_cursor = self.increase_index(self.end_cursor);
            }
        } else {
            self.buffer[self.start_cursor] = None;
            self.start_cursor = self.increase_index(self.start_cursor);
        }
    }

    pub fn overwrite(&mut self, element: T) {
        self.buffer[self.end_cursor] = Some(element);

        if self.start_cursor == self.end_cursor {
            self.start_cursor = self.increase_index(self.start_cursor)
        }
        self.end_cursor = self.increase_index(self.end_cursor);
    }

    fn increase_index(&self, index: usize) -> usize {
        (index + 1) % self.buffer.len()
    }
}
