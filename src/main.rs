
extern crate getopts;

use getopts::Options;
use std::io::Write;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    // chamless copied from rust-coreutils/src/chmod/chmod.rs
    let mut opts = Options::new();
    opts.optflag("c", "changes", "like verbose but report only when a change is made (unimplemented)");
    opts.optflag("", "no-preserve-root", "do not treat '/' specially (the default)");
    opts.optflag("", "preserve-root", "fail to operate recursively on '/'");
    opts.optflagopt("", "reference", "use RFILE's mode instead of MODE values", "RFILE");
    opts.optflag("R", "recursive", "change files and directories recursively");
    opts.separator();
    opts.description_header("Misc Options");
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("V", "version", "output version information and exit");
    opts.separator();
    opts.description_header("Output Options");
    opts.optflag("f", "quiet", "suppress most error messages (unimplemented)"); // TODO: support --silent
    opts.optflag("v", "verbose", "output a diagnostic for every file processed (unimplemented)");
    opts.separator();
    opts.description("Some example longish text. Some example longish text. \
                     Some example longish text. Some example longish text. \
                     Some example longish text. Some example longish text.");

    let mut matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => panic!(why),
    };

    if matches.opt_present("help") {
        let msg = format!("{name} {version}

Usage:
  {program} [OPTION]... MODE[,MODE]... FILE...
  {program} [OPTION]... OCTAL-MODE FILE...
  {program} [OPTION]... --reference=RFILE FILE...
Change the mode of each FILE to MODE.
With --reference, change the mode of each FILE to that of RFILE.
Each MODE is of the form '[ugoa]*([-+=]([rwxXst]*|[ugo]))+|[-+=]?[0-7]+'.",
            name = "chmod", version = "0.1.0", program = "chmod");
    print!("{}", opts.usage(&msg));
    }
}
