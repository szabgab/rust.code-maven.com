use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

use chrono::{DateTime, Utc};
use clap::Parser;

// use regex::Regex;

// from all the md files extract the list of included files
// from the examples/ directory list all the files
// make sure each file is included and is only included once

// Run every rs file, if there is an out file compare the results.
//
const ROOT: &str = "../../..";

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    verbose: bool,

    #[arg(long)]
    use_examples: bool,

    #[arg(long)]
    update: bool,

    #[arg(long)]
    fmt: bool,

    #[arg(long)]
    fmt_check: bool,

    #[arg(long)]
    clippy: bool,

    examples: Vec<String>,
}

fn main() {
    let start: DateTime<Utc> = Utc::now();

    let args = Cli::parse();
    let log_level = if args.verbose {
        log::Level::Info
    } else {
        log::Level::Warn
    };
    simple_logger::init_with_level(log_level).unwrap();
    log::info!("verbose: {}", args.verbose);

    std::env::set_current_dir(ROOT).unwrap();

    let examples = if args.examples.is_empty() {
        get_crates(Path::new("examples"))
    } else {
        args.examples
            .iter()
            .map(|name| PathBuf::from(name))
            .collect()
    };
    log::info!("Number of examples: {}", examples.len());

    let unused_examples = check_use_of_example_files(args.use_examples);

    let (update_success, update_failures) = cargo_on_all(&examples, args.update, &["update"]);
    log::info!(
        "updated_success: {update_success}, update_failure: {}",
        update_failures.len()
    );

    let (fmt_success, fmt_failures) = cargo_on_all(&examples, args.fmt, &["fmt"]);
    log::info!(
        "fmt_success: {fmt_success}, fmt_failure: {}",
        fmt_failures.len()
    );

    let (fmt_check_success, fmt_check_failures) =
        cargo_on_all(&examples, args.fmt_check, &["fmt", "--check"]);
    log::info!(
        "fmt_check_success: {fmt_check_success}, fmt_check_failure: {}",
        fmt_check_failures.len()
    );

    let skip_clippy = &[
        "examples/intro/formatting-required",
        "examples/intro/print",
        "examples/functions/declare-twice",
        "examples/variables/change-literal-string",
        "examples/variables/immutable-string",
        "examples/variables/immutable-number",
        "examples/variables/cannot-change-type",
        "examples/tuples/empty",
        "examples/numbers/small-integers-unfit-in-i8",
        "examples/numbers/rounding-float",
        "examples/booleans/other",
        "examples/ownership/mutable-string-in-immutable-variable",
        "examples/files/list-tree",          // TODO
        "examples/files/open-file-handling", // TODO
        "examples/arrays/numbers-change",
        "examples/types/type-mismatch",
        "examples/errors/out-of-bounds-array",
        "examples/errors/div-by-zero-hard-coded",
        "examples/advanced-functions/calculator", // TODO
    ];


    let (clippy_success, clippy_failures) 
    = cargo_clippy(examples.clone(), args.clippy, &["clippy", "--", "--deny", "warnings"], skip_clippy);
    log::info!(
        "clippy_success: {clippy_success}, clippy_failure: {}",
        clippy_failures.len()
    );

    println!("------- Report -------");
    let end: DateTime<Utc> = Utc::now();
    println!("Elapsed: {}", end.timestamp() - start.timestamp());

    if unused_examples > 0 {
        eprintln!("There are {unused_examples} unused examples");
    }
    if fmt_failures.len() > 0 {
        eprintln!("There are {} examples with fmt errors.", fmt_failures.len());
        for failure in &fmt_failures {
            eprintln!("  {failure:?}",);
        }
    }
    if update_failures.len() > 0 {
        eprintln!(
            "There are {} examples with update errors.",
            update_failures.len()
        );
        for failure in &update_failures {
            eprintln!("  {failure:?}",);
        }
    }
    if clippy_failures.len() > 0 {
        eprintln!("There are {} examples with clippy errors.", clippy_failures.len());
    }
    if unused_examples > 0 || update_failures.len() > 0 || clippy_failures.len() > 0 {
        exit(1);
    }
}

