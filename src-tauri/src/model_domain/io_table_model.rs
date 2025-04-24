use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// IO点表数据模型
/// 用于表示IO点表Excel的每一行数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOTableRow<'a> {
    /// 序号
    pub index: Option<Cow<'a, str>>,
    /// 模块名称
    pub module_name: Option<Cow<'a, str>>,
    /// 模块类型
    pub module_type: Option<Cow<'a, str>>,
    /// 供电类型（有源/无源）
    pub power_supply_type: Option<Cow<'a, str>>,
    /// 线制
    pub wire_system: Option<Cow<'a, str>>,
    /// 通道位号
    pub channel_tag: Option<Cow<'a, str>>,
    /// 位号
    pub tag: Option<Cow<'a, str>>,
    /// 场站名
    pub station_name: Option<Cow<'a, str>>,
    /// 变量名称（HMI）
    pub variable_name_hmi: Option<Cow<'a, str>>,
    /// 变量描述
    pub variable_description: Option<Cow<'a, str>>,
    /// 数据类型
    pub data_type: Option<Cow<'a, str>>,
    /// 读写属性
    pub read_write_property: Option<Cow<'a, str>>,
    /// 保存历史
    pub save_history: Option<Cow<'a, str>>,
    /// 掉电保护
    pub power_off_protection: Option<Cow<'a, str>>,
    /// 量程低限
    pub range_lower_limit: Option<Cow<'a, str>>,
    /// 量程高限
    pub range_upper_limit: Option<Cow<'a, str>>,
    /// SLL设定值
    pub sll_value: Option<Cow<'a, str>>,
    /// SLL设定点位
    pub sll_setpoint: Option<Cow<'a, str>>,
    /// SLL设定点位_PLC地址
    pub sll_setpoint_plc_address: Option<Cow<'a, str>>,
    /// SLL设定点位_通讯地址
    pub sll_setpoint_comm_address: Option<Cow<'a, str>>,
    /// SL设定值
    pub sl_value: Option<Cow<'a, str>>,
    /// SL设定点位
    pub sl_setpoint: Option<Cow<'a, str>>,
    /// SL设定点位_PLC地址
    pub sl_setpoint_plc_address: Option<Cow<'a, str>>,
    /// SL设定点位_通讯地址
    pub sl_setpoint_comm_address: Option<Cow<'a, str>>,
    /// SH设定值
    pub sh_value: Option<Cow<'a, str>>,
    /// SH设定点位
    pub sh_setpoint: Option<Cow<'a, str>>,
    /// SH设定点位_PLC地址
    pub sh_setpoint_plc_address: Option<Cow<'a, str>>,
    /// SH设定点位_通讯地址
    pub sh_setpoint_comm_address: Option<Cow<'a, str>>,
    /// SHH设定值
    pub shh_value: Option<Cow<'a, str>>,
    /// SHH设定点位
    pub shh_setpoint: Option<Cow<'a, str>>,
    /// SHH设定点位_PLC地址
    pub shh_setpoint_plc_address: Option<Cow<'a, str>>,
    /// SHH设定点位_通讯地址
    pub shh_setpoint_comm_address: Option<Cow<'a, str>>,
    /// LL报警
    pub ll_alarm: Option<Cow<'a, str>>,
    /// LL报警_PLC地址
    pub ll_alarm_plc_address: Option<Cow<'a, str>>,
    /// LL报警_通讯地址
    pub ll_alarm_comm_address: Option<Cow<'a, str>>,
    /// L报警
    pub l_alarm: Option<Cow<'a, str>>,
    /// L报警_PLC地址
    pub l_alarm_plc_address: Option<Cow<'a, str>>,
    /// L报警_通讯地址
    pub l_alarm_comm_address: Option<Cow<'a, str>>,
    /// H报警
    pub h_alarm: Option<Cow<'a, str>>,
    /// H报警_PLC地址
    pub h_alarm_plc_address: Option<Cow<'a, str>>,
    /// H报警_通讯地址
    pub h_alarm_comm_address: Option<Cow<'a, str>>,
    /// HH报警
    pub hh_alarm: Option<Cow<'a, str>>,
    /// HH报警_PLC地址
    pub hh_alarm_plc_address: Option<Cow<'a, str>>,
    /// HH报警_通讯地址
    pub hh_alarm_comm_address: Option<Cow<'a, str>>,
    /// 维护值设定
    pub maintenance_value: Option<Cow<'a, str>>,
    /// 维护值设定点位
    pub maintenance_setpoint: Option<Cow<'a, str>>,
    /// 维护值设定点位_PLC地址
    pub maintenance_setpoint_plc_address: Option<Cow<'a, str>>,
    /// 维护值设定点位_通讯地址
    pub maintenance_setpoint_comm_address: Option<Cow<'a, str>>,
    /// 维护使能开关点位
    pub maintenance_enable_switch: Option<Cow<'a, str>>,
    /// 维护使能开关点位_PLC地址
    pub maintenance_enable_switch_plc_address: Option<Cow<'a, str>>,
    /// 维护使能开关点位_通讯地址
    pub maintenance_enable_switch_comm_address: Option<Cow<'a, str>>,
    /// PLC绝对地址
    pub plc_absolute_address: Option<Cow<'a, str>>,
    /// 上位机通讯地址
    pub host_comm_address: Option<Cow<'a, str>>,
}

