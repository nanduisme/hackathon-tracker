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
    let mut links = vec![];

    for i in elements {
        links.push(i.attr("href").await.unwrap().unwrap())
    }

    println!(
        "Name,Place,Registration Starts,Registration End,Hackathon Starts,Hackathon Ends,Link"
    );

    for link in links {
        c.goto(&link)
            .await
            .expect(&format!("Could not access {}", link));
        thread::sleep(Duration::from_secs(5));

        let title = c.find(Locator::Css(".dxzFsX")).await?.text().await?;
        let location: String = match c.find(Locator::Css(".jAZTsD + .jAZTsD > p + p")).await {
            Ok(e) => e.text().await?,
            Err(_) => "None Given".to_string(),
        };

        c.goto(&format!("{}schedule", link))
            .await
            .expect("Schedule could not be reached");
        thread::sleep(Duration::from_secs(5));

        let date_elements = c.find_all(Locator::Css("[name=\"calendar\"] + p")).await?;
        let datetime_elements = date_elements
            .iter()
            .zip(c.find_all(Locator::Css("[name=\"clock\"] + p")).await?);

        let mut datetimes = Vec::new();

        for (date, time) in datetime_elements {
            datetimes.push((date.text().await?, time.text().await?));
        }

        let mut rec = format!("{},\"{}\",", title, location);

        if datetimes.is_empty() {
            rec.push_str("None Given,None Given,None Given,None Given,");
        } else {
            for j in 0..4 {
                rec.push_str(&format!("{} {},", datetimes[j].0, datetimes[j].1))
            }
        }

        rec.push_str(&link);

        println!("{rec}");
    }

    c.close().await
}
