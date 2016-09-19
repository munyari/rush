<a href='https://www.recurse.com' title='Made with love at the Recurse Center'>
<img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a> 
[![GPL Licence](https://badges.frapsoft.com/os/gpl/gpl.svg?v=103)][license]

# rush: The Rust shell

A simple Unix shell written in Rust.

This was my first fairly large project in Rust. Some features that may on the roadmap:

- [ ] Tab completion
- [ ] Syntax highlighting
- [ ] Recognize POSIX sh
- [ ] History

If you're interested in shells written in Rust, check out [Ion][ion] from the Redox OS team.

[ion]: https://github.com/redox-os/ion

# Requirements

This should work on any platform that supports Rust and Cargo, but has only
been tested on GNU/Linux.

- [Rust][rust], version >= 1.6.0
- [Cargo][cargo], version >= 0.10.0

[rust]: https://github.com/rust-lang/rust
[cargo]: https://github.com/rust-lang/cargo

# Building
In the root directory of this project, just run `cargo build`

# Testing
To run the tests, run `cargo test`

# Contributing
Contributions are welcome under the terms of the GNU Public License, version 3.
Please see [LICENSE][license] for more details. Please discuss any major
changes in an issue before opening a large pull request, and keep pull requests
small. Ensure that all tests are passing before opening a pull request, and
that any new functionality is tested.

[license]: https://github.com/munyari/rush/blob/master/LICENSE
