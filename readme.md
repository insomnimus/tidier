[![Build Status](https://github.com/insomnimus/tidier/actions/workflows/main.yml/badge.svg)](https://github.com/insomnimus/tidier/actions)
 [![crates.io](https://img.shields.io/crates/v/tidier.svg)](https://crates.io/crates/tidier)
 [![docs.rs](https://docs.rs/tidier/badge.svg)](https://docs.rs/tidier/)

# Tidier
This crate provides a safe abstraction over the [Tidy](https://github.com/htacg/tidy-html5) C library.

## Features
- Currently, it only supports formatting of HTML, XHTML and XML documents.

## Examples
Note: Check out the basic CLI example in the [examples directory](https://github.com/insomnimus/tidier/tree/main/examples).

```rust
use tidier::{Doc, FormatOptions, Indent};

let html = "<html>
<head><title>Tidy Usage Example</title></head>
<body><p>Usage example</p></body>
</html>";

let opts = FormatOptions {
	wrap: 60,
	strip_comments: true,
	indent: Indent {
		tabs: true,
		..Indent::DEFAULT
	},
	..FormatOptions::DEFAULT
};

// Alternatively
let opts = FormatOptions::new()
	.tabs(true)
	.strip_comments(true)
	.wrap(60);

let doc = Doc::new(html, false)?;
let s1 = doc.format(&opts)?;

// Or for short:
let s2 = tidier::format(html, false, &opts)?;

assert_eq!(s1, s2);

# Ok::<_, tidier::Error>(())
```

## Build Requirements
This crate uses [tidy-sys](https://crates.io/crates/tidy-sys), which generates bindings on the fly using [bindgen](https://github.com/rust-lang/rust-bindgen), then compiles the tidy C library from source.
Therefore you need;
- clang: For parsing C headers to generate Rust bindings
- A C compiler: To compile the Tidy C library
- CMake: To configure and orchestrate compilation of the Tidy C library (it uses CMake as the build system)

You don't need to install libtidy on your system; `tidy-sys` vendors the source code, builds and links to it statically.
