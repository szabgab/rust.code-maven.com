use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::sync::mpsc;

use chrono::{DateTime, Utc};
use clap::Parser;
use threadpool::ThreadPool;

// use regex::Regex;

// from all the md files extract the list of included files
// from the examples/ directory list all the files
// make sure each file is included and is only included once

// Run every rs file, if there is an out file compare the results.
//
const ROOT: &str = "../../..";

const ACTIONS: [&str; 6] = ["update", "fmt", "fmt_check", "clippy", "test", "run"];

#[derive(Parser)]
struct Cli {
    #[arg(long, help = "Print debug information")]
    verbose: bool,

    #[arg(long, help = "Cleanup the target directory")]
    cleanup: bool,

    #[arg(long, help = "Check if all the examples are used in the md files")]
    use_examples: bool,

    #[arg(long)]
    update: bool,

    #[arg(long)]
    fmt: bool,

    #[arg(long)]
    fmt_check: bool,

    #[arg(long)]
    clippy: bool,

    #[arg(long)]
    test: bool,

    #[arg(long)]
    run: bool,

    #[arg(help = "List of examples to run. e.g examples/other/check-slides/")]
    examples: Vec<String>,
}

fn get_actions(args: &Cli) -> Vec<&str> {
    let mut actions: Vec<&str> = vec![];
    if args.update {
        actions.push("update");
    }
    if args.fmt {
        actions.push("fmt")
    }
    if args.fmt_check {
        actions.push("fmt_check")
    }
    if args.clippy {
        actions.push("clippy")
    }
    if args.test {
        actions.push("test")
    }
    if args.run {
        actions.push("run")
    }

    actions
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
        args.examples.iter().map(PathBuf::from).collect()
    };
    log::info!("Number of examples: {}", examples.len());

    let unused_examples = check_use_of_example_files(args.use_examples);

    let mut success: HashMap<String, i32> = HashMap::new();
    let mut failures: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let actions = get_actions(&args);

    cargo_on_all(
        &mut success,
        &mut failures,
        &examples,
        actions.iter().map(|x| x.to_string()).collect(),
        args.cleanup,
    );

    let mut failures_total = 0;
    for action in ACTIONS {
        if success.contains_key(action) && failures.contains_key(action) {
            log::info!(
                "{action} success: {}, failure: {}",
                success[action],
                failures[action].len()
            );
            failures_total += failures[action].len();
        }
    }

    println!("------- Report -------");
    let end: DateTime<Utc> = Utc::now();
    println!("Elapsed: {}", end.timestamp() - start.timestamp());

    if !unused_examples.is_empty() {
        println!("There are {} unused examples", unused_examples.len());
        for example in &unused_examples {
            println!("  {example:?}",);
        }
    }

    for action in ACTIONS {
        if failures.contains_key(action) {
            report_errors(action, &failures[action]);
        }
    }

    if !unused_examples.is_empty() || failures_total > 0 {
        exit(1);
    }
}

fn report_errors(name: &str, failures: &[PathBuf]) {
    if !failures.is_empty() {
        println!("There are {} examples with {name} errors.", failures.len());
        for failure in failures {
            println!("  {failure:?}",);
        }
    }
}

fn check_use_of_example_files(use_examples: bool) -> Vec<String> {
    let mut unused_examples = vec![];
    if !use_examples {
        return unused_examples;
    }
    log::info!("check_use_of_example_files");
    let md_files = get_md_files();
    let imported_files = get_imported_files(md_files);
    let examples = get_all_the_examples();

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
            if filename.starts_with("examples/surrealdb/embedded-rocksdb/tempdb/") {
                continue;
            }
            if filename.starts_with("examples/surrealdb/cli-multi-counter/db/") {
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
            unused_examples.push(filename);
        }
    }
    unused_examples
}

fn cargo_on_all(
    success: &mut HashMap<String, i32>,
    failures: &mut HashMap<String, Vec<PathBuf>>,
    crates: &[PathBuf],
    actions: Vec<String>,
    cleanup: bool,
) {
    log::info!("cargo_on_all {actions:?} START");
    let start: DateTime<Utc> = Utc::now();
    let number_of_crates = crates.len();

    let (tx, rx) = mpsc::channel();
    let max_threads = 2;
    let pool = ThreadPool::new(max_threads);

    for (ix, crate_folder) in crates.iter().cloned().enumerate() {
        log::info!("crate: {}/{number_of_crates}, {crate_folder:?}", ix + 1);
        let mytx = tx.clone();
        let actions = actions.clone();

        pool.execute(move || {
            let res =
                cargo_actions_on_single(&crate_folder, &actions, cleanup, ix + 1, number_of_crates);
            log::debug!("sending res {res:?}");

            mytx.send((res, crate_folder)).unwrap();
        });
    }
    drop(tx); // close this channel to allow the last thread to finish the receiving loop

    for received in rx {
        for action in &actions {
            if received.0[action] {
                log::debug!("success {action}");
                *success.entry(action.clone()).or_insert(0) += 1;
            } else {
                log::debug!("received failures {:?}", received.1.clone());
                failures
                    .entry(action.clone())
                    .or_default()
                    .push(received.1.clone());
                log::debug!("all failures {:?}", failures);
            }
        }
    }

    let end: DateTime<Utc> = Utc::now();
    log::info!(
        "cargo_on_all {actions:?} DONE Elapsed: {}",
        end.timestamp() - start.timestamp()
    );
}

