use crate::model_domain::io_table_model::{IOTable, IOTableRow, IO_TABLE_HEADERS};
use std::path::Path;
use umya_spreadsheet::{Worksheet, Style, Border, Color};
use umya_spreadsheet::writer::xlsx::write;
use umya_spreadsheet::XlsxError;
use std::collections::HashMap;
use std::borrow::Cow;
use once_cell::sync::Lazy;
use thiserror::Error;
use serde_json;
use umya_spreadsheet::structs::HorizontalAlignmentValues;
use std::fmt;

// 常量定义，替代魔术数字
/// 模拟量地址起始值（MD）
const REAL_ADDR_START: u32 = 320;
/// 布尔量地址起始值（MX）
const BOOL_ADDR_START: (u32, u32) = (20, 0);
/// 槽位起始值（跳过第一个槽位，用于通信模块）
const START_SLOT: u32 = 2;
/// 每个机架可用的槽位数
const AVAILABLE_SLOTS_PER_RACK: u32 = 10;
/// 布尔量每字节位数
const BOOL_BITS_PER_BYTE: u32 = 8;
/// REAL类型每点位占用字节数
const REAL_BYTES_PER_POINT: u32 = 4;

/// IO错误类型
#[derive(Error, Debug)]
pub enum IoError {
    #[error("IO模块数量超出了可用机架数量，当前机架数 {rack_count}，需要机架数 {required_rack}")]
    SlotOverflow { rack_count: u32, required_rack: u32 },

    #[error("PLC地址解析错误: {0}")]
    AddrParse(String),

    #[error("Excel导出错误: {0}")]
    ExcelExport(#[from] XlsxError),

    #[error("数据查询错误: {0}")]
    DataQuery(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

/// IO通道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IOChannelType {
    AI,
    AO,
    DI,
    DO,
}

impl fmt::Display for IOChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IOChannelType::AI => write!(f, "AI"),
            IOChannelType::AO => write!(f, "AO"),
            IOChannelType::DI => write!(f, "DI"),
            IOChannelType::DO => write!(f, "DO"),
        }
    }
}

/// 数据类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    REAL,
    BOOL,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::REAL => write!(f, "REAL"),
            DataType::BOOL => write!(f, "BOOL"),
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
#[derive(Debug, Clone, Default)]
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

/// REAL类型点位配置（列名, 后缀）
pub static POINT_CONFIGS: Lazy<[(&'static str, &'static str); 10]> = Lazy::new(|| [
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
]);

/// 设备型号与通道的映射
#[derive(Debug, Clone)]
pub struct ModelChannelMapping {
    pub model_key: String,
    pub channel_type: IOChannelType,
    pub channels: u32,
    pub data_type: DataType,
}

