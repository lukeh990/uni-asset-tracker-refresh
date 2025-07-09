# Universithy Asset Tracker Refresh

A refreshed frontend for my uni's in house asset tracker.

I do work for my university's IT department and we have to use this in house
asset tracking solution. I don't like the UI, so I have decided to use what is
essentially a web scraper to try and build my own UI.

## Notes

Don't mind this. Just somewhere for me to put notes.

### Actual API

GET /Asset

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
  X-Requested-WithS tring ALWAYS "XMLHttpRequest"
```

POST /IDLookup/CXIDLookupSearch

```txt
  id String
  LastName String
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
  PurchaseDate String
  Batch_Id String
  WarrantyLength String
  WarrantyEnd String
  RemovalDate String
  RemovalMethod_Id String
  ServiceTag String
  KY_DOE_ID String
  IsActive [String]
    0	String
    1 String ALWAYS "false"
  Verified [String]
    0	String
    1 String ALWAYS "false"
  CC_Staff_Id String
  __RequestVerificationToken String
```

### Desired API

```rust
fn find_assets(findable_asset FindableAsset) -> Result<Vec<Asset>, ApiError> {}

fn edit_asset(editable_asset EditableAsst) -> Result<(), ApiError> {}

fn get_asset(asset_tag String) -> Result<Asset, ApiError> {}

fn populate_indices() -> Result<Indices, ApiError> {}

fn search_assignee(id String, last_name String) -> Result<Assignee, ApiError> {}
```
