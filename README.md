<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://avatars3.githubusercontent.com/u/68873317?s=120&v=4" height="120" width="120" />
  </div>
  <h1 align="center">libcne</h1>
  <span align="center">Fetch public elector data from CNE website</span>
</div>

## Installation

To install the latest release, add the crate as follows to your dependencies list in your `Cargo.toml`.

```toml
[dependencies]
libcne = { git = "https://github.com/rust-lang-ve/libcne.git", branch = "master" }
```

If you want to use a specific version, you must add the crate to your `Cargo.toml` as follows:

```toml
[dependencies]
libcne = { git = "https://github.com/rust-lang-ve/libcne.git", tag = "v0.1.0" }
```

## Motivation

> The first motivation for this repository is to learn Rust. We have a very good feeling regarding the future of Rust so we decided to learn about the language in community.

`libcne` is a hobbist project to gather **public data available** in the CNE website.
Basically this crate, makes a request to the endpoint available in the site
used to fetch date about where a given ID (CID) belongs for voting.

## Example

The following sample is available in the `libcne/example` directory:

```rust
use libcne::request::find;
use libcne::cne::{Citizenship, Elector};

#[tokio::main]
async fn main() {
  let elector_id: String = String::from("123123123");
  let elector: Elector = libcne::request::find(Citizenship::V, elector_id).await.unwrap();

  println!("{:?}", elector);
}
```

## Contributing

Every contribution to this project is welcome! Feel free to open a pull request or an issue.

## License

Licensed under the MIT License.
