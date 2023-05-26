fn main() {
    let movie_number = 100;
    // println is a macro, we need mark ! as a 
    println!("We are scraping top {} movies from imdb!", movie_number);
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    titles
        .zip(1..movie_number + 1)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
