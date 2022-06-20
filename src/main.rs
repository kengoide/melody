use futures::executor::block_on;
use futures::stream::futures_unordered::Iter;
use scraper::Html;
use scraper::Selector;

fn collect_kansai_phil() -> Vec<String> {
    let url = "https://kansaiphil.jp/concert/";
    let mut response = block_on(surf::get(url)).unwrap();
    let body = block_on(response.body_string()).unwrap();
    let document = Html::parse_document(&body);

    let wrapper_selector = Selector::parse(".concert_post_wrapper").unwrap();
    let title_selector = Selector::parse(".concert_title").unwrap();

    let wrappers = document.select(&wrapper_selector);
    wrappers
        .map(|wrapper| {
            let mut title = wrapper.select(&title_selector);
            title.next().unwrap().text().collect::<Vec<_>>().join("")
        })
        .collect()
}

fn main() {
    for title in collect_kansai_phil() {
        println!("{}", title);
    }
}
