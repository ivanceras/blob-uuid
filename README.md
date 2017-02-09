Convert Uuid to a url friendly 22 character string blob

```rs
    let uuid_str = "557c8018-5e21-4b74-8bb0-9040e2e8ead1";
    println!("uuid_str: {}", uuid_str);
    let blob = "VXyAGF4hS3SLsJBA4ujq0Q";
    println!("blob: {}", blob);
    let uuid = Uuid::parse_str(uuid_str).unwrap();
    assert_eq!(uuid, blob.to_uuid().unwrap());
    assert_eq!(blob, uuid.to_blob());
```

    uuid                               |   blob
---------------------------------------|-------------------------
8ef65ee9-a039-4bf2-a4b3-687fcc1f3cc3   |   jvZe6aA5S_Kks2h_zB88ww
c38ba949-491b-417d-b488-aa4748c13a00   |   w4upSUkbQX20iKpHSME6AA
d1504e8e-dff6-4b9b-a0b1-50c447b5d1a4   |   0VBOjt_2S5ugsVDER7XRpA
8f5159c4-0438-45f1-a7e1-f30c84258082   |   j1FZxAQ4RfGn4fMMhCWAgg
6aaf25bd-7d59-4331-87a1-4ddacf6aa52c   |   aq8lvX1ZQzGHoU3az2qlLA
95af4181-bbbc-4b0c-9d9a-c14fa0a18c65   |   la9Bgbu8SwydmsFPoKGMZQ
2a317215-53fd-45de-95e9-f74c16a13b6a   |   KjFyFVP9Rd6V6fdMFqE7ag
8d2c25e2-73d6-4f4a-820a-f671c0991398   |   jSwl4nPWT0qCCvZxwJkTmA
55458e6b-fcce-446e-884f-ca61a8fd7e9d   |   VUWOa_zORG6IT8phqP1-nQ

