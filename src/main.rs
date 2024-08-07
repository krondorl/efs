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
    for i in 0..MAX_FILES {
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
    if fs.superblock.used_inodes >= MAX_FILES as u16 {
        return Err("No free inodes available.");
    }
    for i in 0..MAX_FILES {
        if !fs.inodes[i as usize].is_used {
            fs.inodes[i as usize].filename = *filename;
            fs.inodes[i as usize].content = *content;
            fs.inodes[i as usize].is_used = true;
            fs.superblock.used_inodes += 1;
            fs.superblock.free_space -= 1;
            break;
        }
    }
    Ok(())
}

fn read_file<'a>(
    fs: &'a mut FileSystem,
    filename: &'a [u8; FILENAME_LENGTH as usize],
) -> Result<[u8; CONTENT_LENGTH as usize], &'a str> {
    for i in 0..MAX_FILES {
        if fs.inodes[i as usize].is_used && fs.inodes[i as usize].filename == *filename {
            return Ok(fs.inodes[i as usize].content);
        }
    }
    Err("File not found.")
}

fn delete_file<'a>(
    fs: &'a mut FileSystem,
    filename: &'a [u8; FILENAME_LENGTH as usize],
) -> Result<(), &'a str> {
    for i in 0..MAX_FILES {
        if fs.inodes[i as usize].is_used && fs.inodes[i as usize].filename == *filename {
            fs.inodes[i as usize].content = [0; CONTENT_LENGTH as usize];
            fs.inodes[i as usize].is_used = false;
            fs.superblock.used_inodes -= 1;
            fs.superblock.free_space += 1;
            return Ok(());
        }
    }
    Err("File not found.")
}

fn main() {
    println!("Easy File System (EFS)");
}
