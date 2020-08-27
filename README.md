# work_report

Create and archive daily work reports.

## Install

[Rust installation page](https://www.rust-lang.org/tools/install)

```bash
$ git clone https://github.com/KindMaple/work_report.git
$ cargo build --release
$ cp target/release/work_report /path/to/any/directory/
```

## Examples

```bash
$ ls
work_report

$ ./work_report 
Template.txt was not found, so it is generated automatically.
    Created: /tmp/20200828/Template.txt
Created: /tmp/20200828/20200828.txt.

$ ls
20200828.txt  Template.txt  work_report
// you can edit `Template.txt` as you see fit.

$ ./work_report 
Archiving...
    Archived: 20200828.txt
All the files have been archived.
Today's work report already exists.

$ tree
.
├── 20200828.txt
├── Archive
│   └── 2020
│       └── 08
│           └── 20200828.txt
├── Template.txt
└── work_report

3 directories, 4 files
```
