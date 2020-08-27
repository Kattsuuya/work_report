extern crate work_report;

use std::env;
use work_report::WorkReportGenerator;

// TODO: テストを書く
// ファイルの読み書きが発生するものをここに書く
// setup()で一時的なディレクトリやファイルを作成して，それを使ってテストを書くといいかも
// let temp_directory = env::temp_dir();

#[test]
fn it_works() {
    let generator = WorkReportGenerator::new(".");
}