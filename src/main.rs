use std::borrow::Cow;
use std::fs::rename;
use std::path::{Component, Path, PathBuf};
use std::thread;
use std::time::Instant;

use clap::Parser;
use crossbeam_channel::unbounded;
use rlimit::Resource;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(version, arg_required_else_help = true)]
struct Args {
    #[arg(long)]
    follow_symlinks: bool,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short = 'n', long)]
    dry_run: bool,
    /// Files to perform jaso merges
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Automatically increase NOFILE rlimit to the allowed maximum
    if let Err(e) = rlimit::increase_nofile_limit(u64::MAX) {
        eprintln!("warning: failed to increase NOFILE rlimit ({e})");
    }

    let nofile = rlimit::getrlimit(Resource::NOFILE).expect("cannot query rlimit");
    if nofile.0 < 1024 || nofile.1 < 1024 {
        eprintln!("warning: NOFILE resource limit is low(={nofile:?}), run `ulimit -n 65536` and try again if panic occurs");
    }

    let start = Instant::now();

    let (tx, rx) = unbounded::<PathBuf>();

    let thread_join_handle = thread::spawn(move || {
        let mut success = 0usize;
        let mut error = 0usize;
        while let Ok(path) = rx.recv() {
            let mut it = path.components();
            let Some(Component::Normal(oldname)) = it.next_back() else { continue }; // skip '..' or '/'
            let Some(oldname) = oldname.to_str() else { continue }; // skip non-unicode filename
            let Cow::Owned(newname) = normalize_into_nfc(oldname) else { continue }; // alloc or skip NFC

            let dirname = it.as_path();
            let Some(dirname) = dirname.to_str() else { continue }; // skip non-unicode dirname

            // NOTE: Actual dirname will be already NFC at this point but this thread can still see
            // NFD dirname. This is because directory traversal and renaming are done in separate
            // threads. So we should NFC-normalize the dirname to avoid TOCTOU.
            let dirname = normalize_into_nfc(dirname); // maybe alloc
            let dirname = Path::new(&*dirname);

            let old = dirname.join(oldname); // alloc
            let new = dirname.join(newname); // alloc

            if args.dry_run {
                success += 1;
                eprintln!("dryrun: {old:?} -> {new:?}");
                continue;
            }
            if let Err(e) = rename(&old, &new) {
                error += 1;
                eprintln!("error: {old:?} -> {new:?} failed with {e}");
                continue;
            }
            success += 1;
            if args.verbose {
                eprintln!("success: {old:?} -> {new:?}");
            }
        }
        (success, error)
    });

    for p in args.paths {
        for entry in WalkDir::new(p).follow_links(args.follow_symlinks) {
            let Ok(entry) = entry else {
                eprintln!("skip: {}", entry.unwrap_err());
                continue;
            };
            let path = entry.into_path();
            tx.send(path).unwrap(); // never fails
        }
    }
    drop(tx);
    let (success, error) = thread_join_handle
        .join()
        .expect("thread panicked unexpectedly");

    let elapsed = start.elapsed();
    if args.dry_run {
        eprintln!("{success} files will be renamed, took {elapsed:?}");
    } else {
        eprintln!("renamed {success} files, took {elapsed:?} seconds");
        if error > 0 {
            eprintln!("failed to rename {error} files");
            std::process::exit(1);
        }
    }
}

/// Return a string normalized in NFC. If the string is already in NFC, it is returned as-is.
///
/// This function is required to reduce repeated computation and memory allocation.
fn normalize_into_nfc(s: &str) -> Cow<str> {
    use unicode_normalization::{is_nfc_quick, IsNormalized, UnicodeNormalization};

    match is_nfc_quick(s.chars()) {
        IsNormalized::Yes => s.into(),
        IsNormalized::No => s.nfc().collect(),
        IsNormalized::Maybe => {
            let nfc: String = s.nfc().collect();
            if s == nfc {
                s.into()
            } else {
                nfc.into()
            }
        }
    }
}
