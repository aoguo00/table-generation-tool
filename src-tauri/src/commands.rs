use tauri::command;
use crate::application_services::jdy_api_services::jdy_api::{create_jiandaoyun_client, DataQueryResponse};
use crate::model_domain::query_model::{ProjectQueryResponse, EquipmentQueryResponse};
use crate::application_services::query_services::query_service::QueryService;
use crate::application_services::excel_services::io_excel_services::{IOExcelService, EquipmentData, convert_equipment_items};
use std::collections::HashMap;
use std::fs;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use std::path::Path;
use tauri_plugin_dialog::FilePath;

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
            // println!("{:#?}",equipment_response);
            Ok(equipment_response)
        },
        Err(err) => Err(format!("查询设备清单失败: {}", err))
    }
}

#[command]
pub async fn process_station_data(
    #[allow(non_snake_case)] equipmentData: Vec<serde_json::Value>
) -> Result<HashMap<String, serde_json::Value>, String> {
    // 转换设备数据
    let equipment_list = convert_equipment_items(equipmentData);
    
    // 处理数据并获取统计结果
    match IOExcelService::process_station_data(&equipment_list) {
        Ok(channel_totals) => {
            // 将结构体转换为可序列化的Map
            let result: HashMap<String, serde_json::Value> = channel_totals
                .into_iter()
                .map(|(key, value)| {
                    (key, serde_json::json!({
                        "count": value.count,
                        "data_type": value.data_type
                    }))
                })
                .collect();
            
            Ok(result)
        },
        Err(e) => Err(format!("处理场站数据失败: {}", e))
    }
}

#[command]
pub async fn generate_io_point_table(
    #[allow(non_snake_case)] equipmentData: Vec<serde_json::Value>,
    #[allow(non_snake_case)] stationName: String,
    window: tauri::Window
) -> Result<String, String> {
    // 转换设备数据
    let equipment_list = convert_equipment_items(equipmentData);
    
    // 生成临时点表文件
    let temp_file_path = match IOExcelService::generate_io_table(&equipment_list, &stationName) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };
    
    // 提示用户选择保存位置
    let file_name = format!("{}_IO点表.xlsx", stationName);
    
    // 获取应用句柄
    let app_handle = window.app_handle();
    
    // 使用阻塞调用打开保存文件对话框
    let save_path = app_handle.dialog()
        .file()
        .add_filter("Excel文件", &["xlsx"])
        .set_file_name(&file_name)
        .blocking_save_file();
    
    // 处理结果
    match save_path {
        Some(filepath) => {
            // 将FilePath转换为标准路径
            let path_str = filepath.to_string();
            let dest_path = Path::new(&path_str);
            let src_path = Path::new(&temp_file_path);
            
            // 将临时文件复制到用户选择的位置
            match fs::copy(src_path, dest_path) {
                Ok(_) => Ok(path_str),
                Err(e) => Err(format!("保存文件失败: {}", e))
            }
        },
        None => Err("用户取消了保存操作".to_string())
    }
}

