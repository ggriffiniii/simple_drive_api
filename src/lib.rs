use hyper::Client;
use yup_oauth2 as oauth2;
use std::error::Error;
pub use google_api3::FieldSelector;
use serde::{Deserialize, de::DeserializeOwned};
use url::Url;

pub struct Drive<'a, A> {
    client: &'a mut hyper::Client,
    authenticator: A,
}


impl<'a, A> Drive<'a, A> where A: oauth2::GetToken {
    pub fn new(client: &'a mut Client, authenticator: A) -> Self {
        Drive{client, authenticator}
    }

    pub fn list_files<T>(&mut self) -> Result<Vec<T>, Box<Error>> where T: FieldSelector + DeserializeOwned {
        #[derive(Deserialize,FieldSelector)]
        struct ListFilesResponse<T> where T: FieldSelector {
            files: Vec<T>,
        }
        use std::io::Read;
        use hyper::header::{Authorization, Bearer};
        let mut params = Vec::new();
        params.push(("alt", "json".to_owned()));
        let field_selector = ListFilesResponse::<T>::field_selector();
        println!("field_selector: {}", field_selector);
        params.push(("fields", ListFilesResponse::<T>::field_selector()));
        let mut url = Url::parse("https://www.googleapis.com/drive/v3/files")?;
        url.query_pairs_mut().clear().extend_pairs(params);
        let token = self.authenticator.token(&["https://www.googleapis.com/auth/drive"])?;
        let auth_header = Authorization(Bearer { token: token.access_token });
        println!("making request for {}", url.as_str());
        let mut resp = self.client.request(hyper::method::Method::Get, url.as_str()).header(auth_header).send()?;
        let mut json_response = String::new();
        resp.read_to_string(&mut json_response).unwrap();
        println!("json response: {}", &json_response);
        let resp: ListFilesResponse<_> = serde_json::from_str(&json_response)?;
        Ok(resp.files)
    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
