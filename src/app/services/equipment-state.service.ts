import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { EquipmentItem } from '../models/equipment.model';
import { DeviceItem } from '../models/device-table.model';

/**
 * 设备状态服务
 * 负责管理设备相关的状态
 */
@Injectable({
  providedIn: 'root'
})
export class EquipmentStateService {
  // 设备数据
  private equipmentDataSubject = new BehaviorSubject<EquipmentItem[]>([]);
  public equipmentData$: Observable<EquipmentItem[]> = this.equipmentDataSubject.asObservable();

  // 设备表数据
  private deviceTableDataSubject = new BehaviorSubject<DeviceItem[]>([]);
  public deviceTableData$: Observable<DeviceItem[]> = this.deviceTableDataSubject.asObservable();

  constructor() { }

  /**
   * 设置设备数据
   * @param data 设备数据
   */
  setEquipmentData(data: EquipmentItem[]): void {
    this.equipmentDataSubject.next(data);
  }

  /**
   * 获取设备数据
   * @returns 设备数据
   */
  getEquipmentData(): EquipmentItem[] {
    return this.equipmentDataSubject.value;
  }

  /**
   * 设置设备表数据
   * @param data 设备表数据
   */
  setDeviceTableData(data: DeviceItem[]): void {
    this.deviceTableDataSubject.next(data);
  }

  /**
   * 获取设备表数据
   * @returns 设备表数据
   */
  getDeviceTableData(): DeviceItem[] {
    return this.deviceTableDataSubject.value;
  }

  /**
   * 清空所有设备相关数据
   */
  clearAll(): void {
    this.equipmentDataSubject.next([]);
    this.deviceTableDataSubject.next([]);
  }

  /**
   * 将设备清单转换为设备表数据
   * @returns 转换后的设备表数据
   */
  convertEquipmentToDeviceTable(): DeviceItem[] {
    const equipmentData = this.getEquipmentData();
    if (equipmentData && equipmentData.length > 0) {
      // 转换格式
      const deviceData = equipmentData.map((item, index) => ({
        id: index + 1,
        name: item.name,
        tagNumber: `TAG-${item.id}`, // 默认位号
        description: item.tech_param || '', // 使用技术参数作为描述
        quantity: item.quantity
      }));

      // 保存到状态
      this.setDeviceTableData(deviceData);
      return deviceData;
    }
    return [];
  }
}
