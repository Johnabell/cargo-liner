<div align="center">
    <img
        src="https://cdn.dribbble.com/users/959848/screenshots/4159478/media/e21863defd968fff7f27690f36a2ccef.gif"
        alt="Cargo ship on its way"
        width="60%"
    />
    <br/>
    <h1>Cargo Liner</h1>
    <h3>Configuration-oriented wrapper around <code>cargo install</code></h3>
    <a href="https://github.com/PaulDance/cargo-liner/releases/latest">
        <img
            alt="GitHub release (latest by date including pre-releases)"
            src="https://img.shields.io/github/v/release/PaulDance/cargo-liner?include_prereleases&style=for-the-badge"
        />
    </a>
    <a href="https://github.com/PaulDance/cargo-liner/releases/latest">
        <img
            alt="GitHub (Pre-)Release Date"
            src="https://img.shields.io/github/release-date-pre/PaulDance/cargo-liner?style=for-the-badge&display_date=published_at"
        />
    </a>
    <a href="https://crates.io/crates/cargo-liner">
        <img
            alt="Crates.io version"
            src="https://img.shields.io/crates/v/cargo-liner?style=for-the-badge"
        />
    </a>
    <br/>
    <a href="https://github.com/PaulDance/cargo-liner/actions">
        <img
            alt="GitHub Workflow Status"
            src="https://img.shields.io/github/actions/workflow/status/PaulDance/cargo-liner/ci.yml?branch=master&style=for-the-badge"
        />
    </a>
    <a href="https://crates.io/crates/cargo-liner">
        <img
            alt="Crates.io downloads"
            src="https://img.shields.io/crates/d/cargo-liner?style=for-the-badge"
        />
    </a>
    <br/>
    <a href="https://github.com/PaulDance/cargo-liner/pulse">
        <img
            alt="Maintainance status: active"
            src="https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg?style=for-the-badge"
        />
    </a>
    <a href="https://github.com/PaulDance/cargo-liner/blob/master/LICENSE.txt">
        <img
            alt="License"
            src="https://img.shields.io/github/license/PaulDance/cargo-liner?style=for-the-badge"
        />
    </a>
</div>


