# cdylib-plugin.rs

## Introduction

Plugin-style shared libraries are shared libraries designed for a
specific host program. The host program calls into the plugin, and the
plugin calls back into APIs in the host program. Importantly, this
means that the plugin references symbols defined only in the host
program; and these symbols are not defined at the time the plugin is
built.

For example, a PostgreSQL extension (which uses a plugin-style shared
library) may use
[SPI](https://www.postgresql.org/docs/current/spi.html) to execute
queries in the current transaction in a running server. Symbols such
as ``SPI_connect`` are defined in PostgreSQL and referenced by the
plugin. Such symbols are resolved when the plugin is *loaded*, but
cannot be resolved at the time the plugin is *built*.

## Problem

In rust, there are two problems with plugin-style cdylib crates:

1. [#62874](https://github.com/rust-lang/rust/issues/62874) On some
   platforms, linking fails at build time due to the undefined
   symbols, unless special arguments are passed to the linker.
1. [#8193](https://github.com/rust-lang/cargo/issues/8193) There is no
   good way to find the path of the library created.
   1. Makes integration testing difficult.
   1. Makes installation difficult.

## Workarounds

Hopefully these problems are solved properly in the future. Until
then, ``cdylib-plugin.rs`` offers workarounds to these problems.

Add a normal dependency and a build dependency to your crate:

```toml
[dependencies]
# ...
cdylib-plugin = "0.1"

[build-dependencies]
# ...
cdylib-plugin = "0.1"

```

Add a ``build.rs`` in your crate, such as:

```rust
extern crate cdylib_plugin;

fn main() {
	// ...
	cdylib_plugin::buildflags();
}
```

In your integration tests or installation code, find the library path
with ``cdylib_plugin::cdylib_path()``.
