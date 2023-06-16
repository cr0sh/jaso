use std::path::PathBuf;

use async_recursion::async_recursion;
use clap::Parser;
use rlimit::Resource;
use tokio::{sync::Semaphore, time::Instant};
use unicode_normalization::{is_nfc, UnicodeNormalization};

const CONCURRENT_TASKS: usize = 32768;
static SEMA: Semaphore = Semaphore::const_new(CONCURRENT_TASKS);

#[derive(Parser, Debug)]
#[clap(version, rename_all = "kebab-case", arg_required_else_help = true)]
struct Args {
    #[arg(long)]
    follow_directory_symlinks: bool,
    #[arg(short, long)]
    verbose: bool,
    #[arg(long)]
    dry_run: bool,
    /// Files to perform jaso merges
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let nofile = rlimit::getrlimit(Resource::NOFILE).expect("cannot query rlimit");

    if nofile.0 < 1024 || nofile.1 < 1024 {
        eprintln!("warning: NOFILE resource limit is low(={nofile:?}), run ulimit -n 65536 and try again if panic occurs");
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
