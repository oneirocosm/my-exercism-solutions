use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    wrapped: R,
    bytes_read: usize,
    times_read: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            bytes_read: 0,
            times_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.times_read
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.times_read += 1;
        let bytes_read_this_time = self.wrapped.read(buf)?;
        self.bytes_read += bytes_read_this_time;
        Ok(bytes_read_this_time)
    }
}

pub struct WriteStats<W: Write> {
    wrapped: W,
    bytes_written: usize,
    times_written: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            bytes_written: 0,
            times_written: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.times_written
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.times_written += 1;
        let bytes_written_this_time = self.wrapped.write(buf)?;
        self.bytes_written += bytes_written_this_time;
        Ok(bytes_written_this_time)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
