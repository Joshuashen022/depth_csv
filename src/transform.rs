use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use quotation::Depth;
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookStore{
    pub last_update_id: i64,
    pub send_time: i64,
    pub receive_time: i64,
    pub bids: Vec<(f64,f64)>, // price, amount
    pub asks: Vec<(f64,f64)>,
}

pub fn transform_to_local(quote: &Depth) -> OrderBookStore{
    let bids :Vec<_> = quote.bids.iter().map(|x|(x.price,x.amount)).collect();
    let asks :Vec<_> = quote.asks.iter().map(|x|(x.price,x.amount)).collect();
    OrderBookStore{
        last_update_id: quote.id,
        send_time: quote.ts,
        receive_time: quote.lts,
        bids,
        asks,
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookStoreCSV<'a>{
    pub last_update_id: i64,
    pub send_time: i64,
    pub receive_time: i64,
    pub bids: Cow<'a, str>,
    pub asks: Cow<'a, str>,
}

impl OrderBookStore{

    fn ordered(&self) -> bool{
        let mut price_last = 0.0;
        let mut res1 = true;
        for &(price, _) in &self.asks{
            if !(price_last <= price){
                res1 = false;
                break;
            }
            price_last = price;
        }

        let mut price_last = self.bids.clone()[0].0;
        let mut res2 = true;
        for &(price, _) in &self.bids{
            if !(price_last >= price){
                res2 = false;
                break;
            }
            price_last = price;
        }

        res1 && res2
    }

    fn asks_20(&self) -> Vec<(f64,f64)>{
        let ask_len = self.asks.len();
        if ask_len > 20 {
            let mut asks = self.asks.clone();
            let _ = asks.split_off(20);
            asks
        } else{
            self.asks.clone()
        }
    }

    fn bids_20(&self) -> Vec<(f64,f64)>{
        if self.bids.len() > 20 {
            println!("self.bids.len");
            let mut bids = self.bids.clone();
            let _ = bids.split_off(20);
            let res = bids;
            
            res
        } else {
            self.bids.clone()
        }
    }

    pub fn csv(&self) -> OrderBookStoreCSV{
        println!(" asks first {:?}, last {:?}", self.asks.first(), self.asks.last());
        // println!(" bid first {:?}, last {:?}", self.bids.first(), self.bids.last());

        assert!(self.ordered());
        let asks = self.asks_20();
        println!(" res first {:?}, last {:?}", asks.first(), asks.last());
        assert_eq!(asks.len(), 20);

        let mut asks_string = String::new();
        for (price, amount) in asks{
            asks_string += &format!("{},{},", price, amount);
        }
        asks_string.pop();

        let bids = self.bids_20();
        // println!(" bids first {:?}, last {:?}", bids.first(), self.bids.last());

        assert_eq!(bids.len(), 20);
        let mut bids_string = String::new();
        for (price, amount) in bids{
            bids_string += &format!("{},{},", price, amount);
        }
        bids_string.pop();

        OrderBookStoreCSV{
            last_update_id: self.last_update_id,
            send_time: self.send_time,
            receive_time: self.receive_time,
            asks: Cow::Owned(asks_string),
            bids: Cow::Owned(bids_string),
        }
    }
}
