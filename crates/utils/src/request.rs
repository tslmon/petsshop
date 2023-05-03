use crate::{settings::structs::Settings, AuthError};
use anyhow::anyhow;
use log::error;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Client;
use serde::Deserialize;
use std::future::Future;
use thiserror::Error;
use url::Url;

#[derive(Clone, Debug, Error)]
#[error("Error sending request, {0}")]
struct SendError(pub String);

#[derive(Clone, Debug, Error)]
#[error("Error receiving response, {0}")]
pub struct RecvError(pub String);

pub async fn retry<F, Fut, T>(f: F) -> Result<T, reqwest::Error>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, reqwest::Error>>,
{
    retry_custom(|| async { Ok((f)().await) }).await
}

async fn retry_custom<F, Fut, T>(f: F) -> Result<T, reqwest::Error>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Result<T, reqwest::Error>, reqwest::Error>>,
{
    let mut response: Option<Result<T, reqwest::Error>> = None;

    for _ in 0u8..3 {
        match (f)().await? {
            Ok(t) => return Ok(t),
            Err(e) => {
                if e.is_timeout() {
                    response = Some(Err(e));
                    continue;
                }
                return Err(e);
            }
        }
    }

    response.expect("retry http request")
}

#[derive(Deserialize, Debug)]
pub(crate) struct IframelyResponse {
    title: Option<String>,
    description: Option<String>,
    thumbnail_url: Option<Url>,
    html: Option<String>,
}

pub(crate) async fn fetch_iframely(
    client: &Client,
    url: &Url,
) -> Result<IframelyResponse, AuthError> {
    let fetch_url = format!("{}/oembed?url={}", Settings::get().iframely_url(), url);

    let response = retry(|| client.get(&fetch_url).send()).await?;

    let res: IframelyResponse = response
        .json()
        .await
        .map_err(|e| RecvError(e.to_string()))?;
    Ok(res)
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PictrsResponse {
    files: Vec<PictrsFile>,
    msg: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PictrsFile {
    file: String,
}

pub(crate) async fn fetch_files(
    client: &Client,
    image_url: &Url,
) -> Result<PictrsResponse, AuthError> {
    is_image_content_type(client, image_url).await?;

    let fetch_url = format!(
        "{}/image/download?url={}",
        Settings::get().files_url(),
        utf8_percent_encode(image_url.as_str(), NON_ALPHANUMERIC) // TODO this might not be needed
    );

    let response = retry(|| client.get(&fetch_url).send()).await?;

    let response: PictrsResponse = response
        .json()
        .await
        .map_err(|e| RecvError(e.to_string()))?;

    if response.msg == "ok" {
        Ok(response)
    } else {
        Err(anyhow!("{}", &response.msg).into())
    }
}

pub async fn fetch_iframely_and_files_data(
    client: &Client,
    url: Option<&Url>,
) -> (Option<String>, Option<String>, Option<String>, Option<Url>) {
    match &url {
        Some(url) => {
            // Fetch iframely data
            let (iframely_title, iframely_description, iframely_thumbnail_url, iframely_html) =
                match fetch_iframely(client, url).await {
                    Ok(res) => (res.title, res.description, res.thumbnail_url, res.html),
                    Err(e) => {
                        error!("iframely err: {}", e);
                        (None, None, None, None)
                    }
                };

            // Fetch files thumbnail
            let files_hash = match iframely_thumbnail_url {
                Some(iframely_thumbnail_url) => {
                    match fetch_files(client, &iframely_thumbnail_url).await {
                        Ok(res) => Some(res.files[0].file.to_owned()),
                        Err(e) => {
                            error!("files err: {}", e);
                            None
                        }
                    }
                }
                // Try to generate a small thumbnail if iframely is not supported
                None => match fetch_files(client, &url).await {
                    Ok(res) => Some(res.files[0].file.to_owned()),
                    Err(e) => {
                        error!("files err: {}", e);
                        None
                    }
                },
            };

            // The full urls are necessary
            let files_thumbnail = if let Some(files_hash) = files_hash {
                let url = Url::parse(&format!(
                    "{}/files/image/{}",
                    Settings::get().get_protocol_and_hostname(),
                    files_hash
                ));
                match url {
                    Ok(parsed_url) => Some(parsed_url),
                    Err(e) => {
                        // This really shouldn't happen unless the settings or hash are malformed
                        error!("Unexpected error constructing files thumbnail URL: {}", e);
                        None
                    }
                }
            } else {
                None
            };

            (
                iframely_title,
                iframely_description,
                iframely_html,
                files_thumbnail,
            )
        }
        None => (None, None, None, None),
    }
}

async fn is_image_content_type(client: &Client, test: &Url) -> Result<(), AuthError> {
    let response = retry(|| client.get(test.to_owned()).send()).await?;
    if response
        .headers()
        .get("Content-Type")
        .ok_or_else(|| anyhow!("No Content-Type header"))?
        .to_str()?
        .starts_with("image/")
    {
        Ok(())
    } else {
        Err(anyhow!("Not an image type.").into())
    }
}

#[cfg(test)]
mod tests {
    // These helped with testing
    // #[test]
    // fn test_iframely() {
    //   let res = fetch_iframely(client, "https://www.redspark.nu/?p=15341").await;
    //   assert!(res.is_ok());
    // }

    // #[test]
    // fn test_pictshare() {
    //   let res = fetch_pictshare("https://upload.wikimedia.org/wikipedia/en/2/27/The_Mandalorian_logo.jpg");
    //   assert!(res.is_ok());
    //   let res_other = fetch_pictshare("https://upload.wikimedia.org/wikipedia/en/2/27/The_Mandalorian_logo.jpgaoeu");
    //   assert!(res_other.is_err());
    // }
}
