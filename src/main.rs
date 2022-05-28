use reqwest;
use tokio;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://1000mostcommonwords.com/1000-most-common-spanish-words/")
        .await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"table.cg-table-four ~ table > tbody > tr"#).unwrap();
    for row in document.select(&selector) {
        println!("{}", row);
    }

    // print!("body = {:?}", body);
    Ok(())
}