/// 静态存储的设备型号映射表
pub static MODEL_MAPPINGS: Lazy<Vec<ModelChannelMapping>> = Lazy::new(|| {
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
});

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
    pub fn get_model_channel_mapping() -> &'static [ModelChannelMapping] {
        &MODEL_MAPPINGS
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
    ///
    /// 计算方法:
    /// - REAL类型: (MD地址数字部分/2) + 43001
    /// - BOOL类型: (MX地址主数字*8) + 位数字 + 3001
    pub fn calculate_modbus_address(plc_address: &str, data_type: DataType) -> Result<u32, IoError> {
        match data_type {
            DataType::REAL => Self::calculate_real_modbus_address(plc_address),
            DataType::BOOL => Self::calculate_bool_modbus_address(plc_address),
        }
    }

    /// 计算REAL类型的Modbus地址
    fn calculate_real_modbus_address(plc_address: &str) -> Result<u32, IoError> {
        // 对于REAL类型：=(MID(AE2,4,4)/2)+43001
        // 从%MD100中提取100，然后计算
        if plc_address.len() < 4 {
            return Err(IoError::AddrParse(format!("无效的PLC REAL地址: {}", plc_address)));
        }

        let md_num = plc_address[3..].parse::<u32>()
            .map_err(|e| IoError::AddrParse(format!("REAL地址解析错误: {}", e)))?;

        Ok((md_num / 2) + 43001)
    }

    /// 计算BOOL类型的Modbus地址
    fn calculate_bool_modbus_address(plc_address: &str) -> Result<u32, IoError> {
        // 对于BOOL类型：=(MID(AE3,4,2)*8)+RIGHT(AE3,1)+3001
        // 从%MX20.0中提取20和0，然后计算
        if plc_address.len() < 4 {
            return Err(IoError::AddrParse(format!("无效的PLC BOOL地址: {}", plc_address)));
        }

        let parts: Vec<&str> = plc_address[3..].split('.').collect();
        if parts.is_empty() {
            return Err(IoError::AddrParse(format!("BOOL地址格式错误: {}", plc_address)));
        }

        let mx_num = parts[0].parse::<u32>()
            .map_err(|e| IoError::AddrParse(format!("BOOL地址MX部分解析错误: {}", e)))?;

        let bit_num = if parts.len() > 1 {
            parts[1].parse::<u32>()
                .map_err(|e| IoError::AddrParse(format!("BOOL地址bit部分解析错误: {}", e)))?
        } else { 0 };

        Ok((mx_num * BOOL_BITS_PER_BYTE) + bit_num + 3001)
    }

    /// 生成PLC地址
    fn generate_plc_address(data_type: DataType, counter: &mut u32, bool_counter: &mut (u32, u32)) -> String {
        match data_type {
            DataType::REAL => {
                // REAL类型使用%MD地址
                let addr = format!("%MD{}", *counter);
                *counter += REAL_BYTES_PER_POINT;
                addr
            },
            DataType::BOOL => {
                // BOOL类型使用%MX地址
                let addr = format!("%MX{}.{}", bool_counter.0, bool_counter.1);

                // 更新BOOL地址计数器
                bool_counter.1 += 1;
                if bool_counter.1 > 7 {
                    bool_counter.0 += 1;
                    bool_counter.1 = 0;
                }

                addr
            }
        }
    }

    /// 生成PLC和通信地址
    fn generate_addresses(
        data_type: DataType,
        real_counter: &mut u32,
        bool_counter: &mut (u32, u32)
    ) -> Result<(String, u32), IoError> {
        let plc_addr = IOExcelService::generate_plc_address(data_type, real_counter, bool_counter);
        let modbus = IOExcelService::calculate_modbus_address(&plc_addr, data_type)?;
        Ok((plc_addr, modbus))
    }

    /// 将IO通道数据导出到Excel
    pub fn export_to_excel(
        equipment_list: &[EquipmentData],
        output_path: &Path,
        station_name: &str
    ) -> Result<(), IoError> {
        // 创建新的电子表格
        let mut spreadsheet = umya_spreadsheet::new_file();
        let worksheet = spreadsheet.get_active_sheet_mut();

        // 准备IO点表数据
        let io_points = Self::prepare_io_points(equipment_list)?;

        // 创建IO表
        let mut io_table = IOTable::new(format!("{}_IO表", station_name));
        for point in io_points {
            io_table.add_row(point);
        }

        // 写入表头
        Self::write_headers(worksheet);

        // 写入数据、设置公式和占位符
        Self::write_data_and_formulas(worksheet, &io_table)?;

        // 调整列宽 - 自动适应内容
        Self::adjust_column_widths(worksheet, &io_table);

        // 保存Excel
        write(&spreadsheet, output_path)?;

        Ok(())
    }

    /// 更新机架和槽位，检查是否超出可用机架数
    fn update_rack_and_slot(
        current_rack: &mut u32,
        current_slot: &mut u32,
        rack_count: u32
    ) -> Result<(), IoError> {
        // 检查是否需要切换到下一个机架
        if *current_slot > AVAILABLE_SLOTS_PER_RACK + 1 {
            *current_rack += 1;
            *current_slot = START_SLOT; // 重置为起始槽位（通常为2）

            // 检查是否超出机架数量
            if *current_rack > rack_count {
                return Err(IoError::SlotOverflow {
                    rack_count,
                    required_rack: *current_rack
                });
            }
        }

        Ok(())
    }

    /// 准备IO点表数据
    fn prepare_io_points(equipment_list: &[EquipmentData]) -> Result<Vec<IOTableRow<'static>>, IoError> {
        // 预估总通道数量，避免频繁扩容
        let estimated_channels = equipment_list.iter()
            .map(|e| e.quantity as usize * 16) // 假设每个设备最多16通道
            .sum();
        let mut io_points = Vec::with_capacity(estimated_channels);

        // 通道计数器（用于生成通道位号）
        let mut module_counters = HashMap::new();
        module_counters.insert(IOChannelType::AI, 1);
        module_counters.insert(IOChannelType::AO, 1);
        module_counters.insert(IOChannelType::DI, 1);
        module_counters.insert(IOChannelType::DO, 1);

        // 序号计数器
        let mut index_counter = 1;

        // PLC地址计数器
        let mut real_address_counter = REAL_ADDR_START; // %MD320开始
        let mut bool_address_counter = BOOL_ADDR_START; // %MX20.0开始，范围是20-300

        // 机架信息
        let rack_count = Self::get_rack_count(equipment_list);

        // 当前槽位跟踪
        let mut current_rack = 1;
        let mut current_slot = START_SLOT; // 从指定起始槽位开始，通常是2

        // 获取设备型号映射
        let model_channel_mapping = Self::get_model_channel_mapping();

        // 按照IO类型对设备进行分类
        let io_equipment_groups = Self::group_equipment_by_io_type(equipment_list, model_channel_mapping);

        // 按照AI/AO/DI/DO的顺序遍历处理设备
        for io_type in &[IOChannelType::AI, IOChannelType::AO, IOChannelType::DI, IOChannelType::DO] {
            if let Some(equipment_group) = io_equipment_groups.get(io_type) {
                for equipment in equipment_group {
                    let spec_model = &equipment.spec_model;
                    // 获取该设备的通道信息
                    let mut channel_info = None;

                    for model in model_channel_mapping {
                        if spec_model.contains(&model.model_key) {
                            channel_info = Some((
                                model.channel_type,
                                model.channels,
                                model.data_type
                            ));
                            break;
                        }
                    }

                    if let Some((io_type_val, channels, data_type)) = channel_info {
                        let quantity = equipment.quantity;
                        let equipment_name = &equipment.equipment_name;
                        let station_name = &equipment.station_name;

                        // 为每个设备的每个通道创建单独的点表条目
                        for _ in 0..quantity {
                            // 获取当前模块号
                            let _module_num = *module_counters.get(&io_type_val).unwrap_or(&1);

                            // 更新机架和槽位
                            Self::update_rack_and_slot(&mut current_rack, &mut current_slot, rack_count)?;

                            // 为该模块的每个通道创建条目
                            for ch in 0..channels {
                                // 生成新的通道位号格式（例如：1_1_AO_0）
                                let channel_code = format!("{}_{}_{}_{}", current_rack, current_slot, io_type_val, ch);

                                // 生成PLC绝对地址和更新地址计数器
                                let (plc_address, modbus_address) = Self::generate_addresses(
                                    data_type,
                                    &mut real_address_counter,
                                    &mut bool_address_counter
                                )?;

                                // 创建点表数据
                                let point_data = Self::create_io_point(
                                    index_counter,
                                    equipment_name,
                                    io_type_val,
                                    channel_code,
                                    station_name,
                                    data_type,
                                    plc_address,
                                    modbus_address
                                );

                                // 添加到点表列表
                                io_points.push(point_data);
                                index_counter += 1;
                            }

                            // 每个设备模块增加模块计数器和槽位计数器
                            if let Some(counter) = module_counters.get_mut(&io_type_val) {
                                *counter += 1;
                            }
                            current_slot += 1;
                        }
                    }
                }
            }
        }

        Ok(io_points)
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

    /// 按IO类型分组设备，优化为一次遍历完成分组
    fn group_equipment_by_io_type<'a>(
        equipment_list: &'a [EquipmentData],
        model_mapping: &[ModelChannelMapping]
    ) -> HashMap<IOChannelType, Vec<&'a EquipmentData>> {
        let mut groups = HashMap::new();
        // 初始化所有IO类型的分组
        groups.insert(IOChannelType::AI, Vec::new());
        groups.insert(IOChannelType::AO, Vec::new());
        groups.insert(IOChannelType::DI, Vec::new());
        groups.insert(IOChannelType::DO, Vec::new());

        // 创建设备型号到通道类型的映射，避免重复查找
        let mut model_to_channel_type = HashMap::new();
        for model in model_mapping {
            model_to_channel_type.insert(model.model_key.as_str(), model.channel_type);
        }

        // 遍历设备列表进行分类
        for equipment in equipment_list {
            let spec_model = &equipment.spec_model;

            // 查找匹配的型号并分组
            for (model_key, channel_type) in &model_to_channel_type {
                if spec_model.contains(*model_key) {
                    if let Some(group) = groups.get_mut(channel_type) {
                        group.push(equipment);
                    }
                    break;
                }
            }
        }

        groups
    }

    /// 创建IO点表行，减少不必要的克隆
    fn create_io_point(
        index: u32,
        equipment_name: &str,
        io_type: IOChannelType,
        channel_code: String,
        station_name: &str,
        data_type: DataType,
        plc_address: String,
        modbus_address: u32
    ) -> IOTableRow<'static> {
        let mut point = IOTableRow::default();

        // 基本信息
        point.index = Some(Cow::Owned(index.to_string()));
        point.module_name = Some(Cow::Owned(equipment_name.to_owned()));

        // 使用静态字符串优化类型字段
        let io_type_str = match io_type {
            IOChannelType::AI => Cow::Borrowed("AI"),
            IOChannelType::AO => Cow::Borrowed("AO"),
            IOChannelType::DI => Cow::Borrowed("DI"),
            IOChannelType::DO => Cow::Borrowed("DO"),
        };
        point.module_type = Some(io_type_str);

        point.channel_tag = Some(Cow::Owned(channel_code));
        point.station_name = Some(Cow::Owned(station_name.to_owned()));

        // 使用静态字符串优化数据类型字段
        let data_type_str = match data_type {
            DataType::REAL => Cow::Borrowed("REAL"),
            DataType::BOOL => Cow::Borrowed("BOOL"),
        };
        point.data_type = Some(data_type_str);

        // 通用属性使用静态字符串引用
        point.read_write_property = Some(Cow::Borrowed("R/W"));
        point.save_history = Some(Cow::Borrowed("是"));
        point.power_off_protection = Some(Cow::Borrowed("是"));

        // 特定类型属性
        if io_type == IOChannelType::AO {
            point.power_supply_type = Some(Cow::Borrowed("/"));
            point.wire_system = Some(Cow::Borrowed("/"));
        }

        // 数据类型相关设置
        match data_type {
            DataType::REAL => {
                // REAL类型需要设置量程
                point.range_lower_limit = Some(Cow::Owned("".to_owned()));
                point.range_upper_limit = Some(Cow::Owned("".to_owned()));
                point.sll_value = Some(Cow::Owned("".to_owned()));
                point.sl_value = Some(Cow::Owned("".to_owned()));
                point.sh_value = Some(Cow::Owned("".to_owned()));
                point.shh_value = Some(Cow::Owned("".to_owned()));
            },
            DataType::BOOL => {
                // BOOL类型使用"/"表示不适用
                point.range_lower_limit = Some(Cow::Borrowed("/"));
                point.range_upper_limit = Some(Cow::Borrowed("/"));
                point.sll_value = Some(Cow::Borrowed("/"));
                point.sl_value = Some(Cow::Borrowed("/"));
                point.sh_value = Some(Cow::Borrowed("/"));
                point.shh_value = Some(Cow::Borrowed("/"));
            }
        }

        // 地址信息
        point.plc_absolute_address = Some(Cow::Owned(plc_address));
        point.host_comm_address = Some(Cow::Owned(modbus_address.to_string()));

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

            // 应用通用单元格样式
            let highlight = HIGHLIGHT_FIELDS.contains(header);
            Self::apply_common_cell_style(style, highlight);
        }
    }

    /// 写入数据、设置公式和占位符
    fn write_data_and_formulas(worksheet: &mut Worksheet, io_table: &IOTable) -> Result<(), IoError> {
        // PLC地址计数器(统一使用，确保不重复)
        let mut real_address_counter = REAL_ADDR_START; // %MD320开始
        let mut bool_address_counter = BOOL_ADDR_START; // %MX20.0开始，范围是20-300

        for (row_idx, row_data) in io_table.rows.iter().enumerate() {
            let row = row_idx as u32 + 2; // 从第2行开始（跳过表头）

            // 获取数据类型和模块类型
            let data_type = row_data.data_type.as_ref().map(|c| c.as_ref()).unwrap_or("BOOL");
            let module_type = row_data.module_type.as_ref().map(|c| c.as_ref()).unwrap_or("");
            let is_real_type = data_type == "REAL";
            let is_analog_module = matches!(module_type, "AI" | "AO");

            // 写入单元格数据（不包括地址，地址将统一分配）
            Self::write_row_data(worksheet, row, row_data)?;

            // 设置Excel公式或占位符
            if is_real_type {
                Self::set_formulas_for_real_type(worksheet, row)?;
            } else {
                Self::set_placeholders_for_bool_type(worksheet, row);
            }

            // 设置维护值设定为"/"
            if let Some(maint_val_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == "维护值设定") {
                worksheet.get_cell_mut((maint_val_pos as u32 + 1, row)).set_value("/".to_string());
            }

            // 从左往右收集所有需要分配地址的列
            // 格式：(列名, 列索引, 是否REAL类型地址)
            let mut address_columns = Vec::new();

            // 获取地址列的顺序
            for (col_idx, header) in IO_TABLE_HEADERS.iter().enumerate() {
                if header.ends_with("_PLC地址") || *header == "PLC绝对地址" {
                    // 判断地址类型（REAL或BOOL）
                    let is_real_addr = if *header == "PLC绝对地址" {
                        // 主地址使用行的数据类型
                        is_real_type
                    } else if header.starts_with("SLL") || header.starts_with("SL") ||
                        header.starts_with("SH") || header.starts_with("SHH") ||
                        header.starts_with("维护值设定点位") {
                        // 这些是REAL类型地址
                        true
                    } else {
                        // 其他都是BOOL类型地址
                        false
                    };

                    address_columns.push((header, col_idx, is_real_addr));
                }
            }

            // 按照列索引排序，确保从左到右处理
            address_columns.sort_by_key(|(_header, col_idx, _is_real)| *col_idx);

            // 对于非模拟量模块，所有额外点位地址都设为"/"
            if !is_analog_module {
                for (header, col_idx, _) in &address_columns {
                    // 只有主PLC绝对地址保留实际值，其他都设置为"/"
                    if **header != "PLC绝对地址" {
                        worksheet.get_cell_mut((*col_idx as u32 + 1, row)).set_value("/".to_string());

                        // 同时设置对应的通讯地址为"/"
                        let comm_header = header.replace("_PLC地址", "_通讯地址");
                        if let Some(comm_idx) = IO_TABLE_HEADERS.iter().position(|&h| h == comm_header) {
                            worksheet.get_cell_mut((comm_idx as u32 + 1, row)).set_value("/".to_string());
                        }
                    }
                }

                // 非模拟量模块只处理主PLC绝对地址
                if let Some(plc_addr_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == "PLC绝对地址") {
                    let main_addr = if is_real_type {
                        // REAL类型使用%MD地址
                        let addr = format!("%MD{}", real_address_counter);
                        real_address_counter += 4; // REAL类型每个点位加4
                        addr
                    } else {
                        // BOOL类型使用%MX地址
                        let addr = format!("%MX{}.{}", bool_address_counter.0, bool_address_counter.1);

                        // 更新BOOL地址计数器
                        bool_address_counter.1 += 1;
                        if bool_address_counter.1 > 7 {
                            bool_address_counter.0 += 1;
                            bool_address_counter.1 = 0;
                        }

                        addr
                    };

                    // 设置PLC绝对地址
                    worksheet.get_cell_mut((plc_addr_pos as u32 + 1, row)).set_value(main_addr.clone());

                    // 设置上位机通讯地址
                    if let Some(host_addr_pos) = IO_TABLE_HEADERS.iter().position(|&h| h == "上位机通讯地址") {
                        let data_type = if is_real_type { DataType::REAL } else { DataType::BOOL };
                        let modbus = Self::calculate_modbus_address(&main_addr, data_type)?;
                        worksheet.get_cell_mut((host_addr_pos as u32 + 1, row)).set_value(modbus.to_string());
                    }
                }

                continue;  // 非模拟量模块处理完毕，跳过后续代码
            }

            // 从左往右依次分配地址（对于模拟量模块）
            for (header, col_idx, is_real) in &address_columns {
                // 分配PLC地址
                let plc_addr = if *is_real {
                    // REAL类型使用%MD地址
                    let addr = format!("%MD{}", real_address_counter);
                    real_address_counter += 4; // REAL类型每个点位加4
                    addr
                } else {
                    // BOOL类型使用%MX地址
                    let addr = format!("%MX{}.{}", bool_address_counter.0, bool_address_counter.1);

                    // 更新BOOL地址计数器
                    bool_address_counter.1 += 1;
                    if bool_address_counter.1 > 7 {
                        bool_address_counter.0 += 1;
                        bool_address_counter.1 = 0;
                    }

                    addr
                };

                // 设置PLC地址
                worksheet.get_cell_mut((*col_idx as u32 + 1, row)).set_value(plc_addr.clone());

                // 设置对应的通讯地址
                let comm_header = if **header == "PLC绝对地址" {
                    "上位机通讯地址".to_string()
                } else {
                    header.replace("_PLC地址", "_通讯地址")
                };

                if let Some(comm_idx) = IO_TABLE_HEADERS.iter().position(|&h| h == comm_header) {
                    let data_type = if *is_real { DataType::REAL } else { DataType::BOOL };
                    let modbus = Self::calculate_modbus_address(&plc_addr, data_type)?;
                    worksheet.get_cell_mut((comm_idx as u32 + 1, row)).set_value(modbus.to_string());
                }
            }
        }

        Ok(())
    }

    /// 写入单行数据
    fn write_row_data(worksheet: &mut Worksheet, row: u32, row_data: &IOTableRow) -> Result<(), IoError> {
        // 按照表头顺序写入每个字段
        for (col_idx, header) in IO_TABLE_HEADERS.iter().enumerate() {
            let col = col_idx as u32 + 1;

            // 通过IOTableRow的get_field_by_name方法获取字段值
            if let Some(cow_value) = row_data.get_field_by_name(header) {
                worksheet.get_cell_mut((col, row)).set_value(cow_value.to_string());
            }

            // 获取数据类型，用于判断是否需要高亮
            let data_type = row_data.data_type.as_ref().map(|c| c.as_ref()).unwrap_or("BOOL");

            // 获取字段值用于判断高亮条件
            let field_value = row_data.get_field_by_name(header)
                .map(|cow| cow.as_ref())
                .unwrap_or("");

            // 确定是否需要高亮：
            // 1. 字段在需要高亮的列表中
            // 2. 如果是BOOL类型，且字段名包含"量程"，则不高亮
            // 3. 字段值不等于"/"（包括空值和其他值都会高亮）
            let should_highlight = HIGHLIGHT_FIELDS.contains(header) &&
                !(data_type == "BOOL" && header.contains("量程")) &&
                field_value != "/";

            // 设置样式
            let style = worksheet.get_style_mut((col, row));
            Self::apply_common_cell_style(style, should_highlight);
        }

        Ok(())
    }

    /// 应用通用单元格样式
    fn apply_common_cell_style(style: &mut Style, highlight: bool) {
        // 设置边框
        style.get_borders_mut().get_bottom_mut().set_border_style(Border::BORDER_THIN);
        style.get_borders_mut().get_top_mut().set_border_style(Border::BORDER_THIN);
        style.get_borders_mut().get_left_mut().set_border_style(Border::BORDER_THIN);
        style.get_borders_mut().get_right_mut().set_border_style(Border::BORDER_THIN);

        // 设置水平对齐方式为左对齐
        style.get_alignment_mut().set_horizontal(HorizontalAlignmentValues::Left);

        // 如果需要高亮，设置背景色
        if highlight {
            style.set_background_color(Color::COLOR_YELLOW.to_string());
        }
    }

    /// 设置REAL类型的公式
    fn set_formulas_for_real_type(worksheet: &mut Worksheet, row: u32) -> Result<(), IoError> {
        // 获取HMI变量名所在的列索引
        let hmi_col_idx = IO_TABLE_HEADERS.iter()
            .position(|&h| h == "变量名称（HMI）")
            .ok_or_else(|| IoError::DataQuery("HMI变量列不存在".to_string()))? as u32 + 1;

        // 使用迭代器简化公式设置，使用静态点位配置
        for (column_name, suffix) in POINT_CONFIGS.iter() {
            if let Some(pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *column_name) {
                let formula = format!("=IF(ISBLANK({}{}),\"{}\",{}{}&\"{}\")",
                                      get_column_letter(hmi_col_idx), row, suffix,
                                      get_column_letter(hmi_col_idx), row, suffix);

                worksheet.get_cell_mut((pos as u32 + 1, row)).set_formula(formula);
            }
        }

        Ok(())
    }

    /// 设置BOOL类型的占位符
    fn set_placeholders_for_bool_type(worksheet: &mut Worksheet, row: u32) {
        // 需要设置"/"的列
        let placeholder_columns = [
            "SLL设定点位", "SL设定点位", "SH设定点位", "SHH设定点位",
            "LL报警", "L报警", "H报警", "HH报警",
            "维护值设定点位", "维护使能开关点位", "维护值设定",
        ];

        for column_name in &placeholder_columns {
            if let Some(pos) = IO_TABLE_HEADERS.iter().position(|&h| h == *column_name) {
                worksheet.get_cell_mut((pos as u32 + 1, row)).set_value("/".to_string());
            }
        }
    }

    /// 自动调整列宽
    fn adjust_column_widths(worksheet: &mut Worksheet, io_table: &IOTable) {
        let mut column_widths = vec![15.0f64; IO_TABLE_HEADERS.len()]; // 默认宽度

        // 计算表头文本宽度
        Self::calculate_header_widths(&mut column_widths);

        // 计算数据行文本宽度
        for (row_idx, _) in io_table.rows.iter().enumerate() {
            let row = row_idx as u32 + 2; // 从第2行开始（跳过表头）
            Self::calculate_row_widths(worksheet, row, &mut column_widths);
        }

        // 应用列宽设置
        for col_idx in 0..IO_TABLE_HEADERS.len() {
            let col_letter = get_column_letter(col_idx as u32 + 1);
            // 限制最大宽度为50，避免过宽
            let width = f64::min(column_widths[col_idx], 50.0);
            worksheet.get_column_dimension_mut(&col_letter).set_width(width);
        }
    }

    /// 计算表头文本宽度
    fn calculate_header_widths(column_widths: &mut [f64]) {
        for (col_idx, header) in IO_TABLE_HEADERS.iter().enumerate() {
            // 根据字符长度估算宽度，中文字符占用更多宽度
            let estimated_width = Self::estimate_text_width(header);
            // 额外添加一些padding，并设置最小宽度
            let width = f64::max(estimated_width + 2.0, 10.0);
            column_widths[col_idx] = f64::max(column_widths[col_idx], width);
        }
    }

    /// 计算单行数据文本宽度
    fn calculate_row_widths(worksheet: &mut Worksheet, row: u32, column_widths: &mut [f64]) {
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
                let estimated_width = Self::estimate_text_width(&cell_value);

                // 额外添加一些padding，并设置最小宽度
                let width = f64::max(estimated_width + 2.0, 10.0);
                column_widths[col_idx] = f64::max(column_widths[col_idx], width);
            }
        }
    }

    /// 估算文本宽度
    fn estimate_text_width(text: &str) -> f64 {
        text.chars().fold(0.0f64, |acc, c| {
            if c.is_ascii() {
                acc + 1.0
            } else {
                // 中文字符宽度是ASCII字符的大约2倍
                acc + 2.0
            }
        })
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
            Err(e) => Err(format!("生成IO点表失败: {:?}", e))
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_real_modbus_address() {
        // 测试REAL类型地址计算
        let result = IOExcelService::calculate_modbus_address("%MD320", DataType::REAL);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 43160); // (320/2) + 43001
        
        let result = IOExcelService::calculate_modbus_address("%MD400", DataType::REAL);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 43200); // (400/2) + 43001
    }
    
    #[test]
    fn test_calculate_bool_modbus_address() {
        // 测试BOOL类型地址计算
        let result = IOExcelService::calculate_modbus_address("%MX20.0", DataType::BOOL);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3161); // (20*8) + 0 + 3001
        
        let result = IOExcelService::calculate_modbus_address("%MX20.7", DataType::BOOL);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3168); // (20*8) + 7 + 3001
        
        let result = IOExcelService::calculate_modbus_address("%MX21.0", DataType::BOOL);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3169); // (21*8) + 0 + 3001
    }
    
    #[test]
    fn test_invalid_address_format() {
        // 测试无效地址格式
        let result = IOExcelService::calculate_modbus_address("MD320", DataType::REAL);
        assert!(result.is_err());
        
        let result = IOExcelService::calculate_modbus_address("%MX20", DataType::BOOL);
        assert!(result.is_err());
        
        let result = IOExcelService::calculate_modbus_address("%MX.0", DataType::BOOL);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_get_column_letter() {
        assert_eq!(get_column_letter(1), "A");
        assert_eq!(get_column_letter(26), "Z");
        assert_eq!(get_column_letter(27), "AA");
        assert_eq!(get_column_letter(52), "AZ");
        assert_eq!(get_column_letter(53), "BA");
    }
}
