use noria::ControllerHandle;

use finox::nasdaq::realtime::RealtimeRoot;
use std::{error::Error, time::Duration};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = ControllerHandle::from_zk("127.0.0.1:2181/isit6")
        .await
        .unwrap();

    db.install_recipe(
        "
    CREATE TABLE Rt(sid varchar(16), t varchar(32), x varchar(16), v varchar(16), PRIMARY KEY(sid));
    CREATE TABLE Quote(sid varchar(16), qid varchar(32));",
    )
    .await
    .unwrap();

    let mut quotes = db.table("Rt").await.unwrap();
    let mut count = db.table("Quote").await.unwrap();

    let (tickers, _) = finox::gen_secs("stocks");
    let urls = tickers[1..5]
        .iter()
        .map(|x| x.to_nasdaq_rt_url())
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect();

    //println!("{:#?}", urls);
    let recs = finox::fetch::<RealtimeRoot>(urls).await;
    //println!("recs: {:#?}", recs);

    for (i, rs) in recs.iter().enumerate() {
        for (j, r) in rs.iter().enumerate() {
            quotes
                .insert(vec![
                    r[0].clone().into(),
                    r[1].clone().into(),
                    r[2].clone().into(),
                    r[3].clone().into(),
                ])
                .await
                .unwrap();
            count
                .insert(vec![r[0].clone().into(), format!("{}{}", i, j).into()])
                .await
                .unwrap();
        }
    }
    println!("Finished writing! Let's wait for things to propagate...");
    tokio::time::delay_for(Duration::from_millis(2000)).await;

    db.extend_recipe(
        "
    QuoteCount: \
        SELECT Quote.sid, COUNT(qid) as counts \
        FROM Quote GROUP BY Quote.sid;
    QUERY Quotes: \
        SELECT Rt.sid, t, x, v, QuoteCount.counts AS counts \
        FROM Rt LEFT JOIN QuoteCount ON (Rt.sid = QuoteCount.sid) \
        WHERE Rt.sid = ?;
    ",
    )
    .await
    .unwrap();
    let mut awvc = db.view("Quotes").await.unwrap();
    let article = awvc.lookup(&["aal".into()], true).await.unwrap();
    println!("{:#?} ", article);
    Ok(())
}
