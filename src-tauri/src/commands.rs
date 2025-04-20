use tauri::command;
use crate::application_services::data_query_services::jdy_api::{create_jiandaoyun_client, DataQueryResponse};

#[command]
pub async fn query_jdy_data_by_project_number(project_number: Option<String>) -> Result<DataQueryResponse, String> {
    let client = create_jiandaoyun_client();
    match client.query_by_project_number(project_number).await {
        Ok(response) => Ok(response),
        Err(err) => Err(format!("查询数据失败: {}", err))
    }
}

#[command]
pub async fn query_equipment_by_station(station_name: String) -> Result<DataQueryResponse, String> {
    let client = create_jiandaoyun_client();
    match client.query_equipment_by_station(station_name).await {
        Ok(response) => Ok(response),
        Err(err) => Err(format!("查询设备清单失败: {}", err))
    }
}