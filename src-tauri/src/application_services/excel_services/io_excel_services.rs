use crate::model_domain::io_table_model::{IOTable, IOTableRow, IO_TABLE_HEADERS};
use std::path::Path;
use umya_spreadsheet::{Spreadsheet, Worksheet, Style, Border, Color, PatternFill};
use umya_spreadsheet::reader::xlsx;
use umya_spreadsheet::writer::xlsx::write;
use std::collections::HashMap;
use serde_json;

/// IO通道类型
pub enum IOChannelType {
    AI,
    AO,
    DI,
    DO,
}

impl ToString for IOChannelType {
    fn to_string(&self) -> String {
        match self {
            IOChannelType::AI => "AI".to_string(),
            IOChannelType::AO => "AO".to_string(),
            IOChannelType::DI => "DI".to_string(),
            IOChannelType::DO => "DO".to_string(),
        }
    }
}

/// 数据类型
pub enum DataType {
    REAL,
    BOOL,
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            DataType::REAL => "REAL".to_string(),
            DataType::BOOL => "BOOL".to_string(),
        }
    }
}

/// 设备信息
#[derive(Debug, Clone)]
pub struct EquipmentData {
    pub equipment_name: String,
    pub spec_model: String,
    pub quantity: u32,
    pub station_name: String,
    // 可以添加其他设备相关字段
}

/// 通道数据统计结果
#[derive(Debug, Clone)]
pub struct ChannelTotal {
    pub count: u32,
    pub data_type: String,
}

/// 需要使用BOOL类型地址的字段前缀
pub const BOOL_TYPE_ADDRESS_FIELDS: [&str; 5] = [
    "LL报警", "L报警", "H报警", "HH报警", "维护使能开关点位"
];

/// 需要用户填写的字段（将在导出时高亮显示）
pub const HIGHLIGHT_FIELDS: [&str; 11] = [
    "供电类型（有源/无源）", "线制", "位号", "变量名称（HMI）", "变量描述",
    "量程低限", "量程高限", "SLL设定值", "SL设定值", "SH设定值", "SHH设定值"
];

/// 设备型号与通道的映射
pub struct ModelChannelMapping {
    pub model_key: String,
    pub channel_type: IOChannelType,
    pub channels: u32,
    pub data_type: DataType,
}

/// 将前端设备项转换为内部设备数据结构
pub fn convert_equipment_items(equipment_items: Vec<serde_json::Value>) -> Vec<EquipmentData> {
    equipment_items.into_iter()
        .filter_map(|item| {
            let name = item.get("name")?.as_str()?.to_string();
            let model = item.get("model")?.as_str()?.to_string();
            let quantity = item.get("quantity")?.as_f64()? as u32;
            let station_name = item.get("station_name")?.as_str()?.to_string();
            
            Some(EquipmentData {
                equipment_name: name,
                spec_model: model,
                quantity,
                station_name,
            })
        })
        .collect()
}

/// IO通道服务
pub struct IOExcelService;

impl IOExcelService {
    /// 获取设备型号映射
    pub fn get_model_channel_mapping() -> Vec<ModelChannelMapping> {
        vec![
            ModelChannelMapping {
                model_key: "LK610".to_string(),
                channel_type: IOChannelType::DI,
                channels: 16,
                data_type: DataType::BOOL,
            },
            ModelChannelMapping {
                model_key: "LK710".to_string(),
                channel_type: IOChannelType::DO,
                channels: 16,
                data_type: DataType::BOOL,
            },
            ModelChannelMapping {
                model_key: "LK411".to_string(),
                channel_type: IOChannelType::AI,
                channels: 8,
                data_type: DataType::REAL,
            },
            ModelChannelMapping {
                model_key: "LK512".to_string(),
                channel_type: IOChannelType::AO,
                channels: 8,
                data_type: DataType::REAL,
            },
        ]
    }

    /// 判断字段是否应该使用BOOL类型地址
    pub fn is_bool_address_field(field_name: &str) -> bool {
        BOOL_TYPE_ADDRESS_FIELDS.iter().any(|prefix| field_name.starts_with(prefix))
    }
    
    /// 根据设备清单计算各类型通道总数及数据类型
    pub fn calculate_channels(equipment_list: &[EquipmentData]) -> HashMap<String, ChannelTotal> {
        // 初始化结果
        let mut channel_totals = HashMap::new();
        channel_totals.insert("AI".to_string(), ChannelTotal { count: 0, data_type: "REAL".to_string() });
        channel_totals.insert("AO".to_string(), ChannelTotal { count: 0, data_type: "REAL".to_string() });
        channel_totals.insert("DI".to_string(), ChannelTotal { count: 0, data_type: "BOOL".to_string() });
        channel_totals.insert("DO".to_string(), ChannelTotal { count: 0, data_type: "BOOL".to_string() });
        
        // 获取设备型号映射
        let model_channel_mapping = Self::get_model_channel_mapping();
        
        // 处理每个设备
        for equipment in equipment_list {
            // 获取设备规格型号
            let spec_model = &equipment.spec_model;
            if spec_model.is_empty() {
                continue;
            }
            
            // 获取数量
            let quantity = equipment.quantity;
            if quantity == 0 {
                continue;
            }
            
            // 查找匹配的设备型号
            for model in model_channel_mapping.iter() {
                if spec_model.contains(&model.model_key) {
                    let channel_type = model.channel_type.to_string();
                    let channel_count = model.channels;
                    
                    // 计算总通道数并添加到对应类型
                    let total_channels = quantity * channel_count;
                    if let Some(total) = channel_totals.get_mut(&channel_type) {
                        total.count += total_channels;
                    }
                    break;
                }
            }
        }
        
        channel_totals
    }
    
    /// 根据PLC绝对地址计算上位机通讯地址
    pub fn calculate_modbus_address(plc_address: &str, data_type: &str) -> u32 {
        if data_type == "REAL" {
            // 对于REAL类型：=(MID(AE2,4,4)/2)+43001
            // 从%MD100中提取100，然后计算
            let md_num = plc_address[3..].parse::<u32>().unwrap_or(0);
            (md_num / 2) + 43001
        } else {
            // 对于BOOL类型：=(MID(AE3,4,2)*8)+RIGHT(AE3,1)+3001
            // 从%MX20.0中提取20和0，然后计算
            let parts: Vec<&str> = plc_address[3..].split('.').collect();
            let mx_num = parts[0].parse::<u32>().unwrap_or(0);
            let bit_num = if parts.len() > 1 { parts[1].parse::<u32>().unwrap_or(0) } else { 0 };
            (mx_num * 8) + bit_num + 3001
        }
    }

    /// 将IO通道数据导出到Excel
    pub fn export_to_excel(
        equipment_list: &[EquipmentData], 
        output_path: &Path, 
        station_name: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 创建新的电子表格
        let mut spreadsheet = umya_spreadsheet::new_file();
        let worksheet = spreadsheet.get_active_sheet_mut();
        
        // 准备IO点表数据
        let io_points = Self::prepare_io_points(equipment_list);
        
        // 创建IO表
        let mut io_table = IOTable::new(format!("{}_IO表", station_name));
        for point in io_points {
            io_table.add_row(point);
        }
        
        // 写入表头
        Self::write_headers(worksheet);
        
        // 写入数据、设置公式和占位符
        Self::write_data_and_formulas(worksheet, &io_table);
        
        // 调整列宽 - 自动适应内容
        Self::adjust_column_widths(worksheet, &io_table);
        
        // 保存Excel
        write(&spreadsheet, output_path)?;
        
        Ok(())
    }

    /// 准备IO点表数据
    fn prepare_io_points(equipment_list: &[EquipmentData]) -> Vec<IOTableRow> {
        let mut io_points = Vec::new();
        
        // 通道计数器（用于生成通道位号）
        let mut module_counters = HashMap::new();
        module_counters.insert("AI", 1);
        module_counters.insert("AO", 1);
        module_counters.insert("DI", 1);
        module_counters.insert("DO", 1);
        
        // 序号计数器
        let mut index_counter = 1;
        
        // PLC地址计数器
        let mut real_address_counter = 320; // %MD320开始
        let mut bool_address_counter = (20, 0); // %MX20.0开始，范围是20-300
        
        // 机架信息
        let rack_count = Self::get_rack_count(equipment_list);
        
        // 当前槽位跟踪
        let mut current_rack = 1;
        let mut current_slot = 2; // 从2开始，因为1号槽位用于LK232通信模块
        
        // 可用槽位数量（每个机架10个可用槽，第一个槽位用于通信模块）
        let available_slots_per_rack = 10;
        
        // 获取设备型号映射
        let model_channel_mapping = Self::get_model_channel_mapping();
        
        // 按照IO类型对设备进行分类
        let io_equipment_groups = Self::group_equipment_by_io_type(equipment_list, &model_channel_mapping);
        
        // 按照AI/AO/DI/DO的顺序遍历处理设备
        for io_type in &["AI", "AO", "DI", "DO"] {
            if let Some(equipment_group) = io_equipment_groups.get(*io_type) {
                for equipment in equipment_group {
                    let spec_model = &equipment.spec_model;
                    // 获取该设备的通道信息
                    let mut channel_info = None;
                    
                    for model in &model_channel_mapping {
                        if spec_model.contains(&model.model_key) {
                            channel_info = Some((
                                model.channel_type.to_string(),
                                model.channels,
                                model.data_type.to_string()
                            ));
                            break;
                        }
                    }
                    
                    if let Some((io_type_str, channels, data_type)) = channel_info {
                        let quantity = equipment.quantity;
                        let equipment_name = &equipment.equipment_name;
                        let station_name = &equipment.station_name;
                        
                        // 为每个设备的每个通道创建单独的点表条目
                        for _ in 0..quantity {
                            // 获取当前模块号
                            let _module_num = *module_counters.get(&io_type_str as &str).unwrap_or(&1);
                            
                            // 计算机架号和槽号
                            if current_slot > 11 || current_slot > available_slots_per_rack + 1 {
                                current_rack += 1;
                                current_slot = 2; // 重置为2，跳过第一个槽位
                                
                                // 检查是否超出机架数量
                                if current_rack > rack_count {
                                    panic!("IO模块数量超出了可用机架数量，请增加机架数量");
                                }
                            }
                            
                            // 为该模块的每个通道创建条目
                            for ch in 0..channels {
                                // 生成新的通道位号格式（例如：1_1_AO_0）
                                let channel_code = format!("{}_{}_{}_{}", current_rack, current_slot, io_type_str, ch);
                                
                                // 生成PLC绝对地址和更新地址计数器
                                let (plc_address, modbus_address) = Self::generate_addresses(
                                    &data_type, 
                                    &mut real_address_counter, 
                                    &mut bool_address_counter
                                );
                                
                                // 创建点表数据
                                let point_data = Self::create_io_point(
                                    index_counter,
                                    equipment_name,
                                    &io_type_str,
                                    &channel_code,
                                    station_name,
                                    &data_type,
                                    &plc_address,
                                    modbus_address
                                );
                                
                                // 添加到点表列表
                                io_points.push(point_data);
                                index_counter += 1;
                            }
                            
                            // 每个设备模块增加模块计数器和槽位计数器
                            if let Some(counter) = module_counters.get_mut(&io_type_str as &str) {
                                *counter += 1;
                            }
                            current_slot += 1;
                        }
                    }
                }
            }
        }
        
        io_points
    }

    /// 获取机架数量
    fn get_rack_count(equipment_list: &[EquipmentData]) -> u32 {
        for equipment in equipment_list {
            if equipment.spec_model.contains("LK117") {
                return equipment.quantity;
            }
        }
        1 // 默认为1个机架
    }

    /// 按IO类型分组设备
    fn group_equipment_by_io_type(
        equipment_list: &[EquipmentData], 
        model_mapping: &[ModelChannelMapping]
    ) -> HashMap<String, Vec<EquipmentData>> {
        let mut groups = HashMap::new();
        groups.insert("AI".to_string(), Vec::new());
        groups.insert("AO".to_string(), Vec::new());
        groups.insert("DI".to_string(), Vec::new());
        groups.insert("DO".to_string(), Vec::new());
        
        // 遍历设备列表进行分类
        for equipment in equipment_list {
            let spec_model = &equipment.spec_model;
            // 检查是否是IO模块
            for model in model_mapping {
                if spec_model.contains(&model.model_key) {
                    let io_type = model.channel_type.to_string();
                    if let Some(group) = groups.get_mut(&io_type) {
                        group.push(equipment.clone()); // 这里还是需要克隆，因为要存储到新的集合中
                    }
                    break;
                }
            }
        }
        
        groups
    }

    /// 生成PLC和通信地址
    fn generate_addresses(
        data_type: &str, 
        real_counter: &mut u32, 
        bool_counter: &mut (u32, u32)
    ) -> (String, u32) {
        if data_type == "REAL" {
            let addr = format!("%MD{}", *real_counter);
            let modbus = Self::calculate_modbus_address(&addr, data_type);
            *real_counter += 4; // REAL类型每个点位加4
            (addr, modbus)
        } else {
            // BOOL类型
            let addr = format!("%MX{}.{}", bool_counter.0, bool_counter.1);
            let modbus = Self::calculate_modbus_address(&addr, data_type);
            
            // 更新BOOL地址计数器
            bool_counter.1 += 1;
            if bool_counter.1 > 7 {
                bool_counter.0 += 1;
                bool_counter.1 = 0;
            }
            
            (addr, modbus)
        }
    }

    /// 创建IO点表行
    fn create_io_point(
        index: u32,
        equipment_name: &str,
        io_type: &str,
        channel_code: &str,
        station_name: &str,
        data_type: &str,
        plc_address: &str,
        modbus_address: u32
    ) -> IOTableRow {
        let mut point = IOTableRow::default();
        
        // 基本信息
        point.index = Some(index.to_string());
        point.module_name = Some(equipment_name.to_string());
        point.module_type = Some(io_type.to_string());
        point.channel_tag = Some(channel_code.to_string());
        point.station_name = Some(station_name.to_string());
        point.data_type = Some(data_type.to_string());
        
        // 通用属性
        point.read_write_property = Some("R/W".to_string());
        point.save_history = Some("是".to_string());
        point.power_off_protection = Some("是".to_string());
        
        // 特定类型属性
        if io_type == "AO" {
            point.power_supply_type = Some("/".to_string());
            point.wire_system = Some("/".to_string());
        }
        
        // 数据类型相关设置
        match data_type {
            "REAL" => {
                // REAL类型需要设置量程
                point.range_lower_limit = Some("".to_string());
                point.range_upper_limit = Some("".to_string());
                point.sll_value = Some("".to_string());
                point.sl_value = Some("".to_string());
                point.sh_value = Some("".to_string());
                point.shh_value = Some("".to_string());
            },
            _ => {
                // BOOL类型使用"/"表示不适用
                point.range_lower_limit = Some("/".to_string());
                point.range_upper_limit = Some("/".to_string());
                point.sll_value = Some("/".to_string());
                point.sl_value = Some("/".to_string());
                point.sh_value = Some("/".to_string());
                point.shh_value = Some("/".to_string());
            }
        }
        
        // 地址信息
        point.plc_absolute_address = Some(plc_address.to_string());
        point.host_comm_address = Some(modbus_address.to_string());
        
        point
    }

    /// 写入表头
    fn write_headers(worksheet: &mut Worksheet) {
        for (col_idx, header) in IO_TABLE_HEADERS.iter().enumerate() {
            let col = col_idx as u32 + 1;
            worksheet.get_cell_mut((col, 1)).set_value(header.to_string());
            
            // 设置表头样式
            let style = worksheet.get_style_mut((col, 1));
            style.get_font_mut().set_bold(true);
            
            // 设置边框
            style.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_THIN);
            style.get_borders_mut().get_top_mut().set_border_style(Border::BORDER_THIN);
            style.get_borders_mut().get_left_mut().set_border_style(Border::BORDER_THIN);
            style.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_THIN);
            
