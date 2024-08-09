// Copyright 2024 Adam Burucs. MIT license.

use colored::*;

const MAX_FILES: u8 = 32;
const FILENAME_LENGTH: u8 = 32;
const CONTENT_LENGTH: u16 = 1024;

#[allow(dead_code)]
#[derive(Debug)]
struct Superblock {
    total_inodes: u16,
    used_inodes: u16,
    free_space: u16,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct INode {
    filename: [u8; FILENAME_LENGTH as usize],
    content: [u8; CONTENT_LENGTH as usize],
    is_used: bool,
}

#[allow(dead_code)]
struct FileSystem {
    superblock: Superblock,
    inodes: [INode; MAX_FILES as usize],
}

impl Superblock {
    fn new() -> Self {
        Self {
            total_inodes: MAX_FILES as u16,
            used_inodes: 0,
            free_space: MAX_FILES as u16,
        }
    }

    fn used_inodes(&self) -> u16 {
        self.used_inodes
    }

    fn set_used_inodes(&mut self, new_used_inodes: u16) {
        self.used_inodes = new_used_inodes;
    }

    fn free_space(&self) -> u16 {
        self.free_space
    }

    fn set_free_space(&mut self, new_free_space: u16) {
        self.free_space = new_free_space;
    }
}

impl INode {
    fn new() -> Self {
        Self {
            filename: [0; FILENAME_LENGTH as usize],
            content: [0; CONTENT_LENGTH as usize],
            is_used: false,
        }
    }

    fn filename(&self) -> [u8; FILENAME_LENGTH as usize] {
        self.filename
    }

    fn set_filename(&mut self, new_filename: [u8; FILENAME_LENGTH as usize]) {
        self.filename = new_filename;
    }

    fn content(&self) -> [u8; CONTENT_LENGTH as usize] {
        self.content
    }

    fn set_content(&mut self, new_content: [u8; CONTENT_LENGTH as usize]) {
        self.content = new_content;
    }

    fn is_used(&self) -> bool {
        self.is_used
    }

    fn set_is_used(&mut self, new_is_used: bool) {
        self.is_used = new_is_used;
    }
}

impl FileSystem {
    fn new() -> Self {
        let superblock = Superblock::new();
        let inode = INode::new();
        let inodes: [INode; MAX_FILES as usize] = [inode; MAX_FILES as usize];

        Self { superblock, inodes }
    }

    fn get_info(&self) -> &Superblock {
        &self.superblock
    }

    fn add_file<'a>(
        &mut self,
        filename: &'a [u8; FILENAME_LENGTH as usize],
        content: &'a [u8; CONTENT_LENGTH as usize],
    ) -> Result<(), &'a str> {
        if self.superblock.used_inodes() >= MAX_FILES as u16 {
            return Err("No free inodes available.");
        }
        for i in 0..MAX_FILES {
            if !self.inodes[i as usize].is_used() {
                self.inodes[i as usize].set_filename(*filename);
                self.inodes[i as usize].set_content(*content);
                self.inodes[i as usize].set_is_used(true);
                self.superblock
                    .set_used_inodes(self.superblock.used_inodes() + 1);
                self.superblock
                    .set_free_space(self.superblock.free_space() - 1);
                break;
            }
        }
        Ok(())
    }

    fn read_file<'a>(
        &self,
        filename: &'a [u8; FILENAME_LENGTH as usize],
    ) -> Result<[u8; CONTENT_LENGTH as usize], &'a str> {
        for i in 0..MAX_FILES {
            if self.inodes[i as usize].is_used() && self.inodes[i as usize].filename() == *filename
            {
                return Ok(self.inodes[i as usize].content());
            }
        }
        Err("File not found.")
    }

    fn delete_file<'a>(
        &mut self,
        filename: &'a [u8; FILENAME_LENGTH as usize],
    ) -> Result<(), &'a str> {
        for i in 0..MAX_FILES {
            if self.inodes[i as usize].is_used() && self.inodes[i as usize].filename() == *filename
            {
                self.inodes[i as usize].set_content([0; CONTENT_LENGTH as usize]);
                self.inodes[i as usize].set_is_used(false);
                self.superblock
                    .set_used_inodes(self.superblock.used_inodes() - 1);
                self.superblock
                    .set_free_space(self.superblock.free_space() + 1);
                return Ok(());
            }
        }
        Err("File not found.")
    }
}

fn make_filename_bytes(filename: &str) -> [u8; FILENAME_LENGTH as usize] {
    let mut filename_tmp: [u8; FILENAME_LENGTH as usize] = [0; FILENAME_LENGTH as usize];
    filename_tmp[..filename.len()].copy_from_slice(filename.as_bytes());
    filename_tmp
}

fn make_content_bytes(content: &str) -> [u8; CONTENT_LENGTH as usize] {
    let mut content_tmp: [u8; CONTENT_LENGTH as usize] = [0; CONTENT_LENGTH as usize];
    content_tmp[..content.len()].copy_from_slice(content.as_bytes());
    content_tmp
}

fn main() {
    println!("{}", "Easy File System (EFS)".bright_white().on_blue());
    println!();
    println!("Creating new file system...");
    let mut fs: FileSystem = FileSystem::new();
    println!("File system created");
    println!();

    println!("{}", "File system information".bright_yellow());
    println!("{:?}", fs.get_info());
    println!();

    let filename_str = "myfile.txt";
    let filename_bytes = make_filename_bytes(filename_str);
    let content_bytes = make_content_bytes("Hey! This is my file. And it is awesome.");

    let file_add = fs.add_file(&filename_bytes, &content_bytes);

    match file_add {
        Ok(..) => println!("{filename_str} file successfully added."),
        Err(e) => println!("Error during adding: {e}"),
    }
    println!();

    println!("Trying to read {filename_str}...");
    let file_read = fs.read_file(&filename_bytes);
    match file_read {
        Ok(read_content) => {
            println!("File contents are the following:");
            println!("{:?}", read_content);
        }
        Err(e) => println!("Error during reading: {e}"),
    }

    println!();
    println!("Trying to delete {filename_str}...");
    let file_delete = fs.delete_file(&filename_bytes);
    match file_delete {
        Ok(..) => println!("{filename_str} file successfully deleted."),
        Err(e) => println!("Error during deleting: {e}"),
    }
    println!();
}
