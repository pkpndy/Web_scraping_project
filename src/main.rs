use reqwest::StatusCode;     
use scraper::{Html, Selector}; 

mod utils;   //importing utils.rs file

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let url = "https://in.indeed.com/jobs?q=rust%20developer&l=Remote&vjk=e7f445bef2886a7e";
    let result = client.get(url).send().await.unwrap();
    
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    let document = Html::parse_document(&raw_html);
    let article_selector = Selector::parse("a.tapItem").unwrap();
    let span_select=Selector::parse("span").unwrap();
    

    for element in document.select(&article_selector) {
        let inner = element.inner_html().to_string();
        
        let mut span_el=element.select(&span_select);
        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };

        match span_el.next(){
            Some(elref) => {
                println!("Job: {}",&elref.inner_html().to_string());
                println!("Link: https:://www.indeed.com{}",&href);
                continue;
            }
            _ => {},
        }
        
        println!("{}", &inner);
        println!("{}", &href);
    }
}

