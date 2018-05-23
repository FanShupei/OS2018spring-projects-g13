mod exec;
mod sleep;
mod exit;
mod fork;

use traps::TrapFrame;

use self::exec::do_exec;
use self::sleep::sleep;
use self::exit::do_exit;
use self::fork::do_fork;
use console::kprintln;

/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.


pub fn handle_syscall(num: u16, tf: &mut TrapFrame) {
    match num {
        1 => {
            sleep(tf.x0 as u32, tf);
        },
        2 => {
            do_exec(tf.x0 as u32, tf);
        },
        3 => {
            kprintln!("user called! {}", tf.x0);
        },
        4 => {
            do_fork();
        }
        5 => {
            do_exit(tf);
        }
        _ => {
            // x7 = 1, syscall does not exist.
            tf.x1to29[6] = 1;
        }
    }
}