fn check_use_of_example_files(use_examples: bool) -> i32 {
    if !use_examples {
        return 0;
    }
    log::info!("check_use_of_example_files");
    let md_files = get_md_files();
    let imported_files = get_imported_files(md_files);
    let examples = get_all_the_examples();

    let mut count = 0;
    for filename in examples {
        if filename.ends_with("swp") {
            continue;
        }
        if filename.ends_with("counter.db") {
            continue;
        }
        if !imported_files.contains(&filename) {
            if filename.starts_with("examples/rocket/people-and-groups/templates/") {
                continue;
            }
            let files = [
                "examples/threads/count-characters/aadef.txt",
                "examples/threads/count-characters/abc.txt",
                "examples/rocket/static-files/static/favicon.ico",
                "examples/files/count-digits/digits.txt",
                "examples/files/read-whole-file/data.txt",
                "examples/ownership/concatenate-content-of-files/dog.txt",
                "examples/ownership/concatenate-content-of-files/cat.txt",
                "examples/ownership/read-file-and-trim-newline/cat.txt",
            ]
            .into_iter()
            .map(|name| name.to_owned())
            .collect::<Vec<String>>();
            if files.contains(&filename) {
                continue;
            }

            log::error!("Unused file: `{filename}`");
            count += 1;
        }
    }
    count
}

fn cargo_on_all<'a>(crates: &'a Vec<PathBuf>, run: bool, args: &[&str]) -> (i32, Vec<&'a PathBuf>) {
      let mut count_success = 0;
    let mut failures = vec![];
    if !run {
        return (count_success, failures);
    }

    for (_ix, crate_folder) in crates.into_iter().enumerate() {
        let result = cargo_on_single(&crate_folder, args, &[]);
        if result {
            count_success += 1;
        } else {
            failures.push(crate_folder);
        }
    }

    (count_success, failures)
}

fn cargo_on_single(crate_path: &PathBuf, args: &[&str], skip: &[&str]) -> bool {
    log::info!("cargo {args:?} on {crate_path:?}",);
    let folder = crate_path.clone().into_os_string().into_string().unwrap();
    let folders = skip
    .into_iter()
    .map(|x| x.to_string())
    .collect::<String>();
    if folders.contains(&folder) {
        return true;
    }

    let error = format!("failed to execute 'cargo {args:?} --check' process");
    let mut cmd = Command::new("cargo");
    for arg in args {
        cmd.arg(arg);
    }

    let result = cmd.current_dir(crate_path).output().expect(&error);

    if !result.status.success() {
        log::error!("Cannot run {args:?} on crate: {crate_path:?}");
        return false;
    }
    true
}


fn cargo_clippy<'a>(crates: Vec<PathBuf>, run: bool, args: &'static [&str], skip: &'static [&str]) -> (i32, Vec<PathBuf>) {
    let mut count_success = 0;
    let mut failures = vec![];
    if !run {
        return (count_success, failures);
    }

    log::info!("cargo_clippy");
    let number_of_crates = crates.len();

    // We want run max_threads at once, when one is finished we start a new one
    // Then we collect the messages from the remaining ones.
    let (tx, rx) = mpsc::channel();
    let max_threads = 2;
    let mut thread_count = 0;
    let mut started = 0;
    let mut finished = 0;

    for (ix, crate_folder) in crates.iter().cloned().enumerate() {
        started += 1;
        log::info!("crate: {}/{number_of_crates}, {crate_folder:?}", ix + 1);
        let mytx = tx.clone();

        thread::spawn(move || {
            let res = cargo_on_single(&crate_folder, args, skip);
            mytx.send((res, crate_folder)).unwrap();
        });
        thread_count += 1;
        if thread_count >= max_threads {
            let received = rx.recv().unwrap();
            if received.0 {
                count_success += 1;
            } else {
                failures.push(received.1);
            }
            finished += 1;
        }
    }

    for received in rx {
        //println!("received {thread_count}");
        finished += 1;
        if received.0 {
            count_success += 1;
        } else {
            failures.push(received.1);
        }
        if finished >= started {
            break;
        }
    }

    log::info!("check crates done");

    (count_success, failures)
}


fn get_crates(path: &Path) -> Vec<PathBuf> {
    log::info!("get_crates");
    let crates = get_crates_recoursive(path);
    log::info!("get_crates done\n");
    crates
}

fn get_crates_recoursive(path: &Path) -> Vec<PathBuf> {
    let mut crates: Vec<PathBuf> = vec![];
    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        if entry.path().ends_with("target") {
            continue;
        }
        //println!("{:?}", entry);
        if entry.path().ends_with("Cargo.toml") {
            //println!("cargo: {:?}", entry.path().parent());
            crates.push(entry.path().parent().unwrap().to_path_buf());
        }
        if entry.path().is_dir() {
            crates.extend(get_crates_recoursive(entry.path().as_path()));
        }
    }

    crates
}

// TODO: go deeper than 2 levels to also handle examples/*/src/main.rs
// TODO: but exclude examples/*/target/
fn get_all_the_examples() -> Vec<String> {
    log::info!("get_all_the_examples");

    let exclude: Vec<String> = [
        "examples/image/create-image/image.png",
        "examples/other/multi_counter_with_manual_csv/counter.csv",
        "examples/other/send-mail-with-sendgrid/config.txt",
    ]
    .iter()
    .map(|path| path.to_string())
    .collect();
    let pathes = get_examples(Path::new("examples"));
    let pathes: Vec<String> = pathes
        .iter()
        .filter(|path| !exclude.contains(path))
        .cloned()
        .collect();

    log::info!("get_all_the_examples done\n");
    pathes
}

fn get_examples(path: &Path) -> Vec<String> {
    let mut examples: Vec<String> = vec![];
    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        if entry.path().ends_with("Cargo.lock") {
            continue;
        }
        if entry.path().ends_with("Cargo.toml") {
            continue;
        }

        if entry.path().is_dir() {
            if entry.path().ends_with("target") {
                continue;
            }
            examples.extend(get_examples(entry.path().as_path()));
            continue;
        }
        //dbg!(&entry);

        if entry.path().is_file() {
            examples.push(entry.path().into_os_string().into_string().unwrap());
            continue;
        }
    }
    examples
    //return Vec::from_iter( examples.iter().map(|s| s.clone().into_os_string().into_string().expect("Bad") ) );
}

fn get_imported_files(md_files: Vec<PathBuf>) -> Vec<String> {
    log::info!("get_imported_files");
    // println!("{:?}", md_files);
    // ![](examples/arrays/update_hash.rs)
    // let re = Regex::new(r"^!\[\]]\((.*)\)\s*$").unwrap();
    let mut imported_files = vec![];
    for filename in md_files {
        //println!("{:?}", filename);
        match File::open(filename.as_path()) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.unwrap();
                    if line.starts_with("![](") && line.ends_with(')') {
                        //println!("{}", &line[4..line.len()-1])
                        imported_files.push((line[4..line.len() - 1]).to_string());
                    }
                }
            }
            Err(error) => {
                log::error!("Failed opening file {filename:?}: {error}");
            }
        }
    }
    log::info!("get_imported_files done\n");
    return Vec::from_iter(imported_files.iter().map(|s| s.to_string()));
}

fn get_md_files() -> Vec<PathBuf> {
    log::info!("get_md_files");
    let mut md_files = vec![];
    let path = Path::new(".");
    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        let filename = entry.path();
        //println!("{:?}", filename); //.as_path());
        let extension = filename.extension();
        if let Some(value) = extension {
            if value == "md" {
                // println!("{:?}", filename);
                //println!("{}", filename);
                md_files.push(filename);
            }
        }
        //println!("{:?}", extension.unwrap())
    }

    log::info!("get_md_files done\n");
    md_files
}
