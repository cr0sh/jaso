# `jaso`
파일들의 이름을 Unicode NFC로 정규화해주는 프로그램입니다.

# 설치
```bash
brew install simnalamburt/x/jaso
```

# 사용법

> 참고: `jaso`는 병렬로 파일 이름을 바꾸기 때문에, 동시에 수많은 파일을 열 수 있습니다. `ulimit -n 65536` 으로 동시에 열 수 있는
> 파일 수를 높인 후 `jaso`를 실행하는 것을 권장합니다.

```
Usage: jaso [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Files to perform jaso merges

Options:
      --follow-directory-symlinks
  -v, --verbose
  -n, --dry-run
  -h, --help                       Print help information
  -V, --version                    Print version information
```

현재 위치한 디렉토리를 정규화하고 싶으면 `jaso .` 을 하시면 됩니다.

- `-v`: 정규화에 시간이 얼마나 걸렸는지와 같은 정보를 추가로 표시합니다.
- `--dry-run`: 파일명을 실제로 바꾸지 않고 어떻게 바꿀 것인지 표시만 수행합니다.
- `--follow-directory-symlinks`: 디렉토리 심링크를 따라갑니다.
  `-r`과 함께 사용하면 무한 루프에 빠질 수 있기 때문에 기본적으로 비활성화되어 있습니다.
- `-h`: 위 도움말을 출력합니다.

&nbsp;

--------
*jaso* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
