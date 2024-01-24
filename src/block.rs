//! Library for block operations.

use crate::memory::*;

/// Abstracts over data blocks.
pub struct Block {
    data: [Byte; BLOCK_SIZE],
}