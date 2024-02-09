// todo: preallocate api on start up from HAlloc
// statically start with 10 slots of 4096 bytes
const INITIAL_BLOCKS: usize = 10;
// typical page size in bytes on linux x86-64
const DEFAULT_BLOCK_SIZE: usize = 4096;

pub mod tsunami;

fn main() {
    let mem_limit = INITIAL_BLOCKS * DEFAULT_BLOCK_SIZE;
    let memory = vec![0u8; mem_limit];

    tsunami::run(&mut memory)
}
