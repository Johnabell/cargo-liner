# [Version 0.4.1 (21/09/2023)](https://crates.io/crates/cargo-liner/0.4.1)
## Fixes

 * Fixed #5: before, the Cargo `.crates.toml` file would be read even if
   `--skip-check` was specified although it only used its contents in order to
   adjust a small part of the logging display that was somewhat useless; the
   option now also ensures reading from this file is avoided as well, which
   makes it even more able to work around some potential future bugs.
 * Fixed #4: the `$CARGO_INSTALL_ROOT` configuration possibility was previously
   not supported by the `.crates.toml`-reading operations, making the tool thus
   completely fail whenever used in such an environment; it now supports it
   entirely by wrapping calls to `cargo config get` in order to retrieve its
   value from either the environment or the `install.root` configuration key of
   the `$CARGO_HOME/config.toml` file, and falling back to the default
   `$CARGO_HOME/.crates.toml` if that fails for any reason, the simple absence
   of the setting being one of them. Whenever the first fails, it is logged as
   a `DEBUG` message before attempting the default, so use `-vv` to investiguate
   if your configuration seems not to be taken into account.

## Documentation

 * `README.md`:
   * Updated the verbatim help message of the `ship` command.
   * Simplified the description of the default command's operation to be a bit
     more summarized and then redirect to the more in-depth description found
     in the `ship` command's specific documentation section.
   * Mention the new `$CARGO_INSTALL_ROOT` support in various places.
   * Reformat the configuration examples to wrap long lines: better readability.
   * Some small rewordings.

## Miscellaneous

 * The dependencies have been updated.
 * Logging now uses pretty debug formatting, which splits arrays and structures
   onto multiple lines, therefore making reading long messages more comfortable.
 * The MSRV has been bumped to `1.70.0`.


# [Version 0.4.0 (28/06/2023)](https://crates.io/crates/cargo-liner/0.4.0)
## Features

 * The log messages will now display whether a package is already installed
   upon running the `ship` command when it is configured: either `Installing`
   when it was not previously installed or `Updating` on the contrary will be
   mentionned in the messages.
 * The `ship` command now uses `cargo search` in order to fetch the latest
   available version for each configured package before calling `cargo install`
   only for each of them that do indeed need an install or update when either
   not installed or installed with a version strictly older than the latest.
   The calls to `cargo search` are done in parallel, making this version check
   quite fast, therefore saving a lot of time by avoiding calling `cargo
   install` sequentially for each already-up-to-date package.
 * The `ship` command now displays a summarized update plan before running the
   actually necessary calls to `cargo install`. That makes the overall
   operation a bit more transparent and readable.
 * A new `--skip-check` option flag has been added to the `ship` command. As
   its name suggests, it enables one to skip the version check entirely. That
   means it will restore the previous simple behavior: run `cargo install` for
   each configured package, whether it needs an install or update or not. Most
   of the time, when updating already-installed packages for example, it will
   not prove very useful. However, it can be a bit quicker to use it when only
   very few packages are configured or if all or almost all are not alredy
   installed. It overall enables one to have a bit more control over the global
   operation of the tool, in case of an unexpected bug for example.
 * The new `--force` option flag has been implemented for the `ship` command.
   It simply passes it onto each call to `cargo install`: see its documentation
   for more information about it. When used in conjunction with `--skip-check`
   for example, it will redownload, recompile and reinstall each of the
   configured packages: use it only when truly necessary.

## Documentation

 * `README.md`:
   * Updated the example output to reflect the changes from the previous
     release but from this one as well.
   * Removed some useless trailing spaces.
   * New features.

## Testing

 * New tests have been added for the new CLI features, but also for the new
   functions wrapping calls to `cargo search`.
 * The GitHub Actions CI is now configured to skip the tests for these new
   search functions: the corresponding network calls seem to be filtered
   somehow.


