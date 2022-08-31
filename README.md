# Web3 Dater

A simple library to get ethereum block by date using `web3`

## Installation

Add the `web3_dater` to your project dependencies:

```shell
cargo add web3_dater
```

or

```toml
[dependencies]
web3_dater = "0.1.1"
```

## Example

```rust
use web3_dater::Web3Dater;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = web3::transports::Http::new("https://rpc.ankr.com/eth").unwrap();
    let web3client = web3::Web3::new(transport);

    // Create a new instance of Web3Dater
    let mut dater = Web3Dater::new(web3client);

    let search_date = DateTime::parse_from_rfc3339("2022-08-31T17:31:00+00:00").unwrap();

    // Get the block by date
    let block = dater.get_block_by_date(search_date, true).await.unwrap();

    println!("{:?}", block);

    Ok(())
}
```

## Docs

[Documentation](https://docs.rs/web3_dater)

## Support

For support, email support@syneware.com

## License

[MIT](LICENSE)
