// Copyright 2024 Adam Burucs. MIT license.

const MAX_FILES: u8 = 32;
const FILENAME_LENGTH: u8 = 32;
const CONTENT_LENGTH: u16 = 1024;

struct Superblock {
    total_inodes: u16,
    used_inodes: u16,
    free_space: u16,
}

struct INode {
    filename: [u8; FILENAME_LENGTH as usize],
    content: [u8; CONTENT_LENGTH as usize],
    is_used: bool,
}

struct FileSystem {
    superblock: Superblock,
    inodes: [INode; MAX_FILES as usize],
}

fn init_filesystem(fs: &mut FileSystem) -> Result<(), ()> {
    fs.superblock.total_inodes = MAX_FILES as u16;
    fs.superblock.used_inodes = 0;
    fs.superblock.free_space = MAX_FILES as u16;
    for i in 0..=MAX_FILES {
        fs.inodes[i as usize].is_used = false;
        fs.inodes[i as usize].content = [0; CONTENT_LENGTH as usize];
        fs.inodes[i as usize].filename = [0; FILENAME_LENGTH as usize];
    }
    Ok(())
}

fn add_file<'a>(
    fs: &'a mut FileSystem,
    filename: &'a [u8; FILENAME_LENGTH as usize],
    content: &'a [u8; CONTENT_LENGTH as usize],
) -> Result<(), &'a str> {
    Ok(())
}

fn main() {
    println!("Easy File System (EFS)");
}
