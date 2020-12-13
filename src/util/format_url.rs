fn is_valid(number: &String) -> bool {
    return number.trim().parse::<f64>().is_ok();
}

pub async fn format_api_urls(media_numbers: &Vec<String>) -> Vec<String> {
    let mut urls = Vec::new();

    for media_number in media_numbers.iter() {
        let valid = is_valid(&media_number);
        if valid {
            urls.push(format!(
                "https://nhentai.net/api/gallery/{media}",
                media = media_number
            ));
        }
    }

    return urls;
}
