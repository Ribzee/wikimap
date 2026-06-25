use reqwest::{Response, Url};
use select::{document::Document, predicate::Name};

pub async fn get_wikipedia_links(response: Response) -> Result<Vec<Url>, anyhow::Error> {
    let url = response.url().to_owned();
    let body = response.text().await?;

    let links: Vec<Url> = Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|node| node.attr("href"))
        .filter_map(|link| url.join(link).ok())
        .filter(|link| {
            link.to_string().contains("en.wikipedia.org/wiki")
                && !link.to_string().contains("/wiki/Main_Page")
                && !link.to_string().contains(url.as_str())
        })
        .collect();
    Ok(links)
}
