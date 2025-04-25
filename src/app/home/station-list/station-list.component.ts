import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NzTableModule } from 'ng-zorro-antd/table';
import { NzSpinModule } from 'ng-zorro-antd/spin';
import { NzButtonModule } from 'ng-zorro-antd/button';
import { NzIconModule } from 'ng-zorro-antd/icon';
import { FormsModule } from '@angular/forms';
import { NzMessageModule, NzMessageService } from 'ng-zorro-antd/message';
import { NzModalModule, NzModalService } from 'ng-zorro-antd/modal';
import { DeviceListComponent } from '../device-list/device-list.component';
import { SharedDataService } from '../../shared-data.service';

// 项目信息接口
interface ProjectInfo {
  id: string;
  project_name: string;
  project_number: string;
  design_number: string;
  customer_name: string;
  station_name: string;
}

/**
 * 场站列表组件
 * 使用表格显示场站列表
 */
@Component({
  selector: 'app-station-list',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    NzTableModule,
    NzSpinModule,
    NzButtonModule,
    NzIconModule,
    NzMessageModule,
    NzModalModule,
    DeviceListComponent
  ],
  templateUrl: './station-list.component.html',
  styleUrl: './station-list.component.scss'
})
export class StationListComponent implements OnInit {
  // 项目数据输入
  @Input() projectData: ProjectInfo[] = [];
  // 加载状态输入
  @Input() isLoading: boolean = false;
  // 设备数据输入
  @Input() equipmentData: any[] = [];
  // 选中项目事件输出
  @Output() projectSelected = new EventEmitter<ProjectInfo>();
  // 选中的项目
  selectedProject: ProjectInfo | null = null;

  constructor(
    private message: NzMessageService,
    private modalService: NzModalService,
    private sharedDataService: SharedDataService
  ) {}

  ngOnInit(): void {
    // 从共享服务中恢复选中的项目
    const savedProject = this.sharedDataService.getSelectedProject();
    if (savedProject) {
      this.selectedProject = savedProject;
    }
  }

  /**
   * 查看设备清单并选择场站
   * 点击场站行时调用
   * @param project 选中的项目
   */
  async viewEquipment(project: ProjectInfo): Promise<void> {
    // 更新选中的项目
    this.selectedProject = project;
    
    // 发出选中事件
    this.projectSelected.emit(project);
    
    // 保存选中项目到共享服务
    this.sharedDataService.setSelectedProject(project);
    
    // 显示加载提示
    const loadingMsg = this.message.loading('正在加载设备清单数据...', { nzDuration: 0 });
    
    // 等待设备数据加载完成
    await new Promise(resolve => setTimeout(resolve, 800));
    
    // 关闭加载提示
    this.message.remove(loadingMsg.messageId);
    
    try {
      // 打开设备清单弹窗
      this.modalService.create({
        nzTitle: `设备清单 - ${project.station_name}`,
        nzContent: DeviceListComponent,
        nzWidth: '80%',
        nzFooter: null,
        nzData: {
          stationName: project.station_name
        }
      });
      
      // 获取设备数量并提示用户
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const response: any = await invoke('query_equipment_by_station', { 
          stationName: project.station_name 
        });
        
        if (response && response.equipment_list) {
          const equipmentCount = response.equipment_list.length;
          // 显示选择场站和设备数量的提示
          this.message.info(`已选择场站: ${project.station_name}，共查询到${equipmentCount}条设备记录`);
        }
      } catch (error) {
        console.error('获取设备数据失败:', error);
        // 仍然显示场站选择提示
        this.message.info(`已选择场站: ${project.station_name}`);
      }
    } catch (error) {
      console.error('打开设备清单弹窗失败:', error);
      this.message.error('打开设备清单弹窗失败');
    }
  }
}
