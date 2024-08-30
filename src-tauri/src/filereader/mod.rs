mod parse_ads;
mod parse_app;
mod parse_xml;

use std::path::PathBuf;


/// Module will be contain all file parsers for metadata files types
/// For now I expect only 3 imported types : ads, app and XML
/// Later it could be possible to implement import from Excel file, but this is not structured files

enum MetadataImportTypes {
    Ads,
    App,
    Xml
}

fn parse_file(path: PathBuf) -> Result<(), String>{
    let file_extention = path.extension().unwrap().to_str().unwrap().to_lowercase();

    let parser_type = match file_extention.as_str() {
        "ads" => MetadataImportTypes::Ads,
        "app" => MetadataImportTypes::App,
        "xml" => MetadataImportTypes::Xml,
        _ => return Err(String::from("Unknown file type passed"))
    };

    Ok(())
}


