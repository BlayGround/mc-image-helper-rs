pub mod cli;
pub mod minecraft;

use std::{cmp::min, error::Error, fs::File, io::Write, path::PathBuf};

use clap::Parser;
use cli::Commands;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use minecraft::LoaderType;
use modrinth_api::apis::{configuration::Configuration, versions_api};
use reqwest::Client;
use sha2::{Digest, Sha512};

pub async fn download_file(
    client: &Client,
    url: &str,
    path: &PathBuf,
    orig_hash: Option<&String>,
) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Downloading {}", url));

    // Download chunks into memory
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut hasher = Sha512::new();
    let mut data = Vec::new(); // Store downloaded data in memory

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;

        // Update SHA-512 hash
        hasher.update(&chunk);

        // Store data in memory
        data.extend_from_slice(&chunk);

        // Update progress bar
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} (pending hash verification)", url));

    // Finalize SHA-512 hash computation
    let calculated_hash = format!("{:x}", hasher.finalize());

    // Check if hash verification is required
    if let Some(expected_hash) = orig_hash {
        if &calculated_hash != expected_hash {
            return Err(format!(
                "Hash mismatch! Expected: {}, Found: {}",
                expected_hash, calculated_hash
            ));
        }
    }

    // Write to file only if hash verification passes
    let mut file =
        File::create(path).or(Err(format!("Failed to create file '{}'", path.display())))?;
    file.write_all(&data).or(Err(format!(
        "Error while writing to file '{}'",
        path.display()
    )))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Configuration::new();
    let cli = cli::Cli::parse();

    match cli.command {
        Commands::InstallModrinthModpack(args) => {
            let loader_str = format!(
                "{:?}",
                args.loader
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(LoaderType::as_str)
                    .collect::<Vec<_>>()
            );

            let game_version_str = if let Some(ref game_version) = args.game_version {
                Some(format!("{:?}", game_version))
            } else {
                None
            };

            let versions = versions_api::get_project_versions(
                &config,
                &args.project,
                Some(&loader_str),
                game_version_str.as_deref(),
                None,
            )
            .await?;

            let matched_version = if args.version == "latest" {
                versions.first().expect("Failed get versions")
            } else {
                versions
                    .iter()
                    .find(|v| v.version_number.contains(&args.version))
                    .expect("No matching version exists")
            };

            for f in matched_version.files.iter() {
                let orig_hash = f.hashes.sha512.as_ref();
                let client = Client::new();
                download_file(
                    &client,
                    &f.url,
                    &args.output_directory.join(f.filename.as_str()),
                    orig_hash,
                )
                .await?
            }
        }
    }

    Ok(())
}
