use clap::Parser;

// enum Total {
//     Auto,
//     Always,
//     Only,
//     Ever
// }

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long, short='c')]
    bytes: bool,

    #[arg(long, short='m')]
    chars: bool,

    #[arg(long, short)]
    lines: bool,

    // --files0-from automatically mapped to this
    // We can use both
    //       --files0-from  file.txt
    // and
    //       --files0-from=file.txt 
    #[arg(long)] //, alias="files0-from")]
    files0_from: Option<String>,

    // --max-line-length  is automatically mapped to this
    #[arg(long, short='L')]
    max_line_length: bool,

    #[arg(long, short)]
    words: bool,

    // #[arg(long)]
    // total: Total,
}

// TODO mutual exclusivity?
fn main() {
    let args = Cli::parse();
    if args.bytes {
        println!("print the byte counts");
    }
    if args.chars {
        println!("print the character counts");
    }
    if args.lines {
        println!("print the newline counts");
    }

    if let Some(files0_from) = args.files0_from {
        if files0_from == "-" {
            println!("If F is - then read names from standard input");
        } else {
            println!("read input from the files specified by NUL-terminated names in file F={}", files0_from);
        }
    }

    if args.max_line_length {
        println!("print the maximum display width");
    }

    if args.words {
        println!("print the word counts");
    }
    // --total=WHEN
    //           when to print a line with total counts; WHEN can be: auto, always, only, never

}
