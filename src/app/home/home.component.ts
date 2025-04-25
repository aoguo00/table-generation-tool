import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { NzInputModule } from 'ng-zorro-antd/input';
import { NzIconModule } from 'ng-zorro-antd/icon';
import { NzButtonModule } from 'ng-zorro-antd/button';
import { NzTableModule } from 'ng-zorro-antd/table';
import { NzTabsModule } from 'ng-zorro-antd/tabs';
import { NzSpinModule } from 'ng-zorro-antd/spin';
import { FormsModule } from '@angular/forms';
import { SearchOutline } from '@ant-design/icons-angular/icons';
import { NzIconService } from 'ng-zorro-antd/icon';
import { NzMessageService } from 'ng-zorro-antd/message';
import { NzMessageModule } from 'ng-zorro-antd/message';
import { StationListComponent } from './station-list/station-list.component';
import { SharedDataService } from '../shared-data.service';

// 项目信息接口
interface ProjectInfo {
  id: string;
  project_name: string;
  project_number: string;
  design_number: string;
  customer_name: string;
  station_name: string;
}

// 项目查询响应
interface ProjectQueryResponse {
  projects: ProjectInfo[];
}

// 设备信息接口
interface EquipmentItem {
  id: string;
  name: string;
  brand: string;
  model: string;
  tech_param: string;
  quantity: number;
  unit: string;
  external_param: string;
}

// 设备查询响应
interface EquipmentQueryResponse {
  equipment_list: EquipmentItem[];
}

/**
 * 主页组件
 * 显示项目查询和设备列表
 */
@Component({
  selector: 'app-home',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    NzInputModule,
    NzIconModule,
    NzButtonModule,
    NzTableModule,
    NzTabsModule,
    NzSpinModule,
    NzMessageModule,
    StationListComponent
  ],
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
  constructor(
    private iconService: NzIconService,
    private message: NzMessageService,
    private router: Router,
    private sharedDataService: SharedDataService
  ) {
    this.iconService.addIcon(SearchOutline);
  }

  title = 'table-generation-tool';
  projectData: ProjectInfo[] = [];
  selectedProject: ProjectInfo | null = null;
  equipmentData: EquipmentItem[] = [];
  isLoading: boolean = false;
  isTauriApp: boolean = false;
  stationNumber: string = '';
  isStationValid: boolean = false;

  async ngOnInit() {
    // 检查是否在Tauri环境中
    try {
      const { getVersion } = await import('@tauri-apps/api/app');
      await getVersion();
      this.isTauriApp = true;
    } catch (error) {
      this.isTauriApp = false;
      console.log('Running in web environment');
    }
    
    // 从共享服务中恢复数据
    this.loadSavedData();
    
    // 初始化验证状态
    this.validateStation();
  }
  
  /**
   * 从共享服务中加载已保存的数据
   */
  loadSavedData(): void {
    const savedProject = this.sharedDataService.getSelectedProject();
    const savedProjectData = this.sharedDataService.getProjectData();
    const savedEquipmentData = this.sharedDataService.getEquipmentData();
    const savedStationNumber = this.sharedDataService.getStationNumber();
    
    if (savedProject) {
      this.selectedProject = savedProject;
    }
    
    if (savedProjectData && savedProjectData.length > 0) {
      this.projectData = savedProjectData;
    }
    
    if (savedEquipmentData && savedEquipmentData.length > 0) {
      this.equipmentData = savedEquipmentData;
    }
    
    if (savedStationNumber) {
      this.stationNumber = savedStationNumber;
    }
  }
  
  async queryProject(projectNumber?: string) {
    if (!projectNumber) {
      this.message.warning('请输入项目编号');
      return;
    }
    
    if (!this.isTauriApp) {
      this.message.warning('此功能仅在桌面应用中可用');
      return;
    }
    
    this.isLoading = true;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const response: ProjectQueryResponse = await invoke('query_jdy_data_by_project_number', { 
        projectNumber: projectNumber 
      });
      
      this.projectData = response.projects || [];
      this.equipmentData = [];
      this.selectedProject = null;
      
      // 保存到共享服务
      this.sharedDataService.setProjectData(this.projectData);
      this.sharedDataService.setSelectedProject(null);
      this.sharedDataService.setEquipmentData([]);
      
      console.log('获取到的项目数据:', response);
    } catch (error) {
      console.error('调用API失败:', error);
      this.message.error('查询失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  clearForm() {
    this.projectData = [];
    this.equipmentData = [];
    this.selectedProject = null;
    this.stationNumber = '';
    
    // 清空共享服务中的数据
    this.sharedDataService.clearAll();
  }
  
  /**
   * 选择项目并加载设备清单
   * @param project 选中的项目
   */
  async selectProject(project: ProjectInfo) {
    this.selectedProject = project;
    // 保存到共享服务
    this.sharedDataService.setSelectedProject(project);
    await this.loadEquipmentData(project.station_name);
  }
  
  async loadEquipmentData(stationName: string) {
    if (!this.isTauriApp) {
      this.message.warning('此功能仅在桌面应用中可用');
      return;
    }

    this.isLoading = true;
    this.equipmentData = [];
    
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const response: EquipmentQueryResponse = await invoke('query_equipment_by_station', { 
        stationName: stationName 
      });
      
      this.equipmentData = response.equipment_list || [];
      
      // 保存到共享服务
      this.sharedDataService.setEquipmentData(this.equipmentData);
    } catch (error) {
      console.error('查询设备清单失败:', error);
      this.message.error('查询设备清单失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  async generatePointTable() {
    if (!this.selectedProject) {
      this.message.warning('请先选择一个项目');
      return;
    }
    
    if (!this.isTauriApp) {
      this.message.warning('此功能仅在桌面应用中可用');
      return;
    }
    
    this.isLoading = true;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const { getCurrentWindow } = await import('@tauri-apps/api/window');

      // 构造发送给后端的数据结构
      const equipmentItems = this.equipmentData.map(item => ({
        ...item,
        station_name: this.selectedProject!.station_name
      }));

      // 获取当前窗口
      const currentWindow = await getCurrentWindow();
      
      // 调用后端生成IO点表
      const filePath: string = await invoke('generate_io_point_table', {
        equipmentData: equipmentItems,
        stationName: this.selectedProject.station_name,
        window: currentWindow
      });
      
      console.log('生成的IO点表路径:', filePath);
      
      // 自动打开生成的文件
      await invoke('open_file', { path: filePath });
      this.message.success(`IO点表已生成并打开: ${filePath}`);
    } catch (error) {
      console.error('生成IO点表失败:', error);
      this.message.error('生成IO点表失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  validateStation() {
    this.isStationValid = this.stationNumber.trim() !== '';
    // 保存场站号到共享服务
    this.sharedDataService.setStationNumber(this.stationNumber);
  }

  /**
   * 跳转到设备表页面
   * 点击下一步按钮时调用
   */
  goToDeviceTable() {
    if (!this.selectedProject) {
      this.message.warning('请先选择一个项目');
      return;
    }
    
    if (!this.isStationValid) {
      this.message.warning('请输入有效的场站编号');
      return;
    }
    
    // 保存当前状态到共享服务
    this.sharedDataService.setProjectData(this.projectData);
    this.sharedDataService.setSelectedProject(this.selectedProject);
    this.sharedDataService.setEquipmentData(this.equipmentData);
    this.sharedDataService.setStationNumber(this.stationNumber);
    
    // 跳转到设备表页面
    this.router.navigate(['/device-table']);
  }
}
