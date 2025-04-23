use serde::{Deserialize, Serialize};

/// IO点表数据模型
/// 用于表示IO点表Excel的每一行数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOTableRow {
    /// 序号
    pub index: Option<String>,
    /// 模块名称
    pub module_name: Option<String>,
    /// 模块类型
    pub module_type: Option<String>,
    /// 供电类型（有源/无源）
    pub power_supply_type: Option<String>,
    /// 线制
    pub wire_system: Option<String>,
    /// 通道位号
    pub channel_tag: Option<String>,
    /// 位号
    pub tag: Option<String>,
    /// 场站名
    pub station_name: Option<String>,
    /// 变量名称（HMI）
    pub variable_name_hmi: Option<String>,
    /// 变量描述
    pub variable_description: Option<String>,
    /// 数据类型
    pub data_type: Option<String>,
    /// 读写属性
    pub read_write_property: Option<String>,
    /// 保存历史
    pub save_history: Option<String>,
    /// 掉电保护
    pub power_off_protection: Option<String>,
    /// 量程低限
    pub range_lower_limit: Option<String>,
    /// 量程高限
    pub range_upper_limit: Option<String>,
    /// SLL设定值
    pub sll_value: Option<String>,
    /// SLL设定点位
    pub sll_setpoint: Option<String>,
    /// SLL设定点位_PLC地址
    pub sll_setpoint_plc_address: Option<String>,
    /// SLL设定点位_通讯地址
    pub sll_setpoint_comm_address: Option<String>,
    /// SL设定值
    pub sl_value: Option<String>,
    /// SL设定点位
    pub sl_setpoint: Option<String>,
    /// SL设定点位_PLC地址
    pub sl_setpoint_plc_address: Option<String>,
    /// SL设定点位_通讯地址
    pub sl_setpoint_comm_address: Option<String>,
    /// SH设定值
    pub sh_value: Option<String>,
    /// SH设定点位
    pub sh_setpoint: Option<String>,
    /// SH设定点位_PLC地址
    pub sh_setpoint_plc_address: Option<String>,
    /// SH设定点位_通讯地址
    pub sh_setpoint_comm_address: Option<String>,
    /// SHH设定值
    pub shh_value: Option<String>,
    /// SHH设定点位
    pub shh_setpoint: Option<String>,
    /// SHH设定点位_PLC地址
    pub shh_setpoint_plc_address: Option<String>,
    /// SHH设定点位_通讯地址
    pub shh_setpoint_comm_address: Option<String>,
    /// LL报警
    pub ll_alarm: Option<String>,
    /// LL报警_PLC地址
    pub ll_alarm_plc_address: Option<String>,
    /// LL报警_通讯地址
    pub ll_alarm_comm_address: Option<String>,
    /// L报警
    pub l_alarm: Option<String>,
    /// L报警_PLC地址
    pub l_alarm_plc_address: Option<String>,
    /// L报警_通讯地址
    pub l_alarm_comm_address: Option<String>,
    /// H报警
    pub h_alarm: Option<String>,
    /// H报警_PLC地址
    pub h_alarm_plc_address: Option<String>,
    /// H报警_通讯地址
    pub h_alarm_comm_address: Option<String>,
    /// HH报警
    pub hh_alarm: Option<String>,
    /// HH报警_PLC地址
    pub hh_alarm_plc_address: Option<String>,
    /// HH报警_通讯地址
    pub hh_alarm_comm_address: Option<String>,
    /// 维护值设定
    pub maintenance_value: Option<String>,
    /// 维护值设定点位
    pub maintenance_setpoint: Option<String>,
    /// 维护值设定点位_PLC地址
    pub maintenance_setpoint_plc_address: Option<String>,
    /// 维护值设定点位_通讯地址
    pub maintenance_setpoint_comm_address: Option<String>,
    /// 维护使能开关点位
    pub maintenance_enable_switch: Option<String>,
    /// 维护使能开关点位_PLC地址
    pub maintenance_enable_switch_plc_address: Option<String>,
    /// 维护使能开关点位_通讯地址
    pub maintenance_enable_switch_comm_address: Option<String>,
    /// PLC绝对地址
    pub plc_absolute_address: Option<String>,
    /// 上位机通讯地址
    pub host_comm_address: Option<String>,
}

