use fantoccini::{ClientBuilder, Locator};
use std::{thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444/")
        .await
        .expect("Oops?");

    c.goto("https://devfolio.co/hackathons/open/")
        .await
        .expect("Unable to access site");
    thread::sleep(Duration::from_secs(10));

    for _ in 0..=10 {
        c.execute("window.scrollBy(0, window.innerHeight);", vec![])
            .await
            .expect("JS failed");
        thread::sleep(Duration::from_secs(3));
    }

    c.wait().for_element(Locator::Css(".lkflLS")).await?;
    let elements = c.find_all(Locator::Css(".lkflLS")).await?;

    for i in elements {
        let link = i.attr("href").await.unwrap();
        let reponse = reqwest::blocking::get(link);
    }

    c.close().await
}
