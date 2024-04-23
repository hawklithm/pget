extern crate reqwest;

use reqwest::header::HeaderMap;

use self::reqwest::header;
use self::reqwest::Client;
use self::reqwest::Response;

const CONTENT_RANGE: &str = "Content-Range";
pub struct Network {
    pub client: Client,
}

impl Default for Network {
    fn default() -> Network {
        Network {
            client: Client::new(),
        }
    }
}

impl Network {
    pub async fn make_request(
        &self,
        url: &String,
        range_opt: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let request = if let Some(range) = range_opt {
            self.client.get(url).header(header::RANGE, range)
        } else {
            self.client.get(url)
        };

        return request.send().await;
    }

    pub async fn get_content_length(&self, url: &String) -> Result<Option<u64>, reqwest::Error> {
        let res = self
            .make_request(url, Some("bytes=0-0".to_string()))
            .await?;
        let headers = res.headers();
        let content_range = get_length_from_meta(headers);
        Ok(content_range)
    }
}

fn get_length_from_meta(headers: &HeaderMap) -> Option<u64> {
    let content_range = headers.get(CONTENT_RANGE)?;
    let size = content_range
        .to_str()
        .ok()?
        .split('/')
        .last()?
        .parse::<u64>()
        .ok()?;
    Some(size)
}
