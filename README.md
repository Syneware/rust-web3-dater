# Web3 Dater

A simple library to get ethereum block by date using web3


## Usage

[example](examples/example.rs)

```rust
let transport = web3::transports::Http::new("https://rpc.ankr.com/eth").unwrap();
let web3client = web3::Web3::new(transport);

let mut dater = Web3Dater::new(web3client.clone());

let search_date = DateTime::parse_from_rfc3339("2022-03-16T00:31:00+00:00").unwrap();
let block = dater.get_block_by_date(search_date, true).await.unwrap();
```