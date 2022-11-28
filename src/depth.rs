mod transform;

use csv::Writer;
use quotation::DepthManager;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

use transform::transform_to_local;

fn main() {
    println!("Hello");

    tracing_subscriber::fmt::init();

    Runtime::new().unwrap().block_on(async {
        let exchange = "binance";
        let pc_symbol = "BTC_USDT_221230_SWAP";
        let pu_symbol = "BTC_USDT_SWAP";
        let spot_symbol = "BTC_USDT";
        let _ = vec![pc_symbol, pu_symbol, spot_symbol];

        let symbol = spot_symbol;
        println!("using symbol {}", symbol);

        let manager1 = DepthManager::with_snapshot(exchange, symbol, 1000);
        let mut receiver = manager1.subscribe();
        println!("using manager1 config {:?}", manager1.config);

        tokio::spawn(async move {
            let mut wtr = Writer::from_path("depth.cache").unwrap();

            sleep(Duration::from_secs(2)).await;
            while let Some(message) = receiver.recv().await {
                let message = transform_to_local(&message);
                wtr.serialize(message.csv()).unwrap();
                wtr.flush().unwrap();
            }
        });


        sleep(Duration::from_secs(1800)).await;
    });

    println!("Depth Done");
}
