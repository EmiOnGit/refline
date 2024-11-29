# Refline

A tool for timed figure drawings and reference collections.

## Getting Started

You can use the justfile or simply compile with the rust compiler.
If you have a functioning rust compiler set up, you can use:
```bash
cargo r -r #to compile and run refline
cargo doc --open #to generate and open the projects documentation in your default pdf program
cargo 
```
A [justfile](./justfile) is included by default with common recipes used by other COSMIC projects. Install from [casey/just][just]

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP

## Documentation
You can generate the documentation for the project by running
```bash
cargo doc --open
```
For the libcosmic documentation refer to the [libcosmic API documentation][api-docs] and [book][book] for help with building applications with [libcosmic][libcosmic].

[api-docs]: https://pop-os.github.io/libcosmic/cosmic/
[book]: https://pop-os.github.io/libcosmic-book/
[libcosmic]: https://github.com/pop-os/libcosmic/
[just]: https://github.com/casey/just
