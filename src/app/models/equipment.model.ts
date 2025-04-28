/**
 * 设备信息接口
 */
export interface EquipmentItem {
  id: string;
  name: string;
  brand: string;
  model: string;
  tech_param: string;
  quantity: number;
  unit: string;
  external_param: string;
}

/**
 * 设备查询响应
 */
export interface EquipmentQueryResponse {
  equipment_list: EquipmentItem[];
}
