import { Injectable } from '@angular/core';

/**
 * 项目信息接口
 */
export interface ProjectInfo {
  id: string;
  project_name: string;
  project_number: string;
  design_number: string;
  customer_name: string;
  station_name: string;
  checked?: boolean;
}

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

/**
 * 共享数据服务
 * 用于在组件之间共享数据
 */
@Injectable({
  providedIn: 'root'
})
export class SharedDataService {
  // 项目数据
  private projectData: ProjectInfo[] = [];
  // 选中的项目
  private selectedProject: ProjectInfo | null = null;
  // 设备数据
  private equipmentData: EquipmentItem[] = [];
  // 设备表数据
  private deviceTableData: DeviceItem[] = [];
  // 场站号
  private stationNumber: string = '';

  constructor() {
    // 初始化共享数据服务
  }

  /**
   * 设置项目数据
   * @param data 项目数据
   */
  setProjectData(data: ProjectInfo[]): void {
    this.projectData = data;
  }

  /**
   * 获取项目数据
   * @returns 项目数据
   */
  getProjectData(): ProjectInfo[] {
    return this.projectData;
  }

  /**
   * 设置选中的项目
   * @param project 选中的项目
   */
  setSelectedProject(project: ProjectInfo | null): void {
    this.selectedProject = project;
  }

  /**
   * 获取选中的项目
   * @returns 选中的项目
   */
  getSelectedProject(): ProjectInfo | null {
    return this.selectedProject;
  }

  /**
   * 设置设备数据
   * @param data 设备数据
   */
  setEquipmentData(data: EquipmentItem[]): void {
    this.equipmentData = data;
  }

  /**
   * 获取设备数据
   * @returns 设备数据
   */
  getEquipmentData(): EquipmentItem[] {
    return this.equipmentData;
  }

  /**
   * 设置设备表数据
   * @param data 设备表数据
   */
  setDeviceTableData(data: DeviceItem[]): void {
    this.deviceTableData = data;
  }

  /**
   * 获取设备表数据
   * @returns 设备表数据
   */
  getDeviceTableData(): DeviceItem[] {
    return this.deviceTableData;
  }

  /**
   * 设置场站号
   * @param number 场站号
   */
  setStationNumber(number: string): void {
    this.stationNumber = number;
  }

  /**
   * 获取场站号
   * @returns 场站号
   */
  getStationNumber(): string {
    return this.stationNumber;
  }

  /**
   * 清空所有数据
   */
  clearAll(): void {
    this.projectData = [];
    this.selectedProject = null;
    this.equipmentData = [];
    this.deviceTableData = [];
    this.stationNumber = '';
  }
}
