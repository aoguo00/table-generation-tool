import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { DeviceItem } from '../models/device-table.model';
import { EquipmentStateService } from './equipment-state.service';

/**
 * 设备表服务
 * 负责处理设备表相关的业务逻辑
 */
@Injectable({
  providedIn: 'root'
})
export class DeviceTableService {
  // 设备表数据
  private deviceTableDataSubject = new BehaviorSubject<DeviceItem[]>([]);
  public deviceTableData$: Observable<DeviceItem[]> = this.deviceTableDataSubject.asObservable();

  constructor(
    private equipmentStateService: EquipmentStateService
  ) { }

  /**
   * 加载设备表数据
   * 如果有保存的设备表数据，则使用保存的数据
   * 如果没有，则尝试从设备清单中转换
   * @returns 设备表数据
   */
  loadDeviceTableData(): DeviceItem[] {
    // 从状态服务获取设备表数据
    const savedTableData = this.equipmentStateService.getDeviceTableData();
    if (savedTableData && savedTableData.length > 0) {
      this.deviceTableDataSubject.next(savedTableData);
      return savedTableData;
    }

    // 如果没有设备表数据，尝试从设备清单中转换
    const deviceData = this.equipmentStateService.convertEquipmentToDeviceTable();
    if (deviceData && deviceData.length > 0) {
      this.deviceTableDataSubject.next(deviceData);
      return deviceData;
    }

    // 如果没有保存的数据和设备清单数据，则返回空数组
    this.deviceTableDataSubject.next([]);
    return [];
  }

  /**
   * 添加新设备
   * @returns 新添加的设备
   */
  addDevice(): DeviceItem {
    const currentData = this.deviceTableDataSubject.value;
    
    // 创建一个新的设备记录并置为编辑状态
    const newDevice: DeviceItem = {
      id: -currentData.length - 1, // 临时ID为负数
      name: '',
      tagNumber: '',
      description: '', // 初始化描述字段
      quantity: 1,
      isEditing: true
    };

    // 添加到列表开头
    const updatedData = [newDevice, ...currentData];
    this.deviceTableDataSubject.next(updatedData);
    this.equipmentStateService.setDeviceTableData(updatedData);

    return newDevice;
  }

  /**
   * 保存设备编辑
   * @param device 设备项
   * @returns 是否保存成功
   */
  saveDeviceEdit(device: DeviceItem): boolean {
    // 验证位号不为空
    if (!device.tagNumber || device.tagNumber.trim() === '') {
      return false;
    }

    // 验证数量大于0
    if (device.quantity <= 0) {
      return false;
    }

    // 退出编辑状态
    device.isEditing = false;

    // 更新数据
    const currentData = this.deviceTableDataSubject.value;
    const updatedData = [...currentData];
    this.deviceTableDataSubject.next(updatedData);
    this.equipmentStateService.setDeviceTableData(updatedData);

    return true;
  }

  /**
   * 取消设备编辑
   * @param device 设备项
   * @param index 设备项索引
   * @returns 更新后的设备数据
   */
  cancelDeviceEdit(device: DeviceItem, index: number): DeviceItem[] {
    const currentData = this.deviceTableDataSubject.value;
    let updatedData = [...currentData];

    // 如果是新添加的行（ID为负数），则删除该行
    if (device.id < 0) {
      // 删除该行
      updatedData.splice(index, 1);
    } else {
      // 如果是编辑现有行，则只退出编辑状态，不删除数据
      device.isEditing = false;

      // 恢复原始数据
      const savedData = this.equipmentStateService.getDeviceTableData();
      const originalItem = savedData.find(item => item.id === device.id);
      if (originalItem) {
        // 恢复原始数据
        device.name = originalItem.name;
        device.tagNumber = originalItem.tagNumber;
        device.description = originalItem.description;
        device.quantity = originalItem.quantity;
      }
    }

    // 更新数据
    this.deviceTableDataSubject.next(updatedData);
    this.equipmentStateService.setDeviceTableData(updatedData);

    return updatedData;
  }

  /**
   * 删除设备
   * @param index 设备项索引
   * @returns 更新后的设备数据
   */
  deleteDevice(index: number): DeviceItem[] {
    const currentData = this.deviceTableDataSubject.value;
    
    // 删除指定索引的设备
    const updatedData = [...currentData];
    updatedData.splice(index, 1);
    
    // 更新数据
    this.deviceTableDataSubject.next(updatedData);
    this.equipmentStateService.setDeviceTableData(updatedData);

    return updatedData;
  }

  /**
   * 更新设备表数据
   * @param devices 设备数据
   */
  updateDeviceTableData(devices: DeviceItem[]): void {
    this.deviceTableDataSubject.next([...devices]);
    this.equipmentStateService.setDeviceTableData([...devices]);
  }

  /**
   * 获取设备表数据
   * @returns 设备表数据
   */
  getDeviceTableData(): DeviceItem[] {
    return this.deviceTableDataSubject.value;
  }

  /**
   * 准备要发送到后端的数据
   * @returns 处理后的设备表数据
   */
  prepareDeviceTableDataForBackend(): any[] {
    // 检查是否有正在编辑的项
    const editingItem = this.deviceTableDataSubject.value.find(item => item.isEditing);
    if (editingItem) {
      return [];
    }

    // 准备要发送到后端的数据
    return this.deviceTableDataSubject.value.map((item, index) => ({
      id: item.id,
      name: item.name,
      tagNumber: item.tagNumber,
      description: item.description,
      quantity: item.quantity,
      order: index + 1  // 添加序号，表示在表格中的位置
    }));
  }
}