# [Version 0.3.0 (19/06/2023)](https://crates.io/crates/cargo-liner/0.3.0)
## Features

 * Package definitions now have a detailed variant available, mimicking the
   `Cargo.toml` format. This has been implemented in order to enable one to
   specify which features should be compiled in the installed packages, by
   configuring a list of feature flags in a similar format to the Cargo
   manifest's `[features]` section. Many thanks to @MaeIsBad: #2.
 * A new `--only-self` option has been added to the `ship` command: it enables
   one to only update `cargo-liner` and nothing else. It is incompatible with
   `--no-self`.
 * Short variants of the options flags of the `ship` command are now enabled.
 * A new `--keep-self` option has been added to the `import` command: it
   ensures the `cargo-liner` package is kept in the package list and thus
   written to the created configuration file. It is compatible with the version
   operator selection options.
 * Basic logging has been implemented. Messages will therefore now be displayed
   in order to inform the user about the currently performed action and its
   results. Errors bubbling up are manually caught in the global main function
   and logged using the same format as the rest of the emitted messages.
 * New `--verbose` and `--quiet` option flags have been implemented. They
   control the global verbosity of the program by changing the logging
   configuration at runtime. They are both available globally, i.e. with or
   without a command and before or after the command name. They can both be
   repeated up to three times in order to adjust how much the verbosity is
   changed. They are incompatible with each other. It is made sure they arrive
   last in the CLI help messages, but still just before `--help` and
   `--version`.

## Fixes

 * The `--no-self-update` option of the `ship` command has been renamed to
   `--no-self`. This makes it more consistent with other new option flags.
 * The TOCTOU bug for the configuration file import has been fixed: between
   checking for the existence of the file and then possibly overwriting it, a
   new file could appear. The `--force` option flag now controls which
   operation is used: when present, the previously always used overwriting is
   called; when absent, an appropriate system call is made to ensure only
   creating a new file is possible atomically.
 * Fixed some Clippy warnings triggered on the test suite.

## Documentation

 * `README.md`:
   * Fixed the CI status badge.
   * Added missing shell syntax highlighting for the code block showing the
     output example.
   * Clarified CLI option defaults.
   * Updated outputs.
   * Various other small adjustments.
 * New features.

## Testing

 * The GitHub CI workflow now denies `rustdoc` warnings by default. Thanks
   to @MaeIsBad: #3.
 * Appropriate tests have been added alongside the new functionalities.


# [Version 0.2.1 (26/11/2022)](https://crates.io/crates/cargo-liner/0.2.1)
## Fixes

 * The `import` sub-command now filters this project's package name out of the
   packages when importing. Avoids fixing to a version needlessly or adding a
   useless line considering the already-implemented self-update feature. Tests
   now also check for this bug.
 * Added the missing `wrap_help` feature to Clap in order to have prettier help
   and usage messages: they now adapt themselves to the current terminal size.

## Documentation

 * `README.md`:
   * Fixed import suggestion in installation steps.
   * Made a proper Summary section.
   * Fixed a small punctuation mistake.
   * Added an example output for the `ship` sub-command.
   * Added appropriate links to badges.
 * `CHANGELOG.md`: added links to crates.io versions.
 * Repository: added issue templates.


# [Version 0.2.0 (20/11/2022)](https://crates.io/crates/cargo-liner/0.2.0)
## Features

 * Self-updating enabled by default.
 * New `ship` subcommand to materialize the default behavior.
   * New `--no-self-update` option to disable the by-default self-updating.
 * New `import` subcommand to convert `$CARGO_HOME/.crates.toml` to
   `$CARGO_HOME/liner.toml`.
   * New `--force` option to overwrite the destination file.
   * New `--exact`, `--patch`, and `--compatible` options to customize imported
     version requirements.

## Testing

 * New tests for the new features.
 * Some basic refactoring.

## Documentation

 * Some minor spelling mistakes have been fixed in `README.md`.
 * CI results badge has been added to `README.md`.
 * `CHANGELOG.md`'s format has been adjusted slightly.
 * Documentation for new features.


# [Version 0.1.1 (16/11/2022)](https://crates.io/crates/cargo-liner/0.1.1)
## Fixes

 * The `deny_unknown_fields` serde attribute on the config parser has been
   removed: it ensures forward and backward compatibility.

## Testing

 * New basic tests have been implemented for some modules.
 * GitHub Actions have been set up in order to provide CI.

## Documentation

 * New files: `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.
 * Sexy badges have been added to the `README.md`.
 * The `CHANGELOG.md` has been tweaked a bit to include version dates.


# [Version 0.1.0 (13/11/2022)](https://crates.io/crates/cargo-liner/0.1.0)
## Features

 * Most basic CLI.
 * Deserialization and validation of configuration file.
 * Call to `cargo install` with configured package names and versions.
