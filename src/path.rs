use std::collections::HashMap;
use std::collections::VecDeque;

use reqwest::Url;

use crate::{parser::get_wikipedia_links, request::RequestHandler};

pub struct PathTracer {
    client: RequestHandler,
    start: Url,
    end: Url,
    connections: HashMap<Url, Vec<Url>>,
    parent: HashMap<Url, Url>,
    queue: VecDeque<Url>,
}

impl PathTracer {
    pub fn new(start: Url, end: Url) -> Self {
        Self {
            client: RequestHandler::default(),
            start,
            end,
            connections: HashMap::new(),
            parent: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    pub async fn trace_path(&mut self, limit: u32) -> Result<Vec<Url>, anyhow::Error> {
        println!("Tracing path between {} and {}", self.start, self.end);
        self.queue.push_back(self.start.clone());
        for _ in 0..limit {
            // path 1
            let link_1 = self
                .queue
                .pop_front()
                .ok_or(anyhow::anyhow!("Unexpected error happened :( 1"))?;
            self.connections.insert(link_1.clone(), Vec::new());
            for link in get_wikipedia_links(self.client.request(link_1.clone()).await?)
                .await?
                .iter()
            {
                self.connections
                    .get_mut(&link_1)
                    .ok_or(anyhow::anyhow!("Unexpected Error happenend :( 2"))?
                    .push(link.clone());

                if !self.parent.contains_key(link) {
                    self.parent.insert(link.clone(), link_1.clone());
                }

                if !self.queue.contains(link) && !self.connections.contains_key(link) {
                    self.queue.push_back(link.clone());
                }

                if link == &self.end {
                    let mut path = VecDeque::from(vec![link.clone()]);
                    let mut current = link;
                    while current != &self.start {
                        current = self
                            .parent
                            .get(current)
                            .ok_or(anyhow::anyhow!("Unexpected error happened :( 3"))?;
                        path.push_front(current.clone());
                    }
                    return Ok(Vec::from(path));
                }
            }
        }
        Err(anyhow::anyhow!(
            "Could not find path between {} and {} for limit of {}",
            self.start,
            self.end,
            limit
        ))
    }
}
