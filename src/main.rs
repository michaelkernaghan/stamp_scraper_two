use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
struct Stamp {
    country_name: String,
    stamp_numbers: Vec<String>,
}
async fn fetch_data(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}
fn process_data(html_content: &str) -> Result<Vec<Stamp>, Box<dyn Error>> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("span").unwrap();
    let mut stamps = Vec::new();

    let year_regex =
        Regex::new(r"\s18\d{2}.|\s19[0-3]\d\s.|\s19[0-3]\d-[0-3]\d.|19\d{2}-40.|1940\s.")?;
    let exclude_keywords = vec![
        "jpg",
        "CATALOGUE",
        "Catalogue",
        "BLOCK",
        "Block",
        "CANADA",
        "Booklets",
        "BOOKLETS",
        "SPECIMEN",
        "cover",
        "Souvenir",
        "CINDERELLAS",
        "BANK NOTE",
        "POSTCARDS",
        "P.O.W. Mail",
        "COVERS",
        "JERSEY",
        "Accum",
        "Omnibus complete",
        "BERLIN",
        "LAW STAMPS",
        "BRITISH COLUMBIA",
        "ONTARIO",
        "SAUDI ARABIA",
        "ALBERTA",
        "SASKATCHEWAN",
        "ESSAYS",
        "BANKNOTE",
        "POST  CARDS",
        "Postcard",
        "postcard",
        "Cover",
        "Forgery",
        "forgery",
        "Fake",
        "Law Stamp",
        "EAST GERMANY",
        "GREAT BRITAIN",
        "GDR",
        "Sel'n",
        "sel'n",
        "ACCESSORIES",
        "booklet",
        "OMNIBUS",
        "Range of",
        "study group",
        "ON PAPER",
        // Add more as needed
    ];

    for element in document.select(&selector) {
        let description = element.text().collect::<Vec<_>>().join(" ");

        if year_regex.is_match(&description)
            && !exclude_keywords.iter().any(|kw| description.contains(kw))
        {
            let country_name_regex = Regex::new(r"^[A-Z]+(?: [A-Z]+)?")?;
            let stamp_number_regex = Regex::new(r"#\w+")?;

            let country_name = country_name_regex
                .find(&description)
                .map_or(String::new(), |m| m.as_str().to_string());
            let stamp_numbers = stamp_number_regex
                .find_iter(&description)
                .map(|m| m.as_str().replace("#", ""))
                .collect();

            stamps.push(Stamp {
                country_name,
                stamp_numbers,
            });
        }
    }

    Ok(stamps)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.fvhstamps.com/WeeklyAuctions/FvhWA.htm";
    let html_content = fetch_data(url).await?;
    let stamps = process_data(&html_content)?;

    for stamp in stamps {
        let formatted_output: Vec<String> = stamp
            .stamp_numbers
            .iter()
            .map(|number| format!("{} {}", stamp.country_name, number))
            .collect();
        println!("{}", formatted_output.join(", "));
    }

    Ok(())
}
