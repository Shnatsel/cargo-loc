# `cargo loc`

Counts the lines of code across your entire dependency tree in a Cargo project. Powered by [`tokei`](https://github.com/XAMPPRocky/tokei).

## Installation

`cargo install cargo-loc`

## What works

 - Feature selection with `--no-default-features`, `--all-features`, `--features=...`
 - Restricting the analysis to specific platform, e.g. `--filter-platform=x86_64-unknown-linux-gnu`
 - Specifying the project with `--manifest-path`
 - Recognizes lots of languages, presents a breakdown by language
 - Telling code apart from comments and blank lines, in all those languages

## What doesn't work (yet?)

 - Excluding dev-dependencies and build-dependencies
 - Analyzing an entire Cargo workspace as opposed to a single crate (and its dependencies)
 - Fancy table output like in `tokei`
 - Configuration of any kind
 - Understanding which parts of a huge crate like `libc` you're actually using and counting accordingly (see [painter](https://github.com/rustfoundation/painter))

## Sample output

Here's `cargo loc` running on itself:

```
Top 20 largest depdendencies:
504205 lines (133025 code): encoding_rs v0.8.34
384808 lines (384747 code): windows-sys v0.52.0
180430 lines (177051 code): winapi v0.3.9
121145 lines (109166 code): libc v0.2.154
54867 lines (51333 code): syn v2.0.63
52523 lines (49170 code): regex-syntax v0.8.3
40453 lines (29483 code): regex-automata v0.4.6
25319 lines (19553 code): rayon v1.10.0
24113 lines (22640 code): pest v2.7.10
23105 lines (18998 code): chrono v0.4.38
20150 lines (16943 code): serde_json v1.0.117
19282 lines (10602 code): wasm-bindgen v0.2.92
17237 lines (10962 code): regex v1.10.4
17095 lines (12062 code): clap v2.34.0
16593 lines (13567 code): crossbeam-channel v0.5.12
16379 lines (13118 code): chrono-tz v0.8.6
15108 lines (11379 code): aho-corasick v1.1.3
14221 lines (11915 code): tera v1.19.1
13421 lines (9231 code): libm v0.2.8
12701 lines (10742 code): serde v1.0.201

Breakdown of the total lines by language:
Rust: 1436510
Plain Text: 369964
Markdown: 35129
TOML: 19082
C: 8803
HTML: 4028
JavaScript: 2790
Python: 2759
JSON: 1432
Makefile: 1361
C Header: 1153
F*: 830
Pest: 735
YAML: 238
Shell: 186
BASH: 183
ReStructuredText: 179
C++: 67
Dockerfile: 9
Pan: 3

Total lines: 1885441
(1355598 code, 444701 comments, 85142 blank lines)
```

Spooky!
