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
    - Cargo.toml  is a manifest (similiar a package.json file in Javascript projects)
    - This is called a manifest, and it contains all of the metadata that Cargo needs to compile your package. This file is written in the TOML format (pronounced /tɑməl/).
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

6. **Modules**
    alloc	            Memory allocation APIs.
    any	                Utilities for dynamic typing or type reflection.
    arch	            IMD and vendor intrinsics module.
    array	            Utilities for the array primitive type.
    ascii	            Operations on ASCII strings and characters.
    backtrace	        Support for capturing a stack backtrace of an OS thread
    borrow	            A module for working with borrowed data.
    boxed	            The Box<T> type for heap allocation.
    cell	            Shareable mutable containers.
    char	            Utilities for the char primitive type.
    clone	            The Clone trait for types that cannot be ‘implicitly copied’.
    cmp	                Utilities for comparing and ordering values.
    collections	        Collection types.
    convert	            Traits for conversions between types.
    default	            The Default trait for types with a default value.
    env	                Inspection and manipulation of the process’s environment.
    error	            Interfaces for working with Errors.
    f32	                Constants for the f32 single-precision floating point type.
    f64	                Constants for the f64 double-precision floating point type.
    ffi	                Utilities related to FFI bindings.
    fmt	                Utilities for formatting and printing Strings.
    fs	                Filesystem manipulation operations.
    future	            Asynchronous basic functionality.
    hash	            Generic hashing support.
    hint	            Hints to compiler that affects how code should be emitted or optimized. Hints may be compile time or runtime.
    io	                Traits, helpers, and type definitions for core I/O functionality.
    iter	            Composable external iteration.
    marker	            Primitive traits and types representing basic properties of types.
    mem	                Basic functions for dealing with memory.
    net	                Networking primitives for TCP/UDP communication.
    num	                Additional functionality for numerics.
    ops	                Overloadable operators.
    option	            Optional values.
    os	                OS-specific functionality.
    panic	            Panic support in the standard library.
    path	            Cross-platform path manipulation.
    pin	                Types that pin data to a location in memory.
    prelude	            The Rust Prelude
    primitive	        This module reexports the primitive types to allow usage that is not possibly shadowed by other declared types.
    process	            A module for working with processes.
    ptr	                Manually manage memory through raw pointers.
    rc	                Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
    result	            Error handling with the Result type.
    slice	            Utilities for the slice primitive type.
    str	                Utilities for the str primitive type.
    string	            A UTF-8–encoded, growable string.
    sync	            Useful synchronization primitives.
    task	            Types and Traits for working with asynchronous tasks.
    thread	            Native threads.
    time	            Temporal quantification.
    vec	                A contiguous growable array type with heap-allocated contents, written Vec<T>.

https://doc.rust-lang.org/std/index.html#modules

7. **Macros**
    assert
    Asserts that a boolean expression is true at runtime.
    assert_eq
    Asserts that two expressions are equal to each other (using PartialEq).
    assert_ne
    Asserts that two expressions are not equal to each other (using PartialEq).
    cfg
    Evaluates boolean combinations of configuration flags at compile-time.
    column
    Expands to the column number at which it was invoked.
    compile_error
    Causes compilation to fail with the given error message when encountered.
    concat
    Concatenates literals into a static string slice.
    dbg
    Prints and returns the value of a given expression for quick and dirty debugging.
    debug_assert
    Asserts that a boolean expression is true at runtime.
    debug_assert_eq
    Asserts that two expressions are equal to each other.
    debug_assert_ne
    Asserts that two expressions are not equal to each other.
    env
    Inspects an environment variable at compile time.
    eprint
    Prints to the standard error.
    eprintln
    Prints to the standard error, with a newline.
    file
    Expands to the file name in which it was invoked.
    format
    Creates a String using interpolation of runtime expressions.
    format_args
    Constructs parameters for the other string-formatting macros.
    include
    Parses a file as an expression or an item according to the context.
    include_bytes
    Includes a file as a reference to a byte array.
    include_str
    Includes a UTF-8 encoded file as a string.
    is_x86_feature_detected
    A macro to test at runtime whether a CPU feature is available on x86/x86-64 platforms.
    line
    Expands to the line number on which it was invoked.
    matches
    Returns whether the given expression matches the provided pattern.
    module_path
    Expands to a string that represents the current module path.
    option_env
    Optionally inspects an environment variable at compile time.
    panic
    Panics the current thread.
    print
    Prints to the standard output.
    println
    Prints to the standard output, with a newline.
    stringify
    Stringifies its arguments.
    thread_local
    Declare a new thread local storage key of type std::thread::LocalKey.
    todo
    Indicates unfinished code.
    tryDeprecated
    Unwraps a result or propagates its error.
    unimplemented
    Indicates unimplemented code by panicking with a message of “not implemented”.
    unreachable
    Indicates unreachable code.
    vec
    Creates a Vec containing the arguments.
    write
    Writes formatted data into a buffer.
    writeln
    Write formatted data into a buffer, with a newline appended.
    cfg_matchExperimental
    A macro for defining #[cfg] match-like statements.
    concat_bytesExperimental
    Concatenates literals into a byte slice.
    concat_identsExperimental
    Concatenates identifiers into one identifier.
    const_format_argsExperimental
    Same as format_args, but can be used in some const contexts.
    format_args_nlExperimental
    Same as format_args, but adds a newline in the end.
    log_syntaxExperimental
    Prints passed tokens into the standard output.
    trace_macrosExperimental
    Enables or disables tracing functionality used for debugging other macros.

https://doc.rust-lang.org/std/index.html#macros