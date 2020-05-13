#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate csv;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;

use diesel::prelude::*;
use dotenv::dotenv;
use futures::stream::StreamExt;

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

mod nasdaq;
use nasdaq::{
    chart::ChartRoot, dividends::DividendsRoot, info::InfoRoot, info::NDAQ_QUOTE_HEADER,
    insiders::InsidersRoot, option_chain::OptionChainRoot,
};

mod models;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let conn = establish_connection();

    let all_urls = gen_urls();

    // let option_urls: Vec<String> = all_urls[0].clone();
    // let chart_urls: Vec<String> = all_urls[1].clone();
    let urls: Vec<String> = all_urls[0].clone();
    // let div_urls: Vec<String> = all_urls[3].clone();
    // let insider_urls: Vec<String> = all_urls[0].clone();
    let now = Instant::now();

    // make distinct if endpoint serves a vec<rec> or a rec

    let fetches = futures::stream::iter(urls.into_iter().map(|url| async move {
        if let Ok(res) = reqwest::get(&url.clone()).await {
            if let Ok(root) = res.json::<InfoRoot>().await {
                // let recs: Vec<Vec<String>> = root.to_recs(); //
                let rec: Vec<&str> = root.to_rec().iter().map(|x| x.as_ref()).collect();
                let quote: models::NewQuote = quote_from_vec(&rec).clone();
                // let id = root.get_id();
                // let id = ndaq_url_to_ticker(url.clone());
                // println!("{}", id);
                // let t: String = epoch_str();
                // let filename: String = format!("./data/insiders/{}_{}.csv", id, t);

                // match write_csv(filename, recs, root.gen_header()) {
                //     Ok(_) => println!("{}", id),
                //     _ => println!("CSV FUCKED good"),
                // }
                // println!("{:?}", rec[0]);
                return Some(quote.clone());
            }
            println!("serialized json wrong {}", url.clone());
            return None;
        }
        println!("no good1");
        return None;
    }))
    .buffer_unordered(16)
    .collect::<Vec<Option<models::NewQuote>>>()
    .await;
    // let recs: Vec<Vec<String>> = fetches.into_iter().flatten().collect();
    // let recs: Vec<Vec<&str>> = fetches.iter().flat_map(|x| x.as_ref()).collect();
    let recs: Vec<models::NewQuote> = fetches.into_iter().flatten().collect();
    // let t: String = epoch_str();
    // let filename: String = format!("./data/quotes/{}.csv", t);
    let len: usize = recs.len();
    // write_csv(
    //     filename,
    //     recs,
    //     NDAQ_QUOTE_HEADER
    //         .iter()
    //         .map(|x| x.clone().to_string())
    //         .collect(),
    // )?;
    // println!("{:#?}", fetches);
    let db_quotes: Vec<models::Quote> = recs.iter().map(|x| create_quote(&conn, x)).collect();
    println!("{:?} ", db_quotes);
    println!(
        "{} seconds: {} records",
        now.elapsed().as_secs(),
        len.to_string()
    );

    Ok(())
}

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn write_csv(
    filename: String,
    data: Vec<Vec<String>>,
    header: Vec<String>,
) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(filename.to_string())
        .expect(format!("whtf csv {}", filename).as_ref());
    wtr.write_record(header.clone())?;
    wtr.flush()?;
    for row in data.iter() {
        assert_eq!(header.len(), row.len()); // perf hit?
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn gen_urls() -> Vec<Vec<String>> {
    // let tick_sfxs = vec!["insider-trades"];
    let tick_sfxs = vec!["info"];
    // "option-chain", "chart", "info", "dividends",
    let tickers: Vec<String> = read_tickers("/home/sippycups/sipfin/finox/ref_data/tickers.txt"); // TODO: get from sql table
    let mut urls: Vec<Vec<String>> = vec![];
    for sfx in tick_sfxs.iter() {
        let sfx_urls: Vec<String> = tickers
            .iter()
            .map(|x| {
                format!(
                    "https://api.nasdaq.com/api/quote/{}/{}?assetclass=stocks",
                    // "https://api.nasdaq.com/api/company/{}/{}?limit=99999&type=ALL",
                    x.to_string(),
                    sfx.to_string()
                )
            })
            .collect();
        urls.push(sfx_urls);
    }
    return urls;
}

pub fn epoch_str() -> String {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();
    return t;
}

pub fn ndaq_url_to_ticker(url: String) -> String {
    let v: Vec<&str> = url.split("/").collect(); // divs
    return format!("{}_insider", v[5]);
}
// pub fn lilfetcher(urls: Vec<String>, )

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_quote<'a>(conn: &diesel::pg::PgConnection, q: &'a models::NewQuote) -> models::Quote {
    diesel::insert_into(schema::quotes::table)
        .values(q)
        .get_result(conn)
        .expect("Error saving new post")
}


pub fn quote_from_vec<'a>(rec: &'a Vec<&'a str>) -> models::NewQuote<'a> {
    // rec.
    return models::NewQuote {
        symbol: rec[0],
        company_name: rec[1],
        stock_type: rec[2],
        exchange: rec[3],
        is_nasdaq_listed: rec[4],
        is_nasdaq100: rec[5],
        is_held: rec[6], 
        last_trade_timestamp: rec[7],
        last_sale_price: rec[8],
        net_change: rec[9],
        percentage_change: rec[10],
        is_real_time: rec[11],
        delta_indicator: rec[12],
        };
}