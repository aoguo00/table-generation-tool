import { Injectable } from '@angular/core';
import { from, Observable } from 'rxjs';
import { ProjectInfo } from '../models/project.model';
import { EquipmentItem } from '../models/equipment.model';

/**
 * API服务
 * 负责所有与后端的通信
 */
@Injectable({
  providedIn: 'root'
})
export class ApiService {
  constructor() { }

  /**
   * 根据项目编号查询项目数据
   * @param projectNumber 项目编号
   * @returns 项目数据Observable
   */
  queryProjectByNumber(projectNumber: string): Observable<{ projects: ProjectInfo[] }> {
    return from(this.invokeCommand('query_jdy_data_by_project_number', { projectNumber }));
  }

  /**
   * 根据场站名称查询设备清单
   * @param stationName 场站名称
   * @returns 设备清单Observable
   */
  queryEquipmentByStation(stationName: string): Observable<{ equipment_list: EquipmentItem[] }> {
    return from(this.invokeCommand('query_equipment_by_station', { stationName }));
  }

  /**
   * 生成IO点表
   * @param equipmentData 设备数据
   * @param stationName 场站名称
   * @returns 生成的文件路径Observable
   */
  generateIOPointTable(equipmentData: any[], stationName: string): Observable<string> {
    return from(this.invokeCommand('generate_io_point_table', { 
      equipmentData, 
      stationName,
      window: this.getCurrentWindow()
    }));
  }

  /**
   * 打开文件
   * @param path 文件路径
   * @returns 操作结果Observable
   */
  openFile(path: string): Observable<void> {
    return from(this.invokeCommand('open_file', { path }));
  }

  /**
   * 处理场站数据
   * @param equipmentData 设备数据
   * @returns 处理结果Observable
   */
  processStationData(equipmentData: any[]): Observable<any> {
    return from(this.invokeCommand('process_station_data', { equipmentData }));
  }

  /**
   * 调用Tauri命令
   * @param command 命令名称
   * @param args 命令参数
   * @returns Promise<any>
   */
  private async invokeCommand(command: string, args: any): Promise<any> {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke(command, args);
    } catch (error) {
      console.error(`调用命令 ${command} 失败:`, error);
      throw error;
    }
  }

  /**
   * 获取当前窗口
   * @returns 当前窗口对象
   */
  private async getCurrentWindow(): Promise<any> {
    try {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      return await getCurrentWindow();
    } catch (error) {
      console.error('获取当前窗口失败:', error);
      throw error;
    }
  }
}
