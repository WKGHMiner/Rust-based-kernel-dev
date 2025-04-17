use crate::{
    print, info,
    batch::run_next_app
};

const STDOUT: usize = 1;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn sys_write(fd: usize, buffer: *const u8, length: usize) -> isize {
    match fd {
        STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buffer, length) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);

            length as isize
        },
        _ => unimplemented!("Unsupported file direction: {}.", fd)
    }
}

pub fn sys_exit(code: i32) -> ! {
    info!("[kernel] Application exited with code {}", code);
    
    run_next_app();
}