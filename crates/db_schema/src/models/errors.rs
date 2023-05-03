use errors_lib_rs::db::ModelErrorMessage;

pub struct PetsShopAPIError {}

impl ModelErrorMessage for PetsShopAPIError {
    fn uniqueviolation_message(_message: Option<&str>) -> Option<String> {
        if _message.is_some() {
            match _message.unwrap() {
                "ukey_submissionss" => Some(String::from("User already exists")),
                _or => Some(String::from("Unknown unique key violation")),
            }
        } else {
            None
        }
    }
    fn foreignkeyviolation_message(_message: Option<&str>) -> Option<String> {
        if _message.is_some() {
            match _message.unwrap() {
                "fkey_clients_tenant_id" => Some(String::from("User id does not exists")),
                _or => Some(String::from("Unknown foreign key violation")),
            }
        } else {
            None
        }
    }
}
