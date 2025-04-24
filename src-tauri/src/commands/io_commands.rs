use tauri::command;
use std::collections::HashMap;
use std::fs;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use std::path::Path;
use defer;
use crate::application_services::excel_services::io_excel_services::{IOExcelService, convert_equipment_items};
use std::process::Command;

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
    
    // 创建临时文件的路径
    let temp_path = Path::new(&temp_file_path);
    
    // 确保在函数结束时删除临时文件
    let _cleanup = defer::defer(|| {
        if let Err(e) = fs::remove_file(temp_path) {
            eprintln!("删除临时文件失败: {}", e);
        }
    });
    
    // 使用spawn_blocking处理文件保存对话框
    let app_handle_clone = app_handle.clone();
    let file_name_clone = file_name.clone();
    let save_path = tauri::async_runtime::spawn_blocking(move || {
        app_handle_clone.dialog()
            .file()
            .add_filter("Excel文件", &["xlsx"])
            .set_file_name(&file_name_clone)
            .blocking_save_file()
    }).await.map_err(|e| format!("对话框操作失败: {}", e))?;
    
    // 处理结果
    match save_path {
        Some(filepath) => {
            // 将FilePath转换为标准路径
            let path_str = filepath.to_string();
            let dest_path = Path::new(&path_str);
            
            // 使用spawn_blocking处理文件复制操作
            let temp_path = temp_path.to_path_buf();
            let dest_path = dest_path.to_path_buf();
            match tauri::async_runtime::spawn_blocking(move || {
                fs::copy(&temp_path, &dest_path)
            }).await.map_err(|e| format!("文件复制操作失败: {}", e))? {
                Ok(_) => Ok(path_str),
                Err(e) => Err(format!("保存文件失败: {}", e))
            }
        },
        None => Err("用户取消了保存操作".to_string())
    }
}

#[command]
pub async fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("打开文件失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开文件失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开文件失败: {}", e))?;
    }

    Ok(())
} 