use serde::{Deserialize, Serialize};

/// 项目信息模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub id: String,
    pub project_name: String,
    pub project_number: String,
    pub design_number: String,
    pub customer_name: String,
    pub station_name: String,
}

/// 设备信息模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentItem {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub model: String,
    pub tech_param: String,
    pub quantity: f64,
    pub unit: String,
    pub external_param: String,
}

/// 项目查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectQueryResponse {
    pub projects: Vec<ProjectInfo>,
}

/// 设备查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentQueryResponse {
    pub equipment_list: Vec<EquipmentItem>,
} 