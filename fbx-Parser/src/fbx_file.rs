use core::panic;
use std::{fs::File,
    str};

use crate::{fbx_node::*, fbx_reader::*};

pub fn read_file(file_path: &str) {

    let file = File::open(&file_path)
        .expect("Should've been able to open a file");

    let mut reader = FbxReader::new(file);
    println!("Offset initial: {}", reader.offset);

    check_fbx_magic((&mut reader).read_magic());
    check_fbx_version((&mut reader).read_version());

    println!("Offset after version: {}", reader.offset);
    let mut node = FbxNode::new(&mut reader);
    
    node.read_node();

}


fn check_fbx_magic(buffer: &[u8; 23]) {
    let magic: &[u8] = "Kaydara FBX Binary  ".as_bytes();
    let magic_actual: &[u8] = &buffer[0..23];

    for i in 0..magic.len() {
        if magic[i] != magic_actual[i] {
            panic!("File should have valid magic");
        }
    }

    if magic_actual[20] != 0x00 {
        panic!("There should be 0x00 instead of {}", magic_actual[20])
    }
    if magic_actual[21] != 0x1A {
        panic!("There should be 0x1A instead of {}", magic_actual[21])
    }
    if magic_actual[22] != 0x00 {
        panic!("There should be 0x00 instead of {}", magic_actual[22])
    }

    println!("Magic check passed!")
}


fn check_fbx_version(buffer: &[u8; 4]) {

    let max_version = 7400;
    let version_actual = i32::from_le_bytes(*buffer);
    
    if version_actual > max_version {
        panic!("File version should not exceed {}. Actual file version: {}", max_version, version_actual);
    }

    println!("Version check passed!")
}
