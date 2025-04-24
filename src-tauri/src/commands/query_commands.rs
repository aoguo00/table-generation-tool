use tauri::command;
use crate::application_services::jdy_api_services::jdy_api::create_jiandaoyun_client;
use crate::model_domain::query_model::{ProjectQueryResponse, EquipmentQueryResponse};
use crate::application_services::query_services::query_service::QueryService;

#[command]
pub async fn query_jdy_data_by_project_number(
    #[allow(non_snake_case)] projectNumber: Option<String>
) -> Result<ProjectQueryResponse, String> {
    let client = create_jiandaoyun_client();
    match client.query_by_project_number(projectNumber).await {
        Ok(response) => {
            // 使用查询服务处理数据
            let project_response = QueryService::process_project_data(&response.data);
            Ok(project_response)
        },
        Err(err) => Err(format!("查询数据失败: {}", err))
    }
}

#[command]
pub async fn query_equipment_by_station(
    #[allow(non_snake_case)] stationName: String
) -> Result<EquipmentQueryResponse, String> {
    let client = create_jiandaoyun_client();
    match client.query_equipment_by_station(stationName).await {
        Ok(response) => {
            // 使用查询服务处理数据
            let equipment_response = QueryService::process_equipment_data(&response.data);
            Ok(equipment_response)
        },
        Err(err) => Err(format!("查询设备清单失败: {}", err))
    }
} 