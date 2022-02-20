use std::{
    convert::TryInto,
    env,
    error::Error,
    fs::{create_dir, File},
    io::{copy, Cursor},
};

use indicatif::ProgressBar;

mod api;
mod cli;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_model = cli::CliModel {
        media: env::args().collect(),
        current_directory: env::current_dir().unwrap(),
    };

    let api_urls: Vec<String> = util::format_api_urls(&cli_model.media).await;
    let api_models = api::get_galleries_info(&api_urls).await?;

    for api_model in api_models.iter() {
        let dir_name = format!("{} - {}", &api_model.id, &api_model.title.pretty);
        let new_path_buf = util::generate_path_buf(&dir_name);
        let images = api::get_images(&api_model).await?;

        create_dir(&new_path_buf)?;

        let image_count: u64 = images.len().try_into()?;
        println!("Saving {} images - {}", image_count, api_model.title.pretty);
        let bar = ProgressBar::new(image_count);

        for image in images {
            let destination_file_path = format!("{}/{}", dir_name, image.file_name);

            let mut destination = File::create(destination_file_path)?;
            let mut file_content = Cursor::new(image.file_contents);

            copy(&mut file_content, &mut destination)?;
            bar.inc(1)
        }

        bar.finish();
    }

    Ok(())
}
