<div align="left">
  <img src="https://avatars3.githubusercontent.com/u/68873317?s=100&v=4" height="100" width="100" />
  <div style="display: inline-block; margin-left: 1rem;">
    <h1>libcne</h1>
    <span>Fetch public elector data from CNE website</span>
  </div>
</div>

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
