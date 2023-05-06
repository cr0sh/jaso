# `jaso`
파일들의 이름을 Unicode NFC로 정규화해주는 프로그램입니다.

# 설치
1. https://rustup.rs 에서 Rust와 Cargo를 설치합니다.
2. 쉘을 재시작합니다 (`exec $SHELL` 혹은 터미널 종료 후 재시작).
3. `cargo install --git https://github.com/cr0sh/jaso.git`

업데이트는 `cargo install -f --git https://github.com/cr0sh/jaso.git` 으로 할 수 있습니다.

# 사용법

> 참고: `jaso`는 병렬로 파일 이름을 바꾸기 때문에, 동시에 수많은 파일을 열 수 있습니다. `ulimit -n 65536` 으로 동시에 열 수 있는
> 파일 수를 높인 후 `jaso`를 실행하는 것을 권장합니다.

```
Usage: jaso [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  Files to perform jaso merges

Options:
  -r, --recursive
      --follow-directory-symlinks
  -v, --verbose
      --dry-run
  -h, --help                       Print help
```

현재 위치한 디렉토리를 정규화하고 싶으면 `jaso -rv .` 을 하시면 됩니다.

- `-r`: 파일 탐색 중 디렉토리를 만나면 안쪽 파일과 그 안쪽... 까지 모두 정규화를 수행합니다.
- `-v`: 정규화에 시간이 얼마나 걸렸는지와 같은 정보를 추가로 표시합니다.
- `--dry-run`: 파일명을 실제로 바꾸지 않고 어떻게 바꿀 것인지 표시만 수행합니다.
- `--follow-directory-symlinks`: 디렉토리 심링크를 따라갑니다.
  `-r`과 함께 사용하면 무한 루프에 빠질 수 있기 때문에 기본적으로 비활성화되어 있습니다.
- `-h`: 위 도움말을 출력합니다.

# 라이센스

이 소프트웨어는 퍼블릭 도메인으로, Unlicense 라이센스를 따릅니다.

```
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
```
