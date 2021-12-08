mod gen_maze;
mod path_finders;
mod util;

mod window;
use window::run;
pub fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
    }
}
