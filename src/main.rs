// Copyright 2024 Adam Burucs. MIT license.

const MAX_FILES: u8 = 32;
const FILENAME_LENGTH: u8 = 32;
const CONTENT_LENGTH: u16 = 1024;

#[allow(dead_code)]
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

    fn total_inodes(&self) -> u16 {
        self.total_inodes
    }

    fn set_total_inodes(&mut self, new_total_inodes: u16) {
        self.total_inodes = new_total_inodes;
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
                return Ok(self.inodes[i as usize].content);
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

fn main() {
    println!("Easy File System (EFS)");
    println!();

    println!("Creating new file system...");
    let mut fs: FileSystem = FileSystem::new();
    println!("File system created");
    println!();

    let my_filename_str: &str = "myfile.txt";
    let my_content_str: &str = "Hey! This is my file. And it is awesome.";

    let mut filename_tmp: [u8; FILENAME_LENGTH as usize] = [0; FILENAME_LENGTH as usize];
    filename_tmp[..my_filename_str.len()].copy_from_slice(my_filename_str.as_bytes());

    let mut content_tmp: [u8; CONTENT_LENGTH as usize] = [0; CONTENT_LENGTH as usize];
    content_tmp[..my_content_str.len()].copy_from_slice(my_content_str.as_bytes());

    let file_add = fs.add_file(&filename_tmp, &content_tmp);

    match file_add {
        Ok(..) => println!("{my_filename_str} file successfully added."),
        Err(e) => println!("Error during adding: {e}"),
    }
}
