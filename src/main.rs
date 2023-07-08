use std::path::PathBuf;

use async_recursion::async_recursion;
use clap::Parser;
use rlimit::Resource;
use tokio::{sync::Semaphore, time::Instant};
use unicode_normalization::{is_nfc, UnicodeNormalization};

const CONCURRENT_TASKS: usize = 32768;
static SEMA: Semaphore = Semaphore::const_new(CONCURRENT_TASKS);

/// jaso normalizes filenames to their Unicode NFC format in parallel
#[derive(Parser, Debug)]
#[clap(version, arg_required_else_help = true)]
struct Args {
    /// Follows symbolic links to directories.
    ///
    /// Note that current implementation of jaso allows infinite recursion due to cyclic symbolic
    /// links.
    #[arg(long)]
    follow_directory_symlinks: bool,
    /// Shows additional information, such as what files has been renamed.
    ///
    /// This option is useful for debugging or logging.
    #[arg(short, long)]
    verbose: bool,
    /// Just indicates what would be renamed, without actually renaming files.
    ///
    /// This option is useful for checking if normalization is needed. This option implies
    /// `--verbose` option.
    ///
    /// Note that it is possible that dry-run succeeds but actual run fails.
    #[arg(short = 'n', long)]
    dry_run: bool,
    /// Paths to normalize recursively.
    ///
    /// If a directory is given, all files in the directory will be normalized.
    /// If a symbolic link is given, the link itself will be normalized too.
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Automatically increase NOFILE rlimit to the allowed maximum
    rlimit::increase_nofile_limit(u64::MAX).expect("failed during increasing NOFILE rlimit");

    let nofile = rlimit::getrlimit(Resource::NOFILE).expect("cannot query rlimit");
    if nofile.0 < 1024 || nofile.1 < 1024 {
        eprintln!("warning: NOFILE resource limit is low(={nofile:?}), run `ulimit -n 65536` and try again if panic occurs");
    }

    if args.follow_directory_symlinks {
        eprintln!("warning: --follow_directory_symlinks is ON; be aware for infinite recursion!");
    }

    let start = Instant::now();

    let cnt = normalize_paths(
        args.follow_directory_symlinks,
        args.verbose,
        args.dry_run,
        args.paths.clone(),
    )
    .await;

    let elapsed = start.elapsed();

    eprintln!("DONE; {cnt} files in {} seconds", elapsed.as_secs_f64());
}

#[async_recursion]
async fn normalize_paths(
    follow_symlinks: bool,
    verbose: bool,
    dry_run: bool,
    paths: Vec<PathBuf>,
) -> u64 {
    let mut tasks = Vec::new();
    let mut cnt = 0;
    for p in paths {
        let _aq = SEMA.acquire();
        let task = tokio::task::spawn(async move {
            let mut cnt = 0;
            let mut p = if let Some(p) = p.to_str().map(str::to_owned) {
                p
            } else {
                eprintln!("warning: {} is not a valid UTF-8 path", p.to_string_lossy());
                return 0;
            };

            if !is_nfc(&p) {
                let newp = p.nfc().collect::<String>();
                if !dry_run {
                    match tokio::fs::rename(&p, &newp).await {
                        Ok(()) => {
                            if verbose {
                                eprintln!("success: {p} -> {newp}");
                            }
                            cnt += 1;
                        }
                        Err(e) => eprintln!("failure: {p} -> {newp}: {e}"),
                    }
                } else {
                    eprintln!("skip: {p} -> {newp}");
                }
                p = newp;
            }

            let p = PathBuf::from(p);
            if (!p.is_symlink() || follow_symlinks) && p.is_dir() {
                let mut paths = Vec::new();
                let mut dir = tokio::fs::read_dir(p).await.expect("cannot list directory");
                while let Ok(Some(d)) = dir.next_entry().await {
                    paths.push(d.path());
                }
                cnt += normalize_paths(follow_symlinks, verbose, dry_run, paths).await;
            }

            cnt
        });
        tasks.push(task);
    }

    for t in tasks {
        cnt += t.await.unwrap();
    }

    cnt
}
