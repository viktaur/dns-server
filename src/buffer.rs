use anyhow::{Result, anyhow};

pub struct ByteDecoder {
    data: Vec<u8>,
    pos: usize
}

impl ByteDecoder {
    pub fn new(slice: &[u8]) -> Self {
        let data = Vec::from(slice);

        ByteDecoder {
            data,
            pos: 0,
        }
    }

    /// Returns a slice withg the next `n` bytes from the current position, updating the
    /// current poisition.
    pub fn read_n_bytes(&mut self, n: usize) -> Result<&[u8]> {
        if self.pos + n > self.data.len() {
            return Err(anyhow!("End of buffer"));
        }

        let slice = &self.data[self.pos..self.pos+n];
        self.pos += n;
        Ok(slice)
    }

    /// Moves the pointer to a specific position in the buffer.
    pub fn _jump_to(&mut self, i: usize) -> Result<()> {
        if i >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos = i;

        // Ok(self.data[self.pos])
        Ok(())
    }

    /// Moves the pointer by one position and returns the new current element.
    pub fn _next(&mut self) -> Result<u8> {
        if self.pos + 1 >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += 1;

        Ok(self.data[self.pos])
    }

    /// Returns the remaining bytes from the current position until the end of the buffer.
    pub fn slice(&self) -> &[u8] {
        &self.data[self.pos..]
    }

    /// Moves the position pointer by `n` bytes.
    pub fn step(&mut self, n: usize) -> Result<()> {
        if self.pos + n >= self.data.len() {
            return Err(anyhow!("End of buffer"));
        }
        self.pos += n;

        Ok(())
    }
}