[car·go·lin·er](https://www.dictionary.com/browse/cargo-liner), _noun_:

> 1. a cargo ship that sails regularly between designated ports according to a
>    published schedule.


## Summary

Cargo Liner is a tool to help one who has packages currently installed or to be
installed through the official `cargo install` command to install and maintain
them up-to-date by editing a small and stable configuration file located at
`$CARGO_HOME/liner.toml`.

Goals:
 * Simple and intuitive API.
 * Stable configuration file: avoid editing it automatically.
 * Actually use `cargo install` and not much else.

Non-goals:
 * Super-duper stability guarantees.
 * Re-implementing half of Cargo for small functionalities.
 * Being """pretty""" _above all else_.
 * Handling the synchronization of the configuration file between various hosts.


## Rationale

`cargo install` works very well to download, compile and install a binary
package. However, it does not offer means to update currently installed
programs without having to specify them manually one by one on the CLI. That
becomes quickly bothersome when having to maintain several packages up-to-date,
especially if it needs to be done on multiple workstations.

Some projects, such as [cargo-update] or [cargo-updater], exist in order to
solve this issue. Their strategy is to exploit the `$CARGO_HOME/.crates.toml`
and `$CARGO_HOME/.crates2.json` files that Cargo generates and maintains in
order to track which packages are installed, their exact version, where they
were downloaded from and which programs they have installed. This strategy is
quite effective, so if you are looking for exactly that, then check them out.

However, some problems are still not solved like this: when configuring a new
workstation, there is still the need to specify each package manually at least
once; when adding a new package on one already-configured workstation, there is
still the need to install it manually on all others. These tools lack *sharing*
and *synchronization*.

The current project therefore inspires itself from tools such as [zplug] for
Zsh and [vim-plug] for Vim by taking orders from a central configuration file.
The tool then simply runs `cargo install` for all packages listed in that file.
That enables one to install and maintain all packages up-to-date, but also to
keep all of one's workstations synchronized by sharing the file between them in
some way, using Git for example.

[cargo-update]: https://github.com/nabijaczleweli/cargo-update
[cargo-updater]: https://github.com/pombadev/cargo-updater
[zplug]: https://github.com/zplug/zplug
[vim-plug]: https://github.com/junegunn/vim-plug


## Installation

 * Run: `cargo install cargo-liner`.
 * Create the configuration file to be located at: `$CARGO_HOME/liner.toml`.
   * See the reference documentation about [Cargo Home] if you have trouble
     locating the directory.
 * Populate the file with packages you wish to be installed, for example:
 
   ```toml
   [packages]
   cargo-expand = "*"
   cargo-tarpaulin = "~0.22"
   nu = "=0.71.0"
   sqlx-cli = { version = "0.6.2", default-features = false, features = ["native-tls", "postgres"] }
   ripgrep = { version = "13.0.0", all-features = true }
   ```

   or use `cargo liner import` to do it automatically for you, see below for
   more detailed explanations.

[Cargo Home]: https://doc.rust-lang.org/cargo/guide/cargo-home.html


## Usage
### CLI

Some basic commands are now available:

```sh
❯ cargo help liner
Cargo subcommand to install and update binary packages listed in configuration.

Usage: cargo liner [OPTIONS] [COMMAND]

Commands:
  ship
          The default command if omitted: install and update configured packages
  import
          Import the `$CARGO_HOME/.crates.toml` Cargo-edited save file as a new
          Liner configuration file
  help
          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so each time.

          When omitted, INFO and above messages of only this crate are logged.
          When used once, DEBUG and above messages of only this crate are logged.
          When used twice, DEBUG and above messages of all crates are logged.
          When used three times or more, TRACE and above messages of all crates
          are logged.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each time.

          When omitted, INFO and above messages of only this crate are logged.
          When used once, WARN and above messages of only this crate are logged.
          When used twice, ERROR messages of all crates are logged.
          When used three times or more, no message will be logged.

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```


#### Default command

When the subcommand is omitted, it will use the `ship` subcommand with default
options.

Simply run `cargo liner` in order to:
 * Read packages from the configuration file.
 * Run `cargo install` for each of them.
 * Self-update.

Example output if `bat` and `cargo-expand` are required:

```sh
❯ cargo liner
 INFO  cargo_liner::cargo > Updating `bat`...
    Updating crates.io index
     Ignored package `bat v0.23.0` is already installed, use --force to override
 INFO  cargo_liner::cargo > Installing `cargo-expand`...
    Updating crates.io index
  Downloaded cargo-expand v1.0.54
  Downloaded 1 crate (26.4 KB) in 0.44s
  Installing cargo-expand v1.0.54
    Updating crates.io index
  Downloaded anstyle-parse v0.2.1
  [...]
  Downloaded clap_builder v4.3.5
  Downloaded 5 crates (311.7 KB) in 0.45s
   Compiling proc-macro2 v1.0.60
   [...]
   Compiling cargo-expand v1.0.54
   [...]
   Compiling cargo-subcommand-metadata v0.1.0
    Finished release [optimized] target(s) in 52.97s
   Replacing /home/pauldance/.cargo/bin/cargo-expand
    Replaced package `cargo-expand v1.0.53` with `cargo-expand v1.0.54` (executable `cargo-expand`)
 INFO  cargo_liner::cargo > Updating `cargo-liner`...
    Updating crates.io index
     Ignored package `cargo-liner v0.3.0` is already installed, use --force to override
 INFO  cargo_liner        > Done.
```


#### `ship` subcommand

The main command: do the installing and updating of packages.

```sh
❯ cargo liner help ship
The default command if omitted: install and update configured packages.

Self-updating is enabled by default.

Usage: cargo liner ship [OPTIONS]

Options:
  -n, --no-self
          Disable self-updating.

          Cannot be used in conjunction with `--only-self`. Default: `false`,
          i.e. self-update.

  -s, --only-self
          Only self-update and do not install or update any other package.

          Cannot be used in conjunction with `--no-self`. Default: `false`,
          i.e. install or update other packages as well.

  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so each time.

          When omitted, INFO and above messages of only this crate are logged.
          When used once, DEBUG and above messages of only this crate are logged.
          When used twice, DEBUG and above messages of all crates are logged.
          When used three times or more, TRACE and above messages of all crates
          are logged.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each time.

          When omitted, INFO and above messages of only this crate are logged.
          When used once, WARN and above messages of only this crate are logged.
          When used twice, ERROR messages of all crates are logged.
          When used three times or more, no message will be logged.

  -h, --help
          Print help information (use `-h` for a summary)
```

Simply run `cargo liner ship` in order to:
 * Read packages from the configuration file.
 * Run `cargo install` for each of them, respecting the version requirements.
 * Self-update only if `--no-self` is not given.


#### `import` subcommand

This command is meant to be used upon installing the tool and using it for the
first time: it populates the configuration file with currently-installed
packages.

```sh
❯ cargo liner help import
Import the `$CARGO_HOME/.crates.toml` Cargo-edited save file as a new Liner
configuration file.

Star versions are used by default. The version transformation options are
mutually exclusive.

Usage: cargo liner import [OPTIONS]

Options:
  -e, --exact
          Import package versions as "exact versions", i.e. prepended with an
          equal operator.

          Cannot be used in conjunction with either `--compatible` or `--patch`.
          Default: `false`, i.e. use a star requirement.

  -c, --compatible
          Import package versions as "compatible versions", i.e. prepended with
          a caret operator.

          Cannot be used in conjunction with either `--exact` or `--patch`.
          Default: `false`, i.e. use a star requirement.

  -p, --patch
          Import package versions as "patch versions", i.e. prepended with a
          tilde operator.

          Cannot be used in conjunction with either `--exact` or `--compatible`.
          Default: `false`, i.e. use a star requirement.

  -f, --force
          Overwrite the current configuration file if it already exists.

          Default: `false`, i.e. return an error in case the file already exists.

  -k, --keep-self
          Also import this `cargo-liner` package into the configuration, for
          example in order to specify a certain version requirement later on.

          Default: `false`, i.e. exclude the current package from the list of
          packages to install or update in the resulting configuration file.
          Note however that the `ship` command will still self-update by
          default.

  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so each time.

          When omitted, INFO and above messages of only this crate are logged.
          When used once, DEBUG and above messages of only this crate are logged.
          When used twice, DEBUG and above messages of all crates are logged.
          When used three times or more, TRACE and above messages of all crates
          are logged.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each time.

          When omitted, INFO and above messages of only this crate are logged.
          When used once, WARN and above messages of only this crate are logged.
          When used twice, ERROR messages of all crates are logged.
          When used three times or more, no message will be logged.

  -h, --help
          Print help information (use `-h` for a summary)
```

For example, if you had previously installed:
 * `bat@0.22.1`
 * `cargo-make@0.36.3`
 * `cargo-outdated@0.11.1`

Then running `cargo liner import` will result in the following configuration
file, if it does not already exist:

```toml
[packages]
bat = "*"
cargo-make = "*"
cargo-outdated = "*"
```

The command will by default import them with star version requirements. The
`--exact`, `--compatible` and `--patch` options are provided in order to
customize how the currently-installed versions are imported into version
requirements: `--exact` will prepend them with `=`, `--compatible` with `^`,
and `--patch` with `~`.

For example, using the previous three packages already installed, running
`cargo liner import --patch` would give:

```toml
[packages]
bat = "~0.22.1"
cargo-make = "~0.36.3"
cargo-outdated = "~0.11.1"
```

The file can of course be edited manually afterwards, as intended.


### Configuration

The file must be located at `$CARGO_HOME/liner.toml` and contain a
properly-formed TOML document respecting the following format:

```toml
[packages]
package-name-1 = "version-req-1"
package-name-2 = "version-req-2"
package-name-3 = { version = "version-req-3", all-features = boolean, default-features = boolean, features = [ "feature-1", "feature-2" ] }
#...
```

where:
 * `package-name-*` must be a valid [package name], i.e. match
   `[a-zA-Z][a-zA-Z0-9_-]*` or something like that.
 * `version-req-*` must be a valid [SemVer] requirement, [Cargo style]. In
   particular, the catch-all wildcard `*` can be used to require the latest
   version available.
 * `feature-*` must be the name of a [cargo feature] defined by the crate being installed.
 * `boolean` is a [toml boolean], either `true` or `false`.
 
[package name]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field
[SemVer]: https://semver.org/
[Cargo style]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
[cargo feature]: https://doc.rust-lang.org/cargo/reference/features.html
[toml boolean]: https://toml.io/en/v1.0.0#boolean


## Contributing

See the [contributing guidelines]. Please also take note of the [code of
conduct].

[contributing guidelines]: ./CONTRIBUTING.md
[code of conduct]: ./CODE_OF_CONDUCT.md


## Copyright notice

[Sergej Tucakov] for the animation used as this project's logo.

[Sergej Tucakov]: https://dribbble.com/sergejtucakov
