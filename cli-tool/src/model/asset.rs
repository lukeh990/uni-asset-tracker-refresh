use super::*;
use chrono::NaiveDate;

pub struct Asset {
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
    verified_by: Option<IdNamePair>,
}

impl Asset {
    pub fn new(
        id: String,
        asset_tag_number: String,
        serial_number: String,
        asset_type: IdNamePair,
    ) -> Self {
        Self {
            id,
            asset_tag_number,
            assignee: None,
            user_type: None,
            department: None,
            room: None,
            asset_type,
            serial_number,
            description: String::new(),
            purchase_date: None,
            batch_id: None,
            warranty_length: None,
            removal_info: None,
            service_tag: String::new(),
            ky_doe_id: String::new(),
            is_active: true,
            verified_by: None,
        }
    }
}
