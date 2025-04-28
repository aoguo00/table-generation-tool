import { Injectable } from '@angular/core';
import { Observable, catchError, from, map, of, switchMap, tap } from 'rxjs';
import { ApiService } from './api.service';
import { ProjectStateService } from './project-state.service';
import { EquipmentStateService } from './equipment-state.service';
import { NzMessageService } from 'ng-zorro-antd/message';

/**
 * IO点表服务
 * 负责处理IO点表生成相关功能
 */
@Injectable({
  providedIn: 'root'
})
export class IoTableService {
  constructor(
    private apiService: ApiService,
    private projectStateService: ProjectStateService,
    private equipmentStateService: EquipmentStateService,
    private message: NzMessageService
  ) { }

  /**
   * 生成IO点表
   * @returns 生成的文件路径Observable
   */
  generateIOPointTable(): Observable<string> {
    const selectedProject = this.projectStateService.getSelectedProject();
    if (!selectedProject) {
      this.message.warning('请先选择一个项目');
      return of('');
    }

    // 准备设备数据
    const equipmentData = this.prepareEquipmentData();
    
    // 调用API生成点表
    return this.apiService.generateIOPointTable(equipmentData, selectedProject.station_name).pipe(
      tap(filePath => console.log('生成的IO点表路径:', filePath)),
      switchMap(filePath => this.handleGeneratedFile(filePath)),
      catchError(error => {
        console.error('生成IO点表失败:', error);
        this.message.error('生成IO点表失败: ' + error);
        return of('');
      })
    );
  }

  /**
   * 准备设备数据
   * @returns 处理后的设备数据
   */
  private prepareEquipmentData(): any[] {
    const selectedProject = this.projectStateService.getSelectedProject();
    const equipmentData = this.equipmentStateService.getEquipmentData();

    if (!equipmentData || equipmentData.length === 0) {
      console.warn('原始设备数据为空，可能导致生成的点表没有数据');
    }

    console.log('准备生成点表的数据源:', equipmentData);

    return equipmentData.map(item => ({
      ...item,
      station_name: selectedProject!.station_name
    }));
  }

  /**
   * 处理生成的文件
   * @param filePath 文件路径
   * @returns 文件路径Observable
   */
  private handleGeneratedFile(filePath: string): Observable<string> {
    if (!filePath) {
      return of('');
    }

    // 自动打开生成的文件
    return this.apiService.openFile(filePath).pipe(
      map(() => {
        this.message.success(`IO点表已生成并打开: ${filePath}`);
        return filePath;
      }),
      catchError(error => {
        console.error('打开文件失败:', error);
        this.message.warning(`IO点表已生成，但打开失败: ${filePath}`);
        return of(filePath);
      })
    );
  }
}
