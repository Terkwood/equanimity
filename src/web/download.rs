use crate::web::StorageState;

const FILE_TYPE: &str = "application/json";

pub fn provide_data(state: &StorageState) -> Result<String, ProvideDataErr> {
    if let Ok(ser) = serde_json::to_string(state) {
        let encoded: String = js_sys::encode_uri_component(&ser).into();

        Ok(format!("data:{};charset=utf-8,{}", FILE_TYPE, encoded))
    } else {
        Err(ProvideDataErr)
    }
}

pub struct ProvideDataErr;
