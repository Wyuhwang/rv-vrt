mod stdio;

use crate::riscv_mm::UserBuffer;

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}

pub use stdio::{Stdin, Stdout};
