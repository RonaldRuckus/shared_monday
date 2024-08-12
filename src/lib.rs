use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedAdapterError {
    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),
    #[error("Data field not found: {0}")]
    DataFieldNotFound(String)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MessageStatus {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "unknown")]
    Unknown,
}

impl MessageStatus{
    pub fn from_string(status: &str) -> MessageStatus {
        match status {
            "sent" => MessageStatus::Sent,
            "delivered" => MessageStatus::Delivered,
            "read" => MessageStatus::Read,
            "failed" => MessageStatus::Failed,
            "pending" => MessageStatus::Pending,
            _ => MessageStatus::Unknown
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MessageStatus::Sent => "sent".to_string(),
            MessageStatus::Delivered => "delivered".to_string(),
            MessageStatus::Read => "read".to_string(),
            MessageStatus::Failed => "failed".to_string(),
            MessageStatus::Pending => "pending".to_string(),
            MessageStatus::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub recipient_id: String,
    pub status: MessageStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    name: Option<String>,
    id: Option<String>,
    column_values: Option<Vec<HashMap<String, serde_json::Value>>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemsPage {
    items: Vec<Item>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct LeadDetails {
    pub name: String,
    pub phone_number: String
}

impl LeadDetails {
    pub fn new(name: String, mut phone_number: String) -> Result<LeadDetails, SharedAdapterError> {

        // Ensure that the phone number is either 10 or 11 digits
        if phone_number.len() != 10 && phone_number.len() != 11 {
            return Err(SharedAdapterError::InvalidPhoneNumber(phone_number));
        }

        // If the phone number is 10 digits, add a '1' to the beginning
        if phone_number.len() == 10 {
            phone_number.insert(0, '1');
        }

        // Get the first name from the full `name`
        let name = name.split_whitespace().next().unwrap().to_string();

        Ok(LeadDetails {
            name,
            phone_number
        })
    }
}

impl TryFrom<ItemsPage> for LeadDetails {
    type Error = SharedAdapterError;

    fn try_from(items_page: ItemsPage) -> Result<Self, Self::Error> {
        let item = items_page.items.first().ok_or(SharedAdapterError::DataFieldNotFound("items".to_string()))?;

        let name = item.name.clone().ok_or(SharedAdapterError::DataFieldNotFound("name".to_string()))?;
        let column_values = item.column_values.clone().ok_or(SharedAdapterError::DataFieldNotFound("column_values".to_string()))?;

        let phone_number = column_values
            .iter()
            .find(|column_value| {
                column_value.get("text").map(|text| text.as_str().unwrap_or_default().contains("1")).unwrap_or_default()
            })
            .ok_or(SharedAdapterError::DataFieldNotFound("phone_number".to_string()))?
            .get("text")
            .ok_or(SharedAdapterError::DataFieldNotFound("text".to_string()))?
            .as_str()
            .ok_or(SharedAdapterError::DataFieldNotFound("text".to_string()))?
            .to_string();

        Ok(LeadDetails::new(name, phone_number)?)
    }
}