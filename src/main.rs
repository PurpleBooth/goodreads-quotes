use rand::seq::SliceRandom;
use std::error::Error;

use clap::{crate_authors, crate_version, App, Arg};
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("tag")
                .help("Goodreads tag")
                .multiple(true)
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let mut rng = rand::thread_rng();
    let mut tags: Vec<_> = matches.values_of("tag").unwrap().collect();
    tags.shuffle(&mut rng);
    let tag = tags.get(0).unwrap();

    let selector = Selector::parse("div.quoteText").unwrap();
    let response =
        reqwest::blocking::get(&format!("https://www.goodreads.com/quotes/tag/{}", tag))?;
    let document = response
        .text()
        .map(|source| Html::parse_document(&source))?;

    let mut quotes: Vec<_> = document
        .select(&selector)
        .map(|element| {
            element
                .text()
                .take(6)
                .map(str::trim)
                .map(String::from)
                .filter(|value| !value.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    quotes.shuffle(&mut rng);

    println!(
        "{}",
        quotes.first().ok_or_else(|| -> Box<dyn Error> {
            format!("No quotes found for \"{}\"", tag).into()
        })?
    );

    Ok(())
}
