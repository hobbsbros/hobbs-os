//! Library for inode operations.

use crate::memory::*;

/// Compute the size of an inode.
const INODE_SIZE: usize = MAX_FILENAME_LENGTH + POINTER_SIZE * MAX_POINTERS;

/// Assert inode size.
const _CHECK_INODE_SIZE: () = assert!(BLOCK_SIZE % INODE_SIZE == 0);

/// Abstracts over inodes.
pub struct Inode {
    name: [char; MAX_FILENAME_LENGTH],
    pointers: [Option<Pointer>; MAX_POINTERS],
}

/// Implements behaviors in inodes.
impl Inode {
    /// Constructs a new inode.
    pub fn new(name: [char; MAX_FILENAME_LENGTH], pointers: [Option<Pointer>; MAX_POINTERS]) -> Self {
        Self {
            name,
            pointers,
        }
    }

    /// Converts an inode into a bytearray.
    pub fn to_bytes(&self) -> [Byte; INODE_SIZE] {
        let mut output = [0; INODE_SIZE];

        todo!();

        output
    }
}