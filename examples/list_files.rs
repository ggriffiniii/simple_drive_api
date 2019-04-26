use simple_drive_api::{Drive, FieldSelector};
use yup_oauth2::{DefaultAuthenticatorDelegate, DiskTokenStorage};
use std::error::Error;
use serde::Deserialize;

#[derive(Debug,Deserialize,FieldSelector)]
#[serde(rename_all = "camelCase")]
struct FileAttrs {
    id: String,
    name: String,
}

#[derive(Debug,Deserialize,FieldSelector)]
#[serde(rename_all = "camelCase")]
struct ExtendedFileAttrs {
    id: String,
    name: String,
    sharing_user: Option<UserInfo>,
    owners: Vec<UserInfo>,
}

#[derive(Debug,Deserialize,FieldSelector)]
#[serde(rename_all = "camelCase")]
struct UserInfo {
    email_address: String,
    display_name: String,
}

fn main() -> Result<(), Box<Error>> {
    let mut client = init_hyper_client()?;
    let mut drive = Drive::new(&mut client, init_authenticator()?);

    // This makes a list files request specifying to only return the fields
    // contained in FileAttrs. The resulting field_specifier is:
    // 'files(id,name)'
    let files: Vec<FileAttrs> = drive.list_files()?;
    println!("files: {:?}", files);
    // This makes a list files request specifying to only return the fields
    // contained in ExtendedFileAttrs. The resulting field_specifier is:
    // 'files(id,name,sharingUser/emailAddress,sharingUser/displayName,owners(emailAddress,displayName))'
    let files: Vec<ExtendedFileAttrs> = drive.list_files()?;
    println!("files: {:?}", files);
    Ok(())
}

type Authenticator = yup_oauth2::Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>;

fn init_authenticator() -> Result<Authenticator, Box<Error>> {
    use yup_oauth2::{
        read_application_secret, ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate,
        DiskTokenStorage, FlowType,
    };
    use std::path::Path;
    let secret: ApplicationSecret = read_application_secret(Path::new("credentials.json"))?;
    let client = init_hyper_client()?;
    Ok(Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        client,
        DiskTokenStorage::new(&"token_store.json".to_string())?,
        Some(FlowType::InstalledInteractive),
    ))
}

fn init_hyper_client() -> Result<hyper::Client, Box<Error>> {
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;
    Ok(hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new()?)))
}