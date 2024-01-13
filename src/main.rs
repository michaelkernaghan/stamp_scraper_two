use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let response = fetch_data("http://www.fvhstamps.com/WeeklyAuctions/FvhWA.htm")?;
    process_data(&response)?;
    Ok(())
}

fn fetch_data(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}

fn process_data(html_content: &str) -> Result<(), Box<dyn Error>> {
    let document = Html::parse_document(html_content);
    let title_selector = Selector::parse("span")?;
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    let exclusion_keywords = get_exclusion_keywords();
    let re = Regex::new(r">(.*?)<")?;
    
    for title in titles {
        if should_include(&title, &exclusion_keywords) {
            let cleaned_title = clean_text(&title);
            extract_and_print_dates(&cleaned_title, &re)?;
        }
    }

    Ok(())
}

fn get_exclusion_keywords() -> Vec<&'static str> {
    vec![
        "jpg", "CATALOGUE", "Catalogue", "BLOCK", "Block", "CANADA", 
        // ... add all other keywords here
    ]
}

fn should_include(title: &str, keywords: &[&str]) -> bool {
    !keywords.iter().any(|&keyword| title.contains(keyword))
}

fn clean_text(text: &str) -> String {
    text.replace("\n", "").replace(",", "")
}

fn extract_and_print_dates(text: &str, regex: &Regex) -> Result<(), Box<dyn Error>> {
    let date_regex = Regex::new(
        r"\s18\d{2}.|\s19[0-3]\d\s.|\s19[0-3]\d-[0-3]\d.|19\d{2}-40.|1940\s."
    )?;
    
    for cap in regex.captures_iter(text) {
        let interesting = &cap[1];
        if date_regex.is_match(interesting) {
            println!("{}", interesting);
        }
    }
    
    Ok(())
}
