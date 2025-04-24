mod application_services;
mod commands;
mod model_domain;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_process::init())
    .invoke_handler(tauri::generate_handler![
      query_jdy_data_by_project_number,//查询JDY数据
      query_equipment_by_station,//查询设备数据
      process_station_data,//处理场站数据
      generate_io_point_table,//生成IO点表
      open_file//打开文件
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}




