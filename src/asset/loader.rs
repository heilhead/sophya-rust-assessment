use super::*;
use crate::render::create_texture_resource;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

const CACHE_DIR: &str = ".cache";

fn load_data_from_url(url: &str) -> anyhow::Result<bytes::Bytes> {
    // This wasn't necessary, but resource loading took awfully long time on each program start,
    // so I thought that this may actually save some development time.
    let hash = Sha256::new().chain_update(url).finalize();
    let cache_file = format!("{CACHE_DIR}/{:x}.png", hash);
    let cache_file = Path::new(&cache_file);

    if cache_file.exists() {
        println!("loading cached data: {cache_file:?}");

        // Correction: it wasn't slow because of download, but because of PNG decoding... oh well =\
        let mut stream = BufReader::new(File::open(cache_file)?);
        let mut buffer = vec![];
        stream.read_to_end(&mut buffer)?;

        Ok(bytes::Bytes::from(buffer))
    } else {
        println!("downloading data: {url}");

        let result = reqwest::blocking::get(url)?.bytes()?;
        let mut stream = BufWriter::new(File::create(cache_file)?);
        stream.write(&result)?;

        Ok(result)
    }
}

fn load_asset_metadata_from_file(path: &str) -> anyhow::Result<AssetMetadata> {
    println!("loading asset metadata: {path}");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = serde_json::from_reader(reader)?;

    Ok(input)
}

fn load_asset_resources(asset: &AssetMetadata) -> anyhow::Result<AssetResourceList> {
    let mut result = HashMap::new();

    let texture_list = asset
        .orientations
        .iter()
        .map(|(_, x)| x.images.iter())
        .flatten()
        .map(|x| (x.0 /* image ID */, &x.1.url /* image URL */))
        .collect::<Vec<_>>();

    for (image_id, image_url) in texture_list {
        let texture = create_texture_resource(load_data_from_url(image_url)?);
        let insert_result = result.insert(image_id.clone(), texture);

        // Make sure resources are unique.
        assert!(insert_result.is_none());
    }

    Ok(result)
}

pub fn load_asset(path: &str) -> anyhow::Result<Asset> {
    // Entire asset loading call is blocking, because of the issues with `reqwest` async loader
    // requiring `tokio` reactor running for async requests. Didn't have time to fix :(
    let metadata = load_asset_metadata_from_file(&path)?;
    let resources = load_asset_resources(&metadata)?;

    Ok(Asset { metadata, resources })
}

pub fn load_asset_bundle(assets: &[&str]) -> anyhow::Result<Vec<Asset>> {
    println!("loading asset bundle: {} assets", assets.len());

    let current_dir: String = String::from(std::env::current_dir()?.as_path().to_str().unwrap());
    let mut result = vec![];

    for asset_name in assets {
        result.push(load_asset(&format!("{current_dir}/{asset_name}"))?);
    }

    println!("bundle loaded successfully");

    Ok(result)
}

pub fn load_texture_from_file(path: &str) -> anyhow::Result<TextureResource> {
    println!("loading texture from file: {path:?}");

    let mut stream = BufReader::new(File::open(path)?);
    let mut buffer = vec![];
    stream.read_to_end(&mut buffer)?;

    Ok(create_texture_resource(bytes::Bytes::from(buffer)))
}
