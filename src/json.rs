pub mod json_data {
    use serde_json;
    use serde::{ Serialize, Deserialize };

    #[derive(Serialize, Deserialize)]
    pub struct JSONData {
        pub data: serde_json::Value
    }
}