use serde_json::Value;
use crate::model_domain::query_model::{
    ProjectInfo, EquipmentItem, ProjectQueryResponse, EquipmentQueryResponse
};

/// 查询服务 - 处理简道云API返回的数据
pub struct QueryService;

impl QueryService {
    /// 处理项目查询结果
    pub fn process_project_data(raw_data: &[Value]) -> ProjectQueryResponse {
        let projects = raw_data
            .iter()
            .filter_map(|item| Self::create_project_info(item))
            .collect();

        ProjectQueryResponse { projects }
    }

    /// 处理设备查询结果
    pub fn process_equipment_data(raw_data: &[Value]) -> EquipmentQueryResponse {
        let mut equipment_list = Vec::new();
        
        for record in raw_data {
            // 尝试从子表单中获取设备列表
            if let Some(items) = record.get("_widget_1635777115095").and_then(|v| v.as_array()) {
                for item in items {
                    if let Some(equipment) = Self::create_equipment_item(item) {
                        equipment_list.push(equipment);
                    }
                }
            } else {
                // 尝试直接从记录中获取设备信息
                if let Some(equipment) = Self::create_equipment_item(record) {
                    if !equipment.name.is_empty() {
                        equipment_list.push(equipment);
                    }
                }
            }
        }
        
        EquipmentQueryResponse { equipment_list }
    }

    /// 从简道云API返回的原始数据创建项目信息
    fn create_project_info(data: &Value) -> Option<ProjectInfo> {
        Some(ProjectInfo {
            id: data.get("_id")?.as_str()?.to_string(),
            project_name: data.get("_widget_1635777114903")?.as_str()?.to_string(),
            project_number: data.get("_widget_1635777114935")?.as_str()?.to_string(),
            design_number: data.get("_widget_1636359817201")?.as_str()?.to_string(),
            customer_name: data.get("_widget_1635777114972")?.as_str()?.to_string(),
            station_name: data.get("_widget_1635777114991")?.as_str()?.to_string(),
        })
    }

    /// 从简道云API返回的原始数据创建设备信息
    fn create_equipment_item(data: &Value) -> Option<EquipmentItem> {
        Some(EquipmentItem {
            id: data.get("_id")?.as_str()?.to_string(),
            name: data.get("_widget_1635777115211")?.as_str()?.to_string(),
            brand: data.get("_widget_1635777115248")?.as_str()?.to_string(),
            model: data.get("_widget_1635777115287")?.as_str()?.to_string(),
            tech_param: data.get("_widget_1641439264111")?.as_str()?.to_string(),
            quantity: data.get("_widget_1635777485580").and_then(|v| v.as_f64()).unwrap_or(0.0),
            unit: data.get("_widget_1654703913698")?.as_str()?.to_string(),
            external_param: data.get("_widget_1641439463480")?.as_str()?.to_string(),
        })
    }
} 