extern crate chrono;
extern crate csv;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use futures::stream::StreamExt;
use headers::*;
use std;
use std::time::{SystemTime, UNIX_EPOCH};

mod headers;
mod yf;
mod utils;
mod keys;
mod sa;
mod news;
mod gs;
fn newsmain() -> Result<(), Box<dyn std::error::Error>> {
    // utils::nytarchive();

    utils::nytfeed();
    utils::gsnews();
    //utils::jpxnews();
    utils::reuters();
    utils::wsj_videos();
    utils::sa();
    //bloomberg::news();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let tickers = utils::read_tickers("../ref_data/tickers.txt");
    newsmain()?;
    //let symbs: Vec<&str> = CURRENCY_SYMBOLS_YF.to_vec(); //.into_iter().cloned().collect();
    //let len = symbs.len();
    //let mut urls: Vec<String> = vec![];
    //for (i, x) in symbs.iter().enumerate() {
    //    for y in symbs[i..len].iter() {
    //        if x == y {
    //            continue;
    //        };
    //        urls.push(
    //        format!(
    //            //"https://query2.finance.yahoo.com/v8/finance/chart/{}=F?interval=1d&period1=0&period2=1589932800",
    //            "https://query1.finance.yahoo.com/v8/finance/chart/{}{}=X?interval=1d&period1=0&period2=1589932800",
    //            x.to_string(),
    //            y.to_string()
    //        )
    //    );
    //    }
    //}
    //let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
    //    if let Ok(res) = reqwest::get(&url.clone()).await {
    //        if let Ok(root) = res.json::<yf::Root>().await {
    //            let recs: Vec<Vec<String>> = yf::Root::to_records(&root);
    //            println!("{}: {}", url, recs.len());
    //            return Some(recs);
    //        }
    //        println!("no json {}", url);
    //        return None;
    //    }
    //    println!("no2");
    //    return None;
    //}))
    //.buffer_unordered(16)
    //.collect::<Vec<Option<Vec<Vec<String>>>>>();
    //let vecs = fetches.await;
    //let recs = vecs
    //    .into_iter()
    //    .flatten()
    //    .collect::<Vec<Vec<Vec<String>>>>()
    //    .into_iter()
    //    .flatten()
    //    .collect::<Vec<Vec<String>>>();
    ////println!("{:#?}", recs);
    //let cur_time = SystemTime::now()
    //    .duration_since(UNIX_EPOCH)
    //    .unwrap()
    //    .as_secs();
    //let file_name = format!("../data/yf_currencies_{}.csv", cur_time.to_string());
    //let mut wtr = csv::Writer::from_path(file_name)?;
    //wtr.write_record(vec!["symbol", "t", "o", "h", "l", "c", "v"])?;

    //for rec in recs.iter() {
    //    wtr.write_record(rec)?;
    //}
    //wtr.flush()?;

    Ok(())
}
