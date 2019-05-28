extern crate base64;
extern crate uuid;
use base64::DecodeError;
use uuid::BytesError;

use uuid::Uuid;

#[derive(Debug)]
pub enum ConvertError {
    BytesError(BytesError),
    FromBase64Error(DecodeError),
}

/// Convert uuid to a 22 character string blob
/// ```rust
///  extern crate uuid;
///  use uuid::Uuid;
///
///  extern crate blob_uuid;
///  fn main(){
///     let uuid = Uuid::parse_str("557c8018-5e21-4b74-8bb0-9040e2e8ead1").unwrap();
///     assert_eq!("VXyAGF4hS3SLsJBA4ujq0Q", blob_uuid::to_blob(&uuid));
///  }
///
/// ```
pub fn to_blob(uuid: &Uuid) -> String {
    base64::encode_config(uuid.as_bytes(), base64::URL_SAFE_NO_PAD)
}

/// Generate a random uuid and return its string blob
///
pub fn random_blob() -> String {
    let uuid = Uuid::new_v4();
    to_blob(&uuid)
}

/// Convert a string blob back to uuid
/// ```rust
/// extern crate uuid;
/// extern crate blob_uuid;
/// use uuid::Uuid;
/// fn main(){
///  let uuid = Uuid::parse_str("557c8018-5e21-4b74-8bb0-9040e2e8ead1").unwrap();
///  assert_eq!(uuid, blob_uuid::to_uuid("VXyAGF4hS3SLsJBA4ujq0Q").unwrap());
/// }
/// ```
pub fn to_uuid(blob: &str) -> Result<Uuid, ConvertError> {
    match base64::decode_config(blob, base64::URL_SAFE_NO_PAD) {
        Ok(bytes) => match Uuid::from_slice(&bytes) {
            Ok(uuid) => Ok(uuid),
            Err(e) => Err(ConvertError::BytesError(e)),
        },
        Err(e) => Err(ConvertError::FromBase64Error(e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blobs() {
        let uuid_str = "557c8018-5e21-4b74-8bb0-9040e2e8ead1";
        println!("uuid_str: {}", uuid_str);
        let blob = "VXyAGF4hS3SLsJBA4ujq0Q";
        println!("blob: {}", blob);
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        assert_eq!(uuid, to_uuid(blob).unwrap());
        assert_eq!(blob, to_blob(&uuid));
    }

    #[test]
    fn test_more_blobs() {
        for _ in 0..10 {
            let uuid = Uuid::new_v4();
            let blob = to_blob(&uuid);
            assert_eq!(blob.chars().count(), 22);
            println!("{}   |   {}", uuid, blob);
        }
    }
}
