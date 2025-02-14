use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaResponse {
    pub file_name: String,
    pub file_data: Vec<u8>,
    pub size: u32,
    pub mime_type: String,
}

pub async fn fetch_image(
    token: String,
    file_name: String,
    // file_data: Vec<u8>,
) -> Option<Vec<u8>> {
    // Send the file data to the Next.js API
    let response = Request::get("http://localhost:3000/api/media/image")
        .header("Authorization", &format!("Bearer {}", token)) // Replace with your JWT token
        // .header("X-File-Name", &file_name) // Include the file name
        .query([("filename", file_name)])
        // .body(file_data)
        // .expect("Couldn't add request body") // Send the raw bytes
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.ok() {
                log::info!("File fetched successfully!");

                // Parse the JSON response
                let fetch_response: Vec<u8> =
                    resp.binary().await.expect("Failed to parse upload response");

                Some(fetch_response)
            } else {
                log::error!("fetched failed: {}", resp.status_text());

                None
            }
        }
        Err(err) => {
            log::error!("fetched error: {:?}", err);

            None
        }
    }
}