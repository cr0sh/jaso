jaso
========
jaso normalizes filenames to their Unicode NFC format in parallel, and is much
faster than [convmv(1)].

```console
$ jaso
jaso normalizes filenames to their Unicode NFC format in parallel

Usage: jaso [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Paths to normalize recursively

Options:
      --follow-directory-symlinks  Follows symbolic links to directories
  -v, --verbose                    Shows additional information, such as what files has been renamed
  -n, --dry-run                    Just indicates what would be renamed, without actually renaming files
  -h, --help                       Print help information (use `--help` for more detail)
  -V, --version                    Print version information

$ jaso .
DONE; 100 files in 1.111529301 seconds
```

### Installation
Using Homebrew in macOS:
```bash
brew install simnalamburt/x/jaso
```

Using Cargo:
```bash
cargo install jaso
```

&nbsp;

--------
*jaso* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[convmv(1)]: https://linux.die.net/man/1/convmv
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
