use core::{
    arch::asm,
    slice::{from_raw_parts, from_raw_parts_mut},
    mem::size_of,
};
use crate::{
    shutdown,
    info, info_print, warn,
    sync::SpinLock,
    trap::TrapContext
};
use lazy_static::lazy_static;

lazy_static! {
    static ref APP_MANAGER: SpinLock<AppManager> = {
        // _num_app is a beacon that points to the include section of our applications.
        unsafe extern "C" {
            safe fn _num_app();
        }

        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = unsafe { num_app_ptr.read_volatile() };

        let mut app_start = [0usize; MAX_APP_NUM + 1];
        // Here, plus the ptr by 1, then it will return the location of `app_0_start`.
        let app_start_raw = unsafe { from_raw_parts(num_app_ptr.add(1), num_app + 1) };
        app_start[..=num_app].copy_from_slice(app_start_raw);

        let manager = AppManager { num_app, current_app: 0, app_start };
        SpinLock::new(manager)
    };
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDR: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub fn get_stack_pointer(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, ctx: TrapContext) -> &'static mut TrapContext {
        let ptr = (self.get_stack_pointer() - size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ptr = ctx;

            ptr.as_mut().unwrap()
        }
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    pub fn get_stack_pointer(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

#[derive(Debug)]
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        info!("[kernel] num_app = {}", self.num_app);
        
        for i in 0..self.num_app {
            info_print!("[kernel] app_{} location: ", i);
            warn!("[{:#x}, {:#x})", self.app_start[i], self.app_start[i + 1]);
        }
    }

    #[allow(unsafe_op_in_unsafe_fn, reason = "Most of the ops are unsafe.")]
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            shutdown!(false);
        } else {
            info!("[kernel] Loading app_{}...", app_id);
        }

        for addr in APP_BASE_ADDR..APP_SIZE_LIMIT + APP_BASE_ADDR {
            (addr as *mut u8).write_volatile(0)
        }

        let src = from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id]
        );
        let dst = from_raw_parts_mut(
            APP_BASE_ADDR as *mut u8,
            src.len()
        );
        dst.copy_from_slice(src);

        asm!("fence.i");
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.lock().print_app_info();
}

pub fn run_next_app() -> ! {
    APP_MANAGER.scoped(|mut manager| {
        let current = manager.get_current_app();
        unsafe { manager.load_app(current) }
        manager.move_to_next_app();
    });

    let ctx = TrapContext::new(APP_BASE_ADDR, USER_STACK.get_stack_pointer());
    unsafe extern "C" { fn __restore(cx_addr: usize); }
    unsafe {
        __restore(KERNEL_STACK.push_context(ctx) as *const _ as usize);
    }

    unreachable!("Unreachable code when running applications.");
}