#[derive(Debug, Clone)]
pub struct Bits {
    size: usize,
    inner: Vec<u8>,
}

impl Bits {
    pub fn new(slots: usize) -> Bits {
        Bits {
            size: slots,
            inner: vec![0; slots],
        }
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.size {
            return None;
        }

        let byte_index = index / 8;
        let bit_index = index % 8;

        Some((*self.inner.get(byte_index)? >> bit_index) & 0b1)
    }

    pub fn set(&mut self, index: usize, val: u8) -> Result<(), ()> {
        if index >= self.size {
            return Err(());
        }

        let byte_index = index / 8;
        let bit_index = index % 8;

        let byte = self.inner.get_mut(byte_index).ok_or(())?;

        let mask = !(0b1 << bit_index);

        *byte = (*byte & mask) | ((val & 0b1) << bit_index);

        Ok(())
    }

    pub fn is_all_set(&self) -> bool {
        let sum: usize = self.inner.iter().map(|i| *i as usize).sum();

        let full_bytes = self.size / 8;
        let remaining = self.size % 8;

        sum == full_bytes * 255 + (1 << remaining) - 1
    }
}
