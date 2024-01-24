//! Main library.

#![no_std]
#![deny(missing_docs)]

mod block;
mod inode;

/// Stores important memory constants and types.
pub mod memory {
    /// Sets block size (4 MiB).
    pub const BLOCK_SIZE: usize = 4096;

    /// Defines the max length of a filename.
    pub const MAX_FILENAME_LENGTH: usize = 32;

    /// Defines the max number of direct pointers.
    pub const MAX_POINTERS: usize = 8;

    /// Defines the pointer type (32-bit).
    pub type Pointer = u32;

    /// Defines the pointer size (in bytes).
    pub const POINTER_SIZE: usize = 4;

    /// Defines the byte type.
    pub type Byte = u8;
}