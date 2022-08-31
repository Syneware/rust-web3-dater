extern crate chrono;
extern crate tokio;
extern crate web3_dater;

use chrono::DateTime;
use web3_dater::Web3Dater;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let transport = web3::transports::Http::new("https://rpc.ankr.com/eth").unwrap();
        let web3client = web3::Web3::new(transport);

        let mut dater = Web3Dater::new(web3client);

        let search_date = DateTime::parse_from_rfc3339("2022-03-16T18:31:00+00:00").unwrap();
        let block = dater.get_block_by_date(search_date, true).await.unwrap();

        println!("{:?}", block);
    })
}
