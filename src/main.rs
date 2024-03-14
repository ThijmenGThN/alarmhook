use reqwest::blocking as fetch;
use scraper;

struct Call {
    date: String,
    message: String,
}

fn main() {
    collect_calls()
        .iter()
        .for_each(|call| println!("{}: {}\n", call.date, call.message));
}

fn collect_calls() -> Vec<Call> {
    let res = fetch::get("https://p2000mobiel.nl/4/ijsselland.html")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&res);
    let selector = scraper::Selector::parse(".call").unwrap();
    let collection = document.select(&selector);

    let calls = collection.map(|raw| {
        let call = Call {
            date: raw
                .select(&scraper::Selector::parse(".date").unwrap())
                .next()
                .unwrap()
                .inner_html(),
            message: raw
                .select(&scraper::Selector::parse(".message").unwrap())
                .next()
                .unwrap()
                .inner_html(),
        };

        call
    });

    calls.collect()
}
