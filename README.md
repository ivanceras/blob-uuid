
[![](https://travis-ci.org/ivanceras/blob-uuid.svg?branch=master)](https://travis-ci.org/ivanceras/blob-uuid)

Convert Uuid to a url friendly 22 character string blob

This is useful for url links to a unique record,  and is shorter than the 36 character stringified uuid.


### Example usage

```rust

   let uuid = Uuid::parse_str("557c8018-5e21-4b74-8bb0-9040e2e8ead1").unwrap();
   assert_eq!("VXyAGF4hS3SLsJBA4ujq0Q", blob_uuid::to_blob(uuid));
```


#### Example conversions:
uuid                                    |   blob
----------------------------------------|-------------------------
 8ef65ee9-a039-4bf2-a4b3-687fcc1f3cc3   |   jvZe6aA5S_Kks2h_zB88ww
 c38ba949-491b-417d-b488-aa4748c13a00   |   w4upSUkbQX20iKpHSME6AA
 d1504e8e-dff6-4b9b-a0b1-50c447b5d1a4   |   0VBOjt_2S5ugsVDER7XRpA
 8f5159c4-0438-45f1-a7e1-f30c84258082   |   j1FZxAQ4RfGn4fMMhCWAgg
 6aaf25bd-7d59-4331-87a1-4ddacf6aa52c   |   aq8lvX1ZQzGHoU3az2qlLA
 95af4181-bbbc-4b0c-9d9a-c14fa0a18c65   |   la9Bgbu8SwydmsFPoKGMZQ
 2a317215-53fd-45de-95e9-f74c16a13b6a   |   KjFyFVP9Rd6V6fdMFqE7ag
 8d2c25e2-73d6-4f4a-820a-f671c0991398   |   jSwl4nPWT0qCCvZxwJkTmA
 55458e6b-fcce-446e-884f-ca61a8fd7e9d   |   VUWOa_zORG6IT8phqP1-nQ


### Using the cli
```
cargo install blob-uuid-cli

blob-uuid 8ef65ee9-a039-4bf2-a4b3-687fcc1f3cc3
jvZe6aA5S_Kks2h_zB88ww
```
Calling `blob-uuid` without argument will generate a return a blob from a generated uuid