impl<'a> IOTableRow<'a> {
    /// 将所有字段按IO_TABLE_HEADERS顺序转换为字符串向量
    pub fn to_vec(&self) -> Vec<Option<String>> {
        let mut result = Vec::with_capacity(IO_TABLE_HEADERS.len());
        
        // 按照表头顺序添加字段值
        for header in IO_TABLE_HEADERS.iter() {
            let value = self.get_field_by_name(header)
                .map(|cow| cow.to_string());
            result.push(value);
        }
        
        result
    }
    
    /// 通过字段名称获取对应的值
    pub fn get_field_by_name(&self, field_name: &str) -> Option<&Cow<'a, str>> {
        match field_name {
            "序号" => self.index.as_ref(),
            "模块名称" => self.module_name.as_ref(),
            "模块类型" => self.module_type.as_ref(),
            "供电类型（有源/无源）" => self.power_supply_type.as_ref(),
            "线制" => self.wire_system.as_ref(),
            "通道位号" => self.channel_tag.as_ref(),
            "位号" => self.tag.as_ref(),
            "场站名" => self.station_name.as_ref(),
            "变量名称（HMI）" => self.variable_name_hmi.as_ref(),
            "变量描述" => self.variable_description.as_ref(),
            "数据类型" => self.data_type.as_ref(),
            "读写属性" => self.read_write_property.as_ref(),
            "保存历史" => self.save_history.as_ref(),
            "掉电保护" => self.power_off_protection.as_ref(),
            "量程低限" => self.range_lower_limit.as_ref(),
            "量程高限" => self.range_upper_limit.as_ref(),
            "SLL设定值" => self.sll_value.as_ref(),
            "SLL设定点位" => self.sll_setpoint.as_ref(),
            "SLL设定点位_PLC地址" => self.sll_setpoint_plc_address.as_ref(),
            "SLL设定点位_通讯地址" => self.sll_setpoint_comm_address.as_ref(),
            "SL设定值" => self.sl_value.as_ref(),
            "SL设定点位" => self.sl_setpoint.as_ref(),
            "SL设定点位_PLC地址" => self.sl_setpoint_plc_address.as_ref(),
            "SL设定点位_通讯地址" => self.sl_setpoint_comm_address.as_ref(),
            "SH设定值" => self.sh_value.as_ref(),
            "SH设定点位" => self.sh_setpoint.as_ref(),
            "SH设定点位_PLC地址" => self.sh_setpoint_plc_address.as_ref(),
            "SH设定点位_通讯地址" => self.sh_setpoint_comm_address.as_ref(),
            "SHH设定值" => self.shh_value.as_ref(),
            "SHH设定点位" => self.shh_setpoint.as_ref(),
            "SHH设定点位_PLC地址" => self.shh_setpoint_plc_address.as_ref(),
            "SHH设定点位_通讯地址" => self.shh_setpoint_comm_address.as_ref(),
            "LL报警" => self.ll_alarm.as_ref(),
            "LL报警_PLC地址" => self.ll_alarm_plc_address.as_ref(),
            "LL报警_通讯地址" => self.ll_alarm_comm_address.as_ref(),
            "L报警" => self.l_alarm.as_ref(),
            "L报警_PLC地址" => self.l_alarm_plc_address.as_ref(),
            "L报警_通讯地址" => self.l_alarm_comm_address.as_ref(),
            "H报警" => self.h_alarm.as_ref(),
            "H报警_PLC地址" => self.h_alarm_plc_address.as_ref(),
            "H报警_通讯地址" => self.h_alarm_comm_address.as_ref(),
            "HH报警" => self.hh_alarm.as_ref(),
            "HH报警_PLC地址" => self.hh_alarm_plc_address.as_ref(),
            "HH报警_通讯地址" => self.hh_alarm_comm_address.as_ref(),
            "维护值设定" => self.maintenance_value.as_ref(),
            "维护值设定点位" => self.maintenance_setpoint.as_ref(),
            "维护值设定点位_PLC地址" => self.maintenance_setpoint_plc_address.as_ref(),
            "维护值设定点位_通讯地址" => self.maintenance_setpoint_comm_address.as_ref(),
            "维护使能开关点位" => self.maintenance_enable_switch.as_ref(),
            "维护使能开关点位_PLC地址" => self.maintenance_enable_switch_plc_address.as_ref(),
            "维护使能开关点位_通讯地址" => self.maintenance_enable_switch_comm_address.as_ref(),
            "PLC绝对地址" => self.plc_absolute_address.as_ref(),
            "上位机通讯地址" => self.host_comm_address.as_ref(),
            _ => None,
        }
    }
}

impl<'a> Default for IOTableRow<'a> {
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
pub struct IOTable<'a> {
    /// 表名
    pub table_name: String,
    /// 数据行
    pub rows: Vec<IOTableRow<'a>>,
}

impl<'a> IOTable<'a> {
    /// 创建新的IO点表
    pub fn new(table_name: String) -> Self {
        Self {
            table_name,
            rows: Vec::new(),
        }
    }
    
    /// 添加一行数据
    pub fn add_row(&mut self, row: IOTableRow<'a>) {
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
