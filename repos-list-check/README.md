# repos-list-check

Checks that repos.md contains all the actively maintained repos in the playframework and lagom organisations.

## Setup

Install Rust: <https://www.rust-lang.org/tools/install>

## Usage

Run `cargo run`.  Any missing repo will be printed, in copy-and-paste form for repos.md, e.g:

```markdown
- [playframework/foo](https://github.com/playframework/foo)
- [lagom/bar](https://github.com/lagom/bar)
```

## Detail

Note there is a list of "archived" repos in the code, that acts as a filter.
