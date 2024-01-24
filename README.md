# Hobbs OS

An extremely simple OS written in pure `no_std` Rust.

## Goals

The developers of the Hobbs OS seek to fulfill the following goals:
- [x] Has no Rust library dependencies
- [x] Has no system library dependencies
- [x] Uses no unsafe code
- [x] Uses flexible block, inode, and pointer sizes
- [ ] Accepts instructions from a Unix-like shell

## Features

- [ ] Constructs blocks
- [ ] Constructs inodes
- [ ] Asserts an integer number of inodes fit inside each block
- [ ] Constructs superblocks
- [ ] Constructs disk images
- [ ] Links inodes to data blocks
- [ ] Reads files from inodes
- [ ] Constructs single indirect pointers
- [ ] Accepts instructions from shell