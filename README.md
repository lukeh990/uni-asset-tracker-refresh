# University Asset Tracker Refresh

A refreshed frontend for my uni's in house asset tracker.

I do work for my university's IT department and we have to use this in house
asset tracking solution. I don't like the UI, so I have decided to use what is
essentially a web scraper to try and build my own UI.

## Notes

Don't mind this. Just somewhere for me to put notes.

### Actual API

GET /Asset/Edit/[ID]

POST /Asset/SearchAssetsByAttribute

```txt
  AssetTagNumber String
  Assignee_Id String
  UserType_Id Sring
  Department_Id String
  URL_RoomIDLookup String ALWAYS "/AssetTracker/RoomLookup/CXIDSearchAgain?viewName=LocationPartial"
  URL_RoomIDLookupSearch String ALWAYS "/AssetTracker/RoomLookup/CXIDLookupSearch"
  Room_Id String
  Fullname String
  LastName String ALWAYS "LocationPartial"
  Category_Id String
  AssetType_Id String
  SerialNumber String
  IsActive String
  __RequestVerificationToken CRSF_TOKEN
  X-Requested-With String ALWAYS "XMLHttpRequest"
```

POST /IDLookup/CXIDLookupSearch

```txt
  id String
  lastName String
```

POST /RoomLookup/CXIDLookupSearch

```txt
  fullName String
  lastName String ALWAYS "RoomIDPartial" or "LocationPartial"
```

POST /Asset/Edit/[ID]

```txt
  Id String
  AssetTagNumber String
  URL_CXIDLookup String ALWAYS "/AssetTracker/IDLookup/CXIDSearchAgain"
  URL_CXIDLookupSearch String ALWAYS "/AssetTracker/IDLookup/CXIDLookupSearch"
  Assignee_Id String
  UserType_Id	String
  Department_Id	String
  URL_RoomIDLookup String ALWAYS "/AssetTracker/RoomLookup/CXIDSearchAgain?viewName=RoomIDPartial"
  URL_RoomIDLookupSearch String ALWAYS "/AssetTracker/RoomLookup/CXIDLookupSearch"
  Room_Id	String
  Fullname String
  LastName String ALWAYS "RoomIDPartial"
  AssetType_Id String
  SerialNumber String
  ADescription String
  PurchaseDate String (ISO8601)
  Batch_Id String
  WarrantyLength String
  WarrantyEnd String (ISO8601)
  RemovalDate String (ISO8601)
  RemovalMethod_Id String
  ServiceTag String
  KY_DOE_ID String
  IsActive String
  Verified String
  CC_Staff_Id String
  __RequestVerificationToken CRSF_TOKEN
```

### Desired API

```rust
struct IdNamePair {
  id: String,
  name: String
}

struct RemovalInfo {
  removal_date: NaiveDate,
  removal_method: IdNamePair
}

struct Asset {
  id: String,
  asset_tag_number: String,
  assignee: Option<IdNamePair>,
  user_type: Option<IdNamePair>,
  department: Option<IdNamePair>,
  room: Option<IdNamePair>,
  asset_type: IdNamePair,
  serial_number: String,
  description: String,
  purchase_date: Option<NaiveDate>,
  batch_id: Option<IdNamePair>,
  warranty_length: Option<String>,
  removal_info: Option<RemovalInfo>,
  service_tag: String,
  ky_doe_id: String,
  is_active: bool,
  verified_by: Option<IdNamePair>
}
```
