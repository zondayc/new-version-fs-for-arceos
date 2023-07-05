#![no_std]
use xv6fs::{file::{VFile,FileType},disk_inode::InodeType,xv6fs::Xv6FS, BlockDevice};
extern crate alloc;
use alloc::{boxed::Box,vec::Vec,string::String, sync::Arc};
use axlog::info;
use axfs_vfs::{VfsOps};
use crate::dir::DirNode;

pub mod dir;
pub mod file;