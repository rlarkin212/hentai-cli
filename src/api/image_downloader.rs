use std::convert::TryInto;

use indicatif::ProgressBar;
use reqwest::Response;

use super::{model::ApiModel, model::ImageModel};

pub async fn get_galleries_info(api_urls: &Vec<String>) -> Result<Vec<ApiModel>, Box<dyn std::error::Error>> {
    let mut api_models: Vec<ApiModel> = Vec::new();

    for api_url in api_urls.iter() {
        let res = reqwest::get(api_url).await?;
        let api_model: ApiModel = res.json().await?;

        api_models.push(api_model);
    }

    return Ok(api_models);
}

pub async fn get_images(api_model: &ApiModel) -> Result<Vec<ImageModel>, Box<dyn std::error::Error>> {
    let mut images: Vec<ImageModel> = Vec::new();

    let image_count: u64 = api_model.num_pages.try_into()?;
    println!("Fetching {} images - {}", image_count, &api_model.title.pretty);
    let progress_bar = ProgressBar::new(image_count);

    for page in 1..api_model.num_pages + 1 {
        let target = format!("https://i.nhentai.net/galleries/{}/{}.jpg", api_model.media_id, page);
        let response = reqwest::get(&target).await?;
        
        let file_name = generate_file_name(&response).await?;
        let file_contents= response.bytes().await?;

        let image_model = ImageModel{
            file_name: file_name,
            file_contents: file_contents,
        };

        images.push(image_model);
        progress_bar.inc(1)
    }

    progress_bar.finish();

    Ok(images)
}

async fn generate_file_name (response: &Response) -> Result<String, Box<dyn std::error::Error>>{
    let f_name = response
    .url()
    .path_segments()
    .and_then(|segments| segments.last())
    .and_then(|name| if name.is_empty() { None } else { Some(name) })
    .unwrap_or("bigtiddy.bin");

    let file_name = String::from(f_name);

    return Ok(file_name);
}