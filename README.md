# Fornjot

[**Blog**](https://www.fornjot.app/blog/) | [**Community**](https://www.fornjot.app/community/) | [**Contribution Guide**](CONTRIBUTING.md)

## About

Fornjot is an **early-stage project** to create a **next-generation, code-first CAD application**. Because [**the world needs another CAD program**](https://github.com/sponsors/hannobraun).

![Screenshot of Fornjot](models/star/star.png)

For an introduction of what the project aims to achieve, [please check out the website](https://www.fornjot.app/).


## Sponsors

Fornjot is supported by [**@webtrax-oz**](https://github.com/webtrax-oz), [**@reivilibre**](https://github.com/reivilibre), [**@lthiery**](https://github.com/lthiery), [**@ahdinosaur**](https://github.com/ahdinosaur), [**@martindederer**](https://github.com/martindederer), [**@bollian**](https://github.com/bollian), [**@rozgo**](https://github.com/rozgo), [**@nullstyle**](https://github.com/nullstyle), [**@tachiniererin**](https://github.com/tachiniererin), [**@sucaba**](https://github.com/sucaba), [**@Kethku**](https://github.com/Kethku), [**@jessebraham**](https://github.com/jessebraham), [**@seanjensengrey**](https://github.com/seanjensengrey), [**@MattOslin**](https://github.com/MattOslin), [**@benwis**](https://github.com/benwis), [**@happysalada**](https://github.com/happysalada), [**@jminer**](https://github.com/jminer), [**@jeevcat**](https://github.com/jeevcat), [**@voxpelli**](https://github.com/voxpelli), [**@U007D**](https://github.com/U007D), [**@guillaumechauvat**](https://github.com/guillaumechauvat), [**@MitchellHansen**](https://github.com/MitchellHansen), [**@mayfieldiv**](https://github.com/mayfieldiv), [**@ksindi**](https://github.com/ksindi), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

**Please consider [supporting me too](https://github.com/sponsors/hannobraun), to help make Fornjot sustainable long-term.**


## Table of Contents

- [**Status**](#status)
- [**Features**](#features)
- [**Usage**](#usage)
- [**Community**](#community)
- [**Get Involved**](#get-involved)
- [**License**](#license)

## Status

Fornjot is **under active development, but still experimental**. Efforts are currently focused on providing a [stable set of basic CAD features](https://github.com/hannobraun/Fornjot/milestone/1).

If you are interested in Fornjot and are considering to use it, you should fully expect to run into limitations pretty much immediately. Unless you are willing to contribute to its development, it would be better to wait for a year or ten, to let it mature. For more information on current limitations and improvements that could be implemented in the near future, [check out the open issues](https://github.com/hannobraun/Fornjot/issues).

To learn about the project's longer-term direction, please refer to the [roadmap](https://www.fornjot.app/roadmap/).


## Features

### Code-first CAD in Rust

Models are defined as Rust code. To ensure fast compile times, they are compiled separately, and loaded into the Fornjot application as a plug-in.

``` rust
use fj::syntax::*;

#[fj::model]
pub fn model(
    #[param(default = 1.0, min = inner * 1.01)] outer: f64,
    #[param(default = 0.5, max = outer * 0.99)] inner: f64,
    #[param(default = 1.0)] height: f64,
) -> fj::Shape {
    let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
    let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));

    let footprint = outer_edge.difference(&inner_edge);
    let spacer = footprint.sweep([0., 0., height]);

    spacer.into()
}
```

This is the code for the [spacer model](/models/spacer).

### Basic modeling features

At this point, Fornjot supports basic 2D shapes (sketches made from lines segments, circles, and limited combinations between them) and sweeping those 2D shapes along a straight path to create a 3D shape.

The short- to mid-term priority is to provide CSG support, more flexible sketches, and more flexible sweeps (along a circle or helix). Long-term, the plan is to keep adding more advanced CAD modeling features, to support even complex models and workflows.

### Supports the major desktop platforms

As of this writing, Fornjot runs on Linux, Windows, and macOS. The project is primarily developed on Linux, so the other platforms might be subject to bugs. If you want to help out, regularly testing on Windows and macOS, and reporting bugs, is a good way to do so.

Short- to mid-term, the plan is to add support for the web platform, so Fornjot can run in browsers. Long-term, the plan is to additionally support the major mobile platforms.

### Export to 3MF & STL

Exporting models to both the [3D Manufacturing Format](https://en.wikipedia.org/wiki/3D_Manufacturing_Format) (3MF), which is used in 3D printing, and STL is supported.


## Usage

### Installation

Since Fornjot uses Rust as the language for defining models, a [Rust toolchain](https://www.rust-lang.org/tools/install) is required to use Fornjot.

To install Fornjot itself, you have the following options:

1. Download a binary from the [latest release](https://github.com/hannobraun/Fornjot/releases).
2. Compile the latest release yourself: `cargo install fj-app`
3. Compile a development version from this repository: `cd path/to/repo; cargo install --path crates/fj-app`

While the Fornjot application is a graphical application that opens a window and displays a 3D view of your model, it can currently only be started from the command-line. The instructions below assume that you have the Fornjot application installed somewhere on your path, under the name `fj-app`.

#### Mac OS
If you download in [latest release](https://github.com/hannobraun/Fornjot/releases) binary, two problems need to be noticed when running on your local computer.
1. The mac will prompt that "fj-app-x86_64-apple-arwin" cannot be opened because it is from an unknown developer.

The solution is Open the Mac system preferences. In the preferences interface, click to open **security and privacy**. In the security and privacy interface, click **General**. In the call settings panel, click the **Still want to open** button. In the pop-up window, click the **Open** button.

2. Failed to open the document "fj-app". Text encoding Unicode (UTF-8) is not applicable.

The solution is enter the following command in the terminal, (we recommend changing the file name to fj-app)
```shell
chmod a+x fj-app
```

#### Via Nix

There's a Nix [flake](https://nixos.wiki/wiki/Flakes) in the subdirectory `./nix` which contains a devshell environment (via `nix develop` or `nix-shell`) and the package `fj-app`.
It can be run/tested with a [flake enabled](https://nixos.wiki/wiki/Flakes#Enable_flakes) nix via `nix run github:hannobraun/Fornjot?dir=nix` or with legacy nix in the directory `./nix`, `nix-build`.

### Defining models

Models are Rust libraries that depend on the [`fj`](https://crates.io/crates/fj) library, which they use to define the geometry. Furthermore, they need to be built as a dynamic library. Just use the examples in the [`models/`](models) directory as a template to define your own.

### Viewing models

To view a model, run:

``` sh
fj-app my-model
```

This will usually compile and load the model in the `my-model/` directory. If there is a configuration file (`fj.toml`) available, it might define a default path to load models from that is different from the current working directory. This is the case [in the Fornjot repository](fj.toml).

Rotate the model by pressing the left mouse button while moving the mouse. Move the model by pressing the right mouse button while moving the mouse. Zoom with the mouse wheel.

Toggle model rendering by pressing `1`. Toggle mesh rendering by pressing `2`. Toggle rendering of debug data by pressing `3`.

### Exporting models

To export a model to a file, run:

``` sh
fj-app my-model --export my-model.3mf
```

The file type is chosen based on the file extension. Both 3MF and STL are supported.

### Model parameters

Models can define parameters that can be overridden. This can be done using the `--parameters` argument:

``` sh
fj-app my-model --parameters "width=3.0,height=5.0"
```


## Community

If you are interested in Fornjot, please consider joining the community. We'd love to have you!

Please check out [the community page on the website](https://www.fornjot.app/community/) for information on where to find us!


## Get Involved

If you are interested in helping out, just fork one of the GitHub repositories and submit a pull request:

- [Main Fornjot repository](https://github.com/hannobraun/Fornjot)
- [Website repository](https://github.com/hannobraun/www.fornjot.app)

If you don't know what to work on, check out the [`good first issues`](https://github.com/hannobraun/Fornjot/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). To get an overview over current priorities, take a look at the [open milestones](https://github.com/hannobraun/Fornjot/milestones).

If you need some more guidance, check out the [contribution guide](CONTRIBUTING.md), [or just ask](https://www.fornjot.app/community/)!


## License

This project is open source, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with it, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[`fj`]: https://crates.io/crates/fj
[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
