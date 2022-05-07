use reqwest::StatusCode;     
use scraper::{Html, Selector}; 

mod utils;   //importing utils.rs file

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let url = "https://in.indeed.com/jobs?q=rust%20developer&l=Remote&vjk=e7f445bef2886a7e";
    
    let result = client.get(url).send().await.unwrap();
    /*
    .get() makes get request to the url passed as parameter
    .send() sends the get request
    .await waits for the response object
    */

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),       //.text() returns the response as string
        _ => panic!("Something went wrong"),
    };

    let document = Html::parse_document(&raw_html);    //parse_document() parses the string of raw_html as a document
    let article_selector = Selector::parse("a.tapItem").unwrap();  //.parse() parses a CSS selector 
    let span_select=Selector::parse("span").unwrap();
    
    
    // a reference to our Selector instance is passed to the select() function on the parsed document
    for element in document.select(&article_selector) {

    // looping on it will produce a list of ElementRef instances if any elements are found that match 
    // the criteria of the selector object
    
        let mut span_el=element.select(&span_select);  // here we use select function and pass selector object

        let href = match element.value().attr("href")   // .value().attr grabs the links in href
        {
            Some(target_url) => target_url,   //gives url if found 
            _ => "no url found",                    //else gives no url found
        };

        match span_el.next()    //.next() is called on every object which advances the iterator and returns the next value
        {
            Some(elref) => {
                let extra: String = String::from("new");
                let inner=&elref.inner_html().to_string();
                if inner.eq(&extra){
                    continue;
                }
                println!("Job: {}",&elref.inner_html().to_string());
                println!("Link: https:://www.indeed.com{}",&href);
                continue;
            }
            _ => {},
        }
    }
}