            // 高亮需要用户填写的字段
            if HIGHLIGHT_FIELDS.contains(header) {
                // 直接在style上设置背景色
                style.set_background_color(Color::COLOR_YELLOW.to_string());
            }
        }
    }

    /// 写入数据、设置公式和占位符
    fn write_data_and_formulas(worksheet: &mut Worksheet, io_table: &IOTable) {
        // PLC地址计数器(用于生成额外地址)
        let mut real_address_counter = 320; // %MD320开始
        let mut bool_address_counter = (20, 0); // %MX20.0开始，范围是20-300
        
        for (row_idx, row_data) in io_table.rows.iter().enumerate() {
            let row = row_idx as u32 + 2; // 从第2行开始（跳过表头）
            
            // 获取数据类型
            let data_type = row_data.data_type.as_deref().unwrap_or("BOOL");
            
            // 写入单元格数据
            Self::write_row_data(worksheet, row, row_data, data_type);
            
            // 设置Excel公式或占位符
            if data_type == "REAL" {
                Self::set_formulas_for_real_type(worksheet, row);
            } else {
                Self::set_placeholders_for_bool_type(worksheet, row);
            }
            
            // 设置维护值设定为"/"
            if let Some(maint_val_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == "维护值设定") {
                worksheet.get_cell_mut((maint_val_pos as u32 + 1, row)).set_value("/".to_string());
            }
            
            // 为各个点位添加PLC地址和通信地址
            Self::add_addresses_for_setpoints(worksheet, row, &mut real_address_counter, &mut bool_address_counter);
        }
    }

    /// 写入单行数据
    fn write_row_data(worksheet: &mut Worksheet, row: u32, row_data: &IOTableRow, data_type: &str) {
        for (col_idx, header) in IO_TABLE_HEADERS.iter().enumerate() {
            let col = col_idx as u32 + 1;
            
            // 获取对应字段的值并写入
            let cell_value: Option<String> = Self::get_field_value(row_data, header);
            
            // 保存单元格值的字符串形式,用于后续判断是否高亮
            let cell_value_str = cell_value.as_deref().unwrap_or("");
            
            // 使用克隆来避免移动已借用的值
            if let Some(ref value) = cell_value {
                worksheet.get_cell_mut((col, row)).set_value(value.clone());
            }
            
            // 设置边框样式
            let style = worksheet.get_style_mut((col, row));
            Self::apply_border_style(style);
            
            // 高亮需要用户填写的字段
            if HIGHLIGHT_FIELDS.contains(header) && !(data_type == "BOOL" && header.contains("量程")) {
                // 只有当值不是"/"时才高亮
                if cell_value_str != "/" {
                    style.set_background_color(Color::COLOR_YELLOW.to_string());
                }
            }
        }
    }

    /// 应用边框样式
    fn apply_border_style(style: &mut Style) {
        style.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_THIN);
        style.get_borders_mut().get_top_mut().set_border_style(Border::BORDER_THIN);
        style.get_borders_mut().get_left_mut().set_border_style(Border::BORDER_THIN);
        style.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_THIN);
    }

    /// 获取特定字段的值而不使用克隆
    fn get_field_value(row_data: &IOTableRow, header: &str) -> Option<String> {
        match header {
            "序号" => row_data.index.clone(),
            "模块名称" => row_data.module_name.clone(),
            "模块类型" => row_data.module_type.clone(),
            "供电类型（有源/无源）" => row_data.power_supply_type.clone(),
            "线制" => row_data.wire_system.clone(),
            "通道位号" => row_data.channel_tag.clone(),
            "位号" => row_data.tag.clone(),
            "场站名" => row_data.station_name.clone(),
            "变量名称（HMI）" => row_data.variable_name_hmi.clone(),
            "变量描述" => row_data.variable_description.clone(),
            "数据类型" => row_data.data_type.clone(),
            "读写属性" => row_data.read_write_property.clone(),
            "保存历史" => row_data.save_history.clone(),
            "掉电保护" => row_data.power_off_protection.clone(),
            "量程低限" => row_data.range_lower_limit.clone(),
            "量程高限" => row_data.range_upper_limit.clone(),
            "SLL设定值" => row_data.sll_value.clone(),
            "SLL设定点位" => row_data.sll_setpoint.clone(),
            "SLL设定点位_PLC地址" => row_data.sll_setpoint_plc_address.clone(),
            "SLL设定点位_通讯地址" => row_data.sll_setpoint_comm_address.clone(),
            "SL设定值" => row_data.sl_value.clone(),
            "SL设定点位" => row_data.sl_setpoint.clone(),
            "SL设定点位_PLC地址" => row_data.sl_setpoint_plc_address.clone(),
            "SL设定点位_通讯地址" => row_data.sl_setpoint_comm_address.clone(),
            "SH设定值" => row_data.sh_value.clone(),
            "SH设定点位" => row_data.sh_setpoint.clone(),
            "SH设定点位_PLC地址" => row_data.sh_setpoint_plc_address.clone(),
            "SH设定点位_通讯地址" => row_data.sh_setpoint_comm_address.clone(),
            "SHH设定值" => row_data.shh_value.clone(),
            "SHH设定点位" => row_data.shh_setpoint.clone(),
            "SHH设定点位_PLC地址" => row_data.shh_setpoint_plc_address.clone(),
            "SHH设定点位_通讯地址" => row_data.shh_setpoint_comm_address.clone(),
            "LL报警" => row_data.ll_alarm.clone(),
            "LL报警_PLC地址" => row_data.ll_alarm_plc_address.clone(),
            "LL报警_通讯地址" => row_data.ll_alarm_comm_address.clone(),
            "L报警" => row_data.l_alarm.clone(),
            "L报警_PLC地址" => row_data.l_alarm_plc_address.clone(),
            "L报警_通讯地址" => row_data.l_alarm_comm_address.clone(),
            "H报警" => row_data.h_alarm.clone(),
            "H报警_PLC地址" => row_data.h_alarm_plc_address.clone(),
            "H报警_通讯地址" => row_data.h_alarm_comm_address.clone(),
            "HH报警" => row_data.hh_alarm.clone(),
            "HH报警_PLC地址" => row_data.hh_alarm_plc_address.clone(),
            "HH报警_通讯地址" => row_data.hh_alarm_comm_address.clone(),
            "维护值设定" => row_data.maintenance_value.clone(),
            "维护值设定点位" => row_data.maintenance_setpoint.clone(),
            "维护值设定点位_PLC地址" => row_data.maintenance_setpoint_plc_address.clone(),
            "维护值设定点位_通讯地址" => row_data.maintenance_setpoint_comm_address.clone(),
            "维护使能开关点位" => row_data.maintenance_enable_switch.clone(),
            "维护使能开关点位_PLC地址" => row_data.maintenance_enable_switch_plc_address.clone(),
            "维护使能开关点位_通讯地址" => row_data.maintenance_enable_switch_comm_address.clone(),
            "PLC绝对地址" => row_data.plc_absolute_address.clone(),
            "上位机通讯地址" => row_data.host_comm_address.clone(),
            _ => None,
        }
    }

    /// 为REAL类型设置公式
    fn set_formulas_for_real_type(worksheet: &mut Worksheet, row: u32) {
        // 点位配置，格式为 (列名, 后缀)
        let point_configs = [
            ("SLL设定点位", "_LoLoLimit"),
            ("SL设定点位", "_LoLimit"),
            ("SH设定点位", "_HiLimit"),
            ("SHH设定点位", "_HiHiLimit"),
            ("LL报警", "_LL"),
            ("L报警", "_L"),
            ("H报警", "_H"),
            ("HH报警", "_HH"),
            ("维护值设定点位", "_whz"),
            ("维护使能开关点位", "_whzzt"),
        ];
        
        // HMI变量所在的列索引是I列
        for (column_name, suffix) in &point_configs {
            if let Some(pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *column_name) {
                let formula = format!("=IF(ISBLANK(I{}),\"{}\",I{}&\"{}\")", row, suffix, row, suffix);
                worksheet.get_cell_mut((pos as u32 + 1, row)).set_formula(formula);
            }
        }
    }
    
    /// 为BOOL类型设置占位符
    fn set_placeholders_for_bool_type(worksheet: &mut Worksheet, row: u32) {
        // 需要设置"/"的列
        let placeholder_columns = [
            "SLL设定点位", "SL设定点位", "SH设定点位", "SHH设定点位",
            "LL报警", "L报警", "H报警", "HH报警",
            "维护值设定点位", "维护使能开关点位", "维护值设定",
            // PLC地址和通信地址
            "SLL设定点位_PLC地址", "SLL设定点位_通讯地址",
            "SL设定点位_PLC地址", "SL设定点位_通讯地址",
            "SH设定点位_PLC地址", "SH设定点位_通讯地址",
            "SHH设定点位_PLC地址", "SHH设定点位_通讯地址",
            "LL报警_PLC地址", "LL报警_通讯地址",
            "L报警_PLC地址", "L报警_通讯地址",
            "H报警_PLC地址", "H报警_通讯地址",
            "HH报警_PLC地址", "HH报警_通讯地址",
            "维护值设定点位_PLC地址", "维护值设定点位_通讯地址",
            "维护使能开关点位_PLC地址", "维护使能开关点位_通讯地址",
        ];
        
        for column_name in &placeholder_columns {
            if let Some(pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *column_name) {
                worksheet.get_cell_mut((pos as u32 + 1, row)).set_value("/".to_string());
            }
        }
    }
    
    /// 为各个点位添加PLC地址和通信地址
    fn add_addresses_for_setpoints(
        worksheet: &mut Worksheet, 
        row: u32, 
        real_address_counter: &mut u32, 
        bool_address_counter: &mut (u32, u32)
    ) {
        // REAL类型地址的点位配置
        let real_address_configs = [
            ("SLL设定点位_PLC地址", "SLL设定点位_通讯地址"),
            ("SL设定点位_PLC地址", "SL设定点位_通讯地址"),
            ("SH设定点位_PLC地址", "SH设定点位_通讯地址"),
            ("SHH设定点位_PLC地址", "SHH设定点位_通讯地址"),
            ("维护值设定点位_PLC地址", "维护值设定点位_通讯地址"),
        ];
        
        // 为REAL类型地址点位添加PLC地址和通信地址
        for (plc_addr_col, comm_addr_col) in &real_address_configs {
            if let Some(addr_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *plc_addr_col) {
                let extra_plc_addr = format!("%MD{}", *real_address_counter);
                worksheet.get_cell_mut((addr_pos as u32 + 1, row)).set_value(extra_plc_addr.clone());
                
                // 计算并设置通信地址
                let extra_comm_addr = Self::calculate_modbus_address(&extra_plc_addr, "REAL");
                if let Some(comm_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *comm_addr_col) {
                    worksheet.get_cell_mut((comm_pos as u32 + 1, row)).set_value(extra_comm_addr.to_string());
                }
                *real_address_counter += 4;
            }
        }
        
        // BOOL类型地址的点位配置
        let bool_address_configs = [
            ("LL报警_PLC地址", "LL报警_通讯地址"),
            ("L报警_PLC地址", "L报警_通讯地址"),
            ("H报警_PLC地址", "H报警_通讯地址"),
            ("HH报警_PLC地址", "HH报警_通讯地址"),
            ("维护使能开关点位_PLC地址", "维护使能开关点位_通讯地址"),
        ];
        
        // 为BOOL类型地址点位添加PLC地址和通信地址
        for (plc_addr_col, comm_addr_col) in &bool_address_configs {
            if let Some(addr_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *plc_addr_col) {
                let extra_plc_addr = format!("%MX{}.{}", bool_address_counter.0, bool_address_counter.1);
                worksheet.get_cell_mut((addr_pos as u32 + 1, row)).set_value(extra_plc_addr.clone());
                
                // 计算并设置通信地址
                let extra_comm_addr = Self::calculate_modbus_address(&extra_plc_addr, "BOOL");
                if let Some(comm_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *comm_addr_col) {
                    worksheet.get_cell_mut((comm_pos as u32 + 1, row)).set_value(extra_comm_addr.to_string());
                }
                // 更新BOOL地址计数器
                bool_address_counter.1 += 1;
                if bool_address_counter.1 > 7 {
                    bool_address_counter.0 += 1;
                    bool_address_counter.1 = 0;
                }
            }
        }
    }
    
    /// 自动调整列宽
    fn adjust_column_widths(worksheet: &mut Worksheet, io_table: &IOTable) {
        let mut column_widths = vec![15.0f64; IO_TABLE_HEADERS.len()]; // 默认宽度为15.0，明确指定f64类型
        
        // 先计算表头宽度
        for (col_idx, header) in IO_TABLE_HEADERS.iter().enumerate() {
            // 根据字符长度估算宽度，中文字符占用更多宽度
            let estimated_width = header.chars().fold(0.0f64, |acc, c| {
                if c.is_ascii() {
                    acc + 1.0
                } else {
                    // 中文字符宽度是ASCII字符的大约2倍
                    acc + 2.0
                }
            });
            // 额外添加一些padding，并设置最小宽度
            let width = f64::max(estimated_width + 2.0, 10.0);
            column_widths[col_idx] = f64::max(column_widths[col_idx], width);
        }
        
        // 再计算每行数据的宽度
        for row_idx in 0..io_table.rows.len() {
            let row = row_idx as u32 + 2; // 从第2行开始（跳过表头）
            
            for (col_idx, _) in IO_TABLE_HEADERS.iter().enumerate() {
                let col = col_idx as u32 + 1;
                
                // 获取单元格值的字符串表示
                if let Some(cell) = worksheet.get_cell((col, row)) {
                    let cell_value = cell.get_value().to_string();
                    // 检查是否是公式，如果是则估算较小宽度
                    if cell_value.starts_with("=") {
                        // 公式的显示内容通常比公式本身短
                        continue;
                    }
                    
                    // 根据字符长度估算宽度
                    let estimated_width = cell_value.chars().fold(0.0f64, |acc, c| {
                        if c.is_ascii() {
                            acc + 1.0
                        } else {
                            // 中文字符宽度是ASCII字符的大约2倍
                            acc + 2.0
                        }
                    });
                    
                    // 额外添加一些padding，并设置最小宽度
                    let width = f64::max(estimated_width + 2.0, 10.0);
                    column_widths[col_idx] = f64::max(column_widths[col_idx], width);
                }
            }
        }
        
        // 应用计算出的列宽，并设置最大宽度限制
        for col_idx in 0..IO_TABLE_HEADERS.len() {
            let col_letter = get_column_letter(col_idx as u32 + 1);
            // 限制最大宽度为50，避免过宽
            let width = f64::min(column_widths[col_idx], 50.0);
            worksheet.get_column_dimension_mut(&col_letter).set_width(width);
        }
    }

    /// 向数据处理服务发送场站设备数据
    pub fn process_station_data(equipment_list: &[EquipmentData]) -> Result<HashMap<String, ChannelTotal>, String> {
        // 计算各类型通道总数
        let channel_totals = Self::calculate_channels(equipment_list);
        
        // 返回统计结果
        Ok(channel_totals)
    }

    /// 生成并导出IO点表
    pub fn generate_io_table(
        equipment_list: &[EquipmentData], 
        station_name: &str
    ) -> Result<String, String> {
        // 不再使用临时路径，而是让用户选择输出路径
        // 用默认文件名构造建议的文件名
        let file_name = format!("{}_IO点表.xlsx", station_name);
        
        // 导出Excel - 注意检查行列是否正确
        // 创建一个临时文件以便用户选择位置前生成文件
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(&file_name);
        
        match Self::export_to_excel(equipment_list, &temp_path, station_name) {
            Ok(_) => Ok(temp_path.to_string_lossy().to_string()),
            Err(e) => Err(format!("生成IO点表失败: {}", e))
        }
    }
}

/// 辅助函数：获取列字母（A, B, C...AA, AB...）
fn get_column_letter(col_num: u32) -> String {
    let mut temp = col_num;
    let mut col_str = String::new();
    
    while temp > 0 {
        let modulo = (temp - 1) % 26;
        col_str.insert(0, (65 + modulo) as u8 as char);
        temp = (temp - modulo) / 26;
    }
    
    col_str
}