fn cargo_actions_on_single(
    crate_folder: &PathBuf,
    actions: &[String],
    cleanup: bool,
    ix: usize,
    total: usize,
) -> HashMap<String, bool> {
    log::info!("Actions on {crate_folder:?} START {ix}/{total}");
    let start = Utc::now();
    let mut res = HashMap::new();
    for action in actions {
        res.insert(action.clone(), cargo_on_single(crate_folder, action));
    }
    if cleanup {
        std::fs::remove_dir_all(crate_folder.join("target")).unwrap();
    }
    let end = Utc::now();
    log::info!(
        "Actions on {crate_folder:?} DONE {ix}/{total} Elapsed: {}",
        end.timestamp() - start.timestamp()
    );
    res
}

fn cargo_on_single(crate_path: &PathBuf, action: &str) -> bool {
    log::info!("{action} on {crate_path:?} START");

    let args = get_args(action);
    let skip = skip(action);

    let folder = crate_path.clone().into_os_string().into_string().unwrap();
    let folders = skip.iter().map(|x| x.to_string()).collect::<String>();
    if folders.contains(&folder) {
        return true;
    }

    let error = format!("failed to execute 'cargo {args:?} --check' process");
    let mut cmd = Command::new("cargo");
    for arg in args {
        cmd.arg(arg);
    }

    let result = cmd.current_dir(crate_path).output().expect(&error);

    log::info!("{action} on {crate_path:?} DONE");
    if !result.status.success() {
        let code = result.status.code().unwrap();
        log::error!("Cannot execute {args:?} on crate: {crate_path:?} exit code: {code} stdout: {} stderr: {}", std::str::from_utf8(&result.stdout).unwrap(), std::str::from_utf8(&result.stderr).unwrap());
        return false;
    }
    true
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

fn skip(name: &str) -> &'static [&'static str] {
    let skip_update = &[
        "examples/threads/map-with-thread", // error: no matching package named `threaded-map` found
    ];

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

    let skip_run = &[
        "examples/surrealdb/connect-to-server", // needs a SurrealDB server to run
        // clap examples are expecting command line parameters
        "examples/clap/clap-complete-shell",
        "examples/clap/clap-example",
        "examples/clap/default-value-if-equals",
        "examples/clap/default-value-if-equals-multiple-values",
        "examples/clap/default-value-if-ispresent",
        "examples/clap/enumerated",
        "examples/clap/environment-variable",
        "examples/clap/even-number",
        "examples/clap/help-with-arguments",
        "examples/clap/limit-number-of-args",
        "examples/clap/mutually-exclusive-group",
        "examples/clap/mutually-exclusive-or",
        "examples/clap/number-string-bool",
        "examples/clap/repeat-the-same-argument",
        "examples/clap/several-positional-arguments",
        "examples/clap/short-arguments",
        "examples/clap/show-description",
        "examples/clap/show-description-from-code",
        "examples/clap/show-generated-description",
        "examples/clap/show-version-number",
        "examples/clap/single-long-argument",
        "examples/clap/single-positional-argument",
        "examples/clap/subcommands",
        "examples/clap/validate-number-range",
        "examples/clap/value-name",
        "examples/clap/value-parser-fixed-list",
        "examples/clap/wc-cli",
        // rocket examples are all running a server without stopping so we can't run them in the CI
        "examples/rocket/blog-with-fromparam",
        "examples/rocket/blog-with-guard",
        "examples/rocket/calculator-with-get",
        "examples/rocket/catch-division-by-zero",
        "examples/rocket/configuration",
        "examples/rocket/config-with-tests",
        "examples/rocket/create-tera-filter",
        "examples/rocket/echo-using-get",
        "examples/rocket/echo-using-post",
        "examples/rocket/generated-rawhtml",
        "examples/rocket/guards",
        "examples/rocket/hello-world",
        "examples/rocket/hello-world-external-test-file",
        "examples/rocket/hello-world-html",
        "examples/rocket/hello-world-tera-template",
        "examples/rocket/http-404-page-with-static-content",
        "examples/rocket/in-memory-counter",
        "examples/rocket/logging",
        "examples/rocket/logging-with-log4rs-to-file",
        "examples/rocket/multi-counter-using-cookies",
        "examples/rocket/multi-counter-using-encrypted-cookies",
        "examples/rocket/path-parameters",
        "examples/rocket/people-and-groups",
        "examples/rocket/redirect-to-fixed-url",
        "examples/rocket/redirect-with-parameters",
        "examples/rocket/request-guard",
        "examples/rocket/return-result",
        "examples/rocket/return-result-user-id",
        "examples/rocket/separate-files",
        "examples/rocket/set-cookie",
        "examples/rocket/simple-todo-with-surrealdb",
        "examples/rocket/single-counter-in-text-file",
        "examples/rocket/skip-route",
        "examples/rocket/skip-route-using-guard",
        "examples/rocket/static-files",
        "examples/rocket/userid-in-path",
        "examples/rocket/use-tera-filter",
    ];

    let skip_test = &[
        "examples/rocket/return-result-user-id", // TODO
        "examples/rocket/userid-in-path",        // TODO
        "examples/rocket/skip-route",            // TODO
    ];

    if name == "update" {
        return skip_update;
    }
    if name == "clippy" {
        return skip_clippy;
    }
    if name == "run" {
        return skip_run;
    }
    if name == "test" {
        return skip_test;
    }

    &[]
}

fn get_args(action: &str) -> &'static [&'static str] {
    if action == "clippy" {
        return &["clippy", "--", "--deny", "warnings"];
    }
    if action == "update" {
        return &["update"];
    }
    if action == "fmt" {
        return &["fmt"];
    }
    if action == "fmt_check" {
        return &["fmt", "--check"];
    }
    if action == "test" {
        return &["test"];
    }
    if action == "run" {
        return &["run"];
    }

    panic!("Unknown action: {action}");
}
