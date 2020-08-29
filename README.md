<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/rust-lang-ve/design/main/assets/logo_above.png" height="120" width="120" />
  </div>
  <h1 align="center">libcne-ve</h1>
  <h4 align="center">Fetches data from a public endpoint in the CNE website and deserializes its contents.</h4>
</div>

## Installation

To install the latest release, add the crate as follows to your dependencies list in your `Cargo.toml`.

```toml
[dependencies]
libcne-ve = "0.1.0"
```

If you want to use a specific version, you must add the crate to your `Cargo.toml` as follows:

```toml
[dependencies]
libcne = { git = "https://github.com/rust-lang-ve/libcne-ve.git", tag = "v0.1.0" }
```

## Motivation

> The first motivation for this repository is to learn Rust. We have a very good feeling regarding the future of Rust so we decided to learn about the language in community.

`libcne-ve` is a hobbist project to gather **public data available** in the CNE website.

Basically this crate, makes a request to the endpoint available in the site
used to fetch date about where a given ID (CID) belongs as voting center, scraps the
HTML response into an `Elector` `struct` and returns it.

## Example

The following sample is available in the `libcne-ve/example` directory:

```rust
use libcne_ve::request::find;
use libcne_ve::cne::{Citizenship, Elector};

#[tokio::main]
async fn main() {
  let elector_id: String = String::from("123123123");
  let elector: Elector = find(Citizenship::V, elector_id).await.unwrap();

  println!("{:?}", elector);
}
```

## Releasing

To release a new version you must tag with git and push to the `main` branch.

```bash
git tag -a v0.1.0 -m "First Release"
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome! Feel free to open a pull request or an issue.

## License

Licensed under the GNU General Public License.