impl Default for IOTableRow {
    fn default() -> Self {
        Self {
            index: None,
            module_name: None,
            module_type: None,
            power_supply_type: None,
            wire_system: None,
            channel_tag: None,
            tag: None,
            station_name: None,
            variable_name_hmi: None,
            variable_description: None,
            data_type: None,
            read_write_property: None,
            save_history: None,
            power_off_protection: None,
            range_lower_limit: None,
            range_upper_limit: None,
            sll_value: None,
            sll_setpoint: None,
            sll_setpoint_plc_address: None,
            sll_setpoint_comm_address: None,
            sl_value: None,
            sl_setpoint: None,
            sl_setpoint_plc_address: None,
            sl_setpoint_comm_address: None,
            sh_value: None,
            sh_setpoint: None,
            sh_setpoint_plc_address: None,
            sh_setpoint_comm_address: None,
            shh_value: None,
            shh_setpoint: None,
            shh_setpoint_plc_address: None,
            shh_setpoint_comm_address: None,
            ll_alarm: None,
            ll_alarm_plc_address: None,
            ll_alarm_comm_address: None,
            l_alarm: None,
            l_alarm_plc_address: None,
            l_alarm_comm_address: None,
            h_alarm: None,
            h_alarm_plc_address: None,
            h_alarm_comm_address: None,
            hh_alarm: None,
            hh_alarm_plc_address: None,
            hh_alarm_comm_address: None,
            maintenance_value: None,
            maintenance_setpoint: None,
            maintenance_setpoint_plc_address: None,
            maintenance_setpoint_comm_address: None,
            maintenance_enable_switch: None,
            maintenance_enable_switch_plc_address: None,
            maintenance_enable_switch_comm_address: None,
            plc_absolute_address: None,
            host_comm_address: None,
        }
    }
}

/// IO点表
/// 包含多行IO点位数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOTable {
    /// 表名
    pub table_name: String,
    /// 数据行
    pub rows: Vec<IOTableRow>,
}

impl IOTable {
    /// 创建新的IO点表
    pub fn new(table_name: String) -> Self {
        Self {
            table_name,
            rows: Vec::new(),
        }
    }
    
    /// 添加一行数据
    pub fn add_row(&mut self, row: IOTableRow) {
        self.rows.push(row);
    }
    
    /// 获取行数
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// 表头常量，用于Excel导入导出
pub const IO_TABLE_HEADERS: [&str; 53] = [
    "序号", "模块名称", "模块类型", "供电类型（有源/无源）", "线制", "通道位号", "位号", "场站名", 
    "变量名称（HMI）", "变量描述", "数据类型", "读写属性", "保存历史", "掉电保护", 
    "量程低限", "量程高限", "SLL设定值", "SLL设定点位", "SLL设定点位_PLC地址", "SLL设定点位_通讯地址",
    "SL设定值", "SL设定点位", "SL设定点位_PLC地址", "SL设定点位_通讯地址",
    "SH设定值", "SH设定点位", "SH设定点位_PLC地址", "SH设定点位_通讯地址",
    "SHH设定值", "SHH设定点位", "SHH设定点位_PLC地址", "SHH设定点位_通讯地址",
    "LL报警", "LL报警_PLC地址", "LL报警_通讯地址",
    "L报警", "L报警_PLC地址", "L报警_通讯地址",
    "H报警", "H报警_PLC地址", "H报警_通讯地址",
    "HH报警", "HH报警_PLC地址", "HH报警_通讯地址",
    "维护值设定", "维护值设定点位", "维护值设定点位_PLC地址", "维护值设定点位_通讯地址", 
    "维护使能开关点位", "维护使能开关点位_PLC地址", "维护使能开关点位_通讯地址",
    "PLC绝对地址", "上位机通讯地址"
];
