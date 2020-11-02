use chrono::Local;
use glob::glob;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Generator to create daily work reports.
///
/// ## Examples
///
/// ```rust
/// extern crate work_report;
/// use work_report::WorkReportGenerator;
///
/// let generator = WorkReportGenerator::new("./work_report");
///
/// // Create a new work report.
/// // Argument `date` must be in the format of `YYYYmmdd`.
/// generator.create_new("20200101");
/// // Create today's work report.
/// generator.create_for_today();
///
/// // Archive a specific file.
/// generator.archive("./work_report/20200101.txt");
/// // Archive all the files.
/// generator.archive_all();
/// ```
pub struct WorkReportGenerator {
    exec_dir: String,
}

/// Core methods.
impl WorkReportGenerator {
    /// ## Arguments
    ///
    /// * `dir` - The path to the directory which manages work reports.
    pub fn new(dir: impl Into<String>) -> WorkReportGenerator {
        WorkReportGenerator {
            exec_dir: dir.into(),
        }
    }

    /// Archive all the work reports in the current directory by year and month in separate directoryies.
    ///
    /// ## Examples
    ///
    /// `./20200826.txt`
    /// is copied to
    /// `./Archive/2020/08/20200826.txt`
    pub fn archive_all(&self) {
        println!("Archiving...");
        // "YYYYmmdd.txt"のパターンにマッチするテキストファイルのみをアーカイブする
        let re = Regex::new(r"\d{8}.txt$").unwrap();
        for entry in glob(&format!("{}/*.txt", self.exec_dir))
            .expect("cannot get the contents of the directory.")
        {
            // 無効なパスは無視する
            let path_ = match entry {
                Ok(path) => path,
                Err(_) => continue,
            };
            let path = path_.to_str().unwrap();
            if re.is_match(path) {
                self.archive(&path);
            }
        }
        println!("All the files have been archived.");
    }

    /// Archive the work report.
    /// The file name must be in the format of `YYYYmmdd`.
    ///
    /// ## Arguments
    ///
    /// * `src_path` - The path to the file you want to archive.
    ///
    /// ## Examples
    ///
    /// `./20200826.txt`
    /// is copied to
    /// `./Archive/2020/08/20200826.txt`
    pub fn archive(&self, src_path: &str) {
        // ファイル名を切り取り，アーカイブ先までの途中のディレクトリを作成する
        let root = Path::new(src_path).file_name().unwrap().to_str().unwrap();
        let partial_path_for_archive = generate_partial_path_for_archive_dir(root);
        let dst_dir = format!("{}/{}", self.exec_dir, partial_path_for_archive);
        fs::create_dir_all(&dst_dir).expect("cannot create the directory");

        let dst_path: String = format!("{}/{}", dst_dir, root);
        let archived = Path::new(&dst_path).exists();
        // 未アーカイブか，ファイルの内容が更新されていれば，コピーする
        if !archived || updated(&src_path, &dst_path) {
            fs::copy(&src_path, &dst_path).expect("cannot copy the file.");
            println!("    Archived: {}", &root);
        }
    }

    /// Create today's work report based on `Template.txt` in the same directory as the execution file.
    pub fn create_for_today(&self) {
        let today = Local::today().format("%Y%m%d").to_string();
        self.create_new(today);
    }

    /// Create a new work report.
    /// Argument `date` must be in the format of `YYYYmmdd`.
    ///
    /// This method is equivalent to executing the following shell command.
    ///
    /// ```bash
    /// $ cp Template date.txt
    /// ```
    pub fn create_new(&self, date: impl Into<String>) {
        let dst_filename = format!("{}/{}.txt", self.exec_dir, date.into());
        let src_filename = format!("{}/Template.txt", self.exec_dir);
        if Path::new(&dst_filename).exists() {
            println!("Today's work report already exists.");
            return;
        }
        if !Path::new(&src_filename).exists() {
            println!("Template.txt was not found, so it is generated automatically.");
            self.create_template();
            println!("    Created: {}", &src_filename);
        }
        fs::copy(&src_filename, &dst_filename).expect("cannot copy the file.");
        println!("    Created: {}.", &dst_filename);
    }

    /// Create `Template.txt`
    fn create_template(&self) {
        let file_path = format!("{}/Template.txt", self.exec_dir);
        let mut file = File::create(&file_path).unwrap();
        // 自動生成されるTemplate.txtの中身
        let content = "\
<Today's task>\n\
-\n\
-\n\
\n\
<TODO>\n\
-\n\
-\n\
";
        writeln!(file, "{}", &content).unwrap();
        file.sync_all().expect("failed to write out `Template.txt`");
    }
}

/// Check whether the file has been updated by comparing with the previously archived file by hash value.
fn updated(path1: &str, path2: &str) -> bool {
    let content1 = fs::read_to_string(&path1).expect("cannot read the file.");
    let content2 = fs::read_to_string(&path2).expect("cannot read the file.");
    let hash1 = md5::compute(content1);
    let hash2 = md5::compute(content2);
    // ハッシュ値が異なる = ファイルが更新されている
    hash1 != hash2
}

/// Generates the partial path to the archive directory.
///
/// ## Examples
///
/// `20200826.txt` -> `Archive/2020/08`
fn generate_partial_path_for_archive_dir(file_name: &str) -> String {
    // ファイル名から年，月，日をそれぞれ取り出すための正規表現
    let re = Regex::new(
        r"(?x)
        (?P<Y>\d{4})
        (?P<m>\d{2})
        (?P<d>\d{2})
        .txt",
    )
    .unwrap();
    let caps = re.captures(file_name).unwrap();
    let year = caps
        .name("Y")
        .expect("did not match the regular expression.")
        .as_str();
    let month = caps
        .name("m")
        .expect("did not match the regular expression.")
        .as_str();
    let partial_path = format!("Archive/{}/{}", year, month);
    partial_path
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_partial_path_for_archive_dir() {
        fn before_after(input: &str, expected_output: String) {
            println!("{} -> {}", input, expected_output);
            assert_eq!(
                generate_partial_path_for_archive_dir(input),
                expected_output
            );
        }

        before_after("20200826.txt", "Archive/2020/08".to_string());
    }
}
