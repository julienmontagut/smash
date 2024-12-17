# Kosh

Kosh is an opinionated shell with a focus on simplicity and ease of use.

🚧 This is a work in progress aimed at exercising my rust skills. 🚧

ANSI Codes are supported in:

- Windows Terminal (ANSI, UTF-8, $TERM=xterm-256color)
  - CMD since Windows 10 version 1909 also supports ANSI. No effort is required for ealier versions.
- macOS Terminal (ANSI, UTF-8, $TERM=xterm-256color)
- Gnome Terminal (UTF-8)

The aim is for the code to be as clean as possible. Therefore, the supported
features shall be implemented on top of features available in the following.

However, the terminal must also be able to function in the Linux Console.

- [ ] Test in the linux console, ANSI codes, keyboard events, mouse events and UTF-8 support.

## Continuous delivery

On each commit to the main branch, clippy, fmt and unit tests are run.
If they pass, a new release is created and published to crates.io.
The version number is automatically incremented based on the commit message.
If the commit message contains `BREAKING CHANGE`, the major version is incremented.
If the commit message contains `feat`, the minor version is incremented.
Otherwise, the patch version is incremented.

Cargo release is used to create the release.

The release is published to crates.io using the `cargo publish` command.
The release is also published to GitHub Releases.

## Features

- [x] Basic shell
- [ ] Command line editing
- [ ] Command history
- [ ] Command completion and suggestions
- [ ] Syntax highlighting
- [ ] Beautiful prompt
- [ ] Aliases
- [ ] POSIX compliance <https://pubs.opengroup.org/onlinepubs/9699919799/>
- [ ] Auto-install missing commands
- [ ] Smart prompt handling git repos, rust projects, etc.
- [More](https://www.gnu.org/software/bash/manual/html_node/Basic-Shell-Features.html#Basic-Shell-Features) to come...

## Ideas

- [ ] Auto-install missing commands
- [ ] Auto-update outdated commands (requires integration of a package manager)
- [ ] Smart prompt handling git repos, rust projects, etc.

## Interesting links

[256 Colors - Cheat Sheet - Xterm, HEX, RGB, HSL](https://www.ditig.com/256-colors-cheat-sheet)

[ANSI Escape Codes](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)

[Mode Functions (The GNU C Library)](https://www.gnu.org/software/libc/manual/html_node/Mode-Functions.html#Mode-Functions)

[Top (The GNU C Library)](https://www.gnu.org/software/libc/manual/html_node/index.html)

[termios(3) - Linux manual page](https://www.man7.org/linux/man-pages/man3/tcgetattr.3.html)
