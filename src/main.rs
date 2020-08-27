extern crate work_report;

use std::env;
use work_report::WorkReportGenerator;

fn main() {
    let mut exec_dir = env::current_exe().expect("cannot get the current directory.");
    // /path/to/exec/a.out -> /path/to/exec
    exec_dir.pop();
    let exec_dir = exec_dir.to_str().unwrap();

    let generator = WorkReportGenerator::new(exec_dir);
    generator.archive_all();
    generator.create_for_today();
}
