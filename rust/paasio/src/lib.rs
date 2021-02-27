use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    read_count: usize,
    read_byte_count: usize,
    internal_reader: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            read_count: 0,
            read_byte_count: 0,
            internal_reader: wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.internal_reader
    }

    pub fn bytes_through(&self) -> usize {
        self.read_byte_count
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read_count += 1;

        let result = self.internal_reader.read(buf);
        if let Ok(count) = result {
            self.read_byte_count += count
        }

        result
    }
}

pub struct WriteStats<W> {
    write_count: usize,
    write_byte_count: usize,
    internal_writer: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            write_count: 0,
            write_byte_count: 0,
            internal_writer: wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.internal_writer
    }

    pub fn bytes_through(&self) -> usize {
        self.write_byte_count
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write_count += 1;
        let result = self.internal_writer.write(buf);
        if let Ok(count) = result {
            self.write_byte_count += count
        }

        result
    }

    fn flush(&mut self) -> Result<()> {
        self.internal_writer.flush()
    }
}
