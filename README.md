# Rust Explorer

This is a Rust project for explore Rust language.

Why Rust?
    - First-class multithreading
    - Compiler error to improperly access shared data
    - Can uncover bugs at compile time
    - Makes refactoring simple
    - Reduces the number of tests needed
    - Module system makes code separation simple
    - Adding a dependency is 1 line in a config file
    - Generate docs, lint code, auto format
    - Performance
    - Program behaviors can be enforced at compile time
    - Enhanced program reliability
    - Built-in dependency management, similar to npm
    - Quickly growing ecosystem of libraries
    - Friendly & welcoming developer community

Docs: https://doc.rust-lang.org

## Getting Started

1. **Install:**
    - Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
    - Check installation: `rustc --version` and `cargo -V`
    - Update Rust version: `rustc update`

    Tools installed:
        - cargo: package manager (Cargo is the Rust package manager. It is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build.)
        - clippy: linter 
        - rustc: rust standard lib
        - rustfmt: format code according style guide

2. **Project:**
    - Create project: `cargo new <project-name>` (`--bin` creates a binary, `--lib` creates a library)
        Note: cargo new initializes a new git repository by default. If you don't want it to do that, pass --vcs none
    - Run: `cargo run`
    - Build: `cargo build`
    - Build release: `cargo build --release`

3. **Setup VS for Rust:**
    - rust-analyzer: for Rust language support
    - Better TOML: for Better TOML language support
    - Error Lens: for highlighting errors, giving warnings and showing other language diagnostics.
    - crates: for managing Rust dependencies with Cargo.toml

4. **Cargo.toml, package and dependencies**
    - See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

5. **Commands**
    add                  Add dependencies to a Cargo.toml manifest file
    b                    alias: build
    bench                Execute all benchmarks of a local package
    build                Compile a local package and all of its dependencies
    c                    alias: check
    check                Check a local package and all of its dependencies for errors
    clean                Remove artifacts that cargo has generated in the past
    clippy               Checks a package to catch common mistakes and improve your Rust code.
    config               Inspect configuration values
    d                    alias: doc
    doc                  Build a package's documentation
    fetch                Fetch dependencies of a package from the network
    fix                  Automatically fix lint warnings reported by rustc
    fmt                  Formats all bin and lib files of the current crate using rustfmt.
    generate-lockfile    Generate the lockfile for a package
    help                 Displays help for a cargo subcommand
    read-manifest        Print a JSON representation of a Cargo.toml manifest.
    remove               Remove dependencies from a Cargo.toml manifest file
    report               Generate and display various kinds of reports
    rm                   alias: remove
    run                  Run a binary or example of the local package
    rustc                Compile a package, and pass extra options to the compiler
    rustdoc              Build a package's documentation, using specified custom flags.
    search               Search packages in crates.io
    t                    alias: test
    test                 Execute all unit and integration tests and build examples of a local package
    tree                 Display a tree visualization of a dependency graph
    uninstall            Remove a Rust binary
    update               Update dependencies as recorded in the local lock file
    vendor               Vendor all dependencies for a project locally
    verify-project       Check correctness of crate manifest
    version              Show version information
    yank                 Remove a pushed crate from the index

sources:
https://doc.rust-lang.org/stable/cargo/commands/