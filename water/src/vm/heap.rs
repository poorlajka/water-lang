pub struct Heap {
    memory: Vec<u8>,
    bump: usize,

}

#[repr(u8)]
pub enum ObjectKind {
    String = 0,
    Array = 1,
}

// Header is 16 bytes - two 8 byte words
// Word 1: [8 bits kind][56 bits size]
// Word 2: GC metadata (mark bit for now)
const HEADER_SIZE: usize = 16;
const KIND_SHIFT: u64 = 56;
const SIZE_MASK: u64 = (1 << 56) - 1;
const GC_MARK_BIT: u64 = 1;

impl Heap {
    pub fn new(capacity: usize) -> Self {
        Self {
            memory: vec![0u8; capacity],
            bump: 0,
        }
    }

    pub fn alloc(&mut self, kind: ObjectKind, data_size: usize) -> Option<usize> {
        let total_size = HEADER_SIZE + data_size;
        
        // Align to 8 bytes
        let aligned_size = (total_size + 7) & !7;

        if self.bump + aligned_size > self.memory.len() {
            return None; // out of memory, GC will handle this later
        }

        let ptr = self.bump;
        self.bump += aligned_size;

        // Write word 1 - kind and size
        let word1 = ((kind as u64) << KIND_SHIFT) | (data_size as u64 & SIZE_MASK);
        self.write_u64(ptr, word1);

        // Write word 2 - GC metadata, zero initialized
        self.write_u64(ptr + 8, 0);

        Some(ptr)
    }

    // Read/write helpers
    pub fn write_u64(&mut self, offset: usize, val: u64) {
        self.memory[offset..offset + 8].copy_from_slice(&val.to_le_bytes());
    }

    pub fn read_u64(&self, offset: usize) -> u64 {
        u64::from_le_bytes(self.memory[offset..offset + 8].try_into().unwrap())
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        self.memory[offset..offset + bytes.len()].copy_from_slice(bytes);
    }

    pub fn read_bytes(&self, offset: usize, len: usize) -> &[u8] {
        &self.memory[offset..offset + len]
    }

    // Header accessors
    pub fn get_kind(&self, ptr: usize) -> u8 {
        (self.read_u64(ptr) >> KIND_SHIFT) as u8
    }

    pub fn get_size(&self, ptr: usize) -> usize {
        (self.read_u64(ptr) & SIZE_MASK) as usize
    }

    pub fn get_mark(&self, ptr: usize) -> bool {
        self.read_u64(ptr + 8) & GC_MARK_BIT != 0
    }

    pub fn set_mark(&mut self, ptr: usize, marked: bool) {
        let word2 = self.read_u64(ptr + 8);
        let new_word2 = if marked {
            word2 | GC_MARK_BIT
        } else {
            word2 & !GC_MARK_BIT
        };
        self.write_u64(ptr + 8, new_word2);
    }

    // Data section starts after the header
    pub fn data_ptr(&self, ptr: usize) -> usize {
        ptr + HEADER_SIZE
    }
}