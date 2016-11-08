extern crate rustc_serialize;
extern crate uuid;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use rustc_serialize::base64::URL_SAFE;
use rustc_serialize::base64::FromBase64Error;
use uuid::ParseError;


use uuid::Uuid;

#[derive(Debug)]
pub enum ConvertError{
    ParseError(ParseError),
    FromBase64Error(FromBase64Error)
}

trait ToBlob{
    fn to_blob(&self) -> String;
}

impl ToBlob for Uuid{
    fn to_blob(&self) -> String{
        self.as_bytes().to_base64(URL_SAFE)
    }
}

trait FromBlob{
    fn to_uuid(&self) -> Result<Uuid, ConvertError>;
}

impl FromBlob for str{
    fn to_uuid(&self) -> Result<Uuid, ConvertError>{
        match self.from_base64(){
            Ok(bytes) => {
                match Uuid::from_bytes(&bytes){
                    Ok(uuid) => Ok(uuid),
                    Err(e) => Err(ConvertError::ParseError(e))
                }
            },
            Err(e) => {
                Err(ConvertError::FromBase64Error(e))
            }
        }
    }
}

#[test]
fn test_blobs() {
    let uuid_str = "557c8018-5e21-4b74-8bb0-9040e2e8ead1";
    println!("uuid_str: {}", uuid_str);
    let blob = "VXyAGF4hS3SLsJBA4ujq0Q";
    println!("blob: {}", blob);
    let uuid = Uuid::parse_str(uuid_str).unwrap();
    assert_eq!(uuid, blob.to_uuid().unwrap());
    assert_eq!(blob, uuid.to_blob());
}

pub fn uuid_to_blob(uuid: &Uuid) -> String{
    uuid.to_blob()
}

pub fn blob_to_uuid(blob: &str) -> Result<Uuid, ConvertError>{
    blob.to_uuid()
}
