/**
 * 设备表项目接口
 */
export interface DeviceItem {
  id: number;
  name: string;
  tagNumber: string;
  description: string; // 描述
  quantity: number;
  isEditing?: boolean;
}
