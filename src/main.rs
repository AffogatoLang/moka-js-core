#[macro_use]
extern crate js;
extern crate libc;
extern crate docopt;
extern crate moka_runner;

use moka_runner::util;
use moka_runner::runtime::Container;

use std::str;
use std::path::Path;

use moka_runner::runtime::library::std::console;

use docopt::Docopt;

const USAGE: &'static str = "Usage: runner <path>";

fn main() {
    let args = Docopt::new(USAGE)
                .and_then(|d| d.parse())
                .unwrap_or_else(|e| e.exit());

    let path = args.get_str("<path>");

    let file = util::files::read_as_file(Path::new(path)).unwrap();

    let name = file.0;
    let contents = file.1.as_str();

    let container = Container::new();
    rooted!(in(container.context) let rooted = container.global);
    let root_handle = rooted.handle();

    container.declare_global_fn(root_handle, "log\0", Some(console::log), 1);
    container.exec_c(root_handle, contents, name.as_str())
}
