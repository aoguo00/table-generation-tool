import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

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

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'table-generation-tool';
  projectData: ProjectInfo[] = [];
  selectedProject: ProjectInfo | null = null;
  equipmentData: EquipmentItem[] = [];
  isLoading: boolean = false;
  
  async queryProject(projectNumber?: string) {
    if (!projectNumber) {
      alert('请输入项目编号');
      return;
    }
    
    this.isLoading = true;
    try {
      const response: ProjectQueryResponse = await invoke('query_jdy_data_by_project_number', { 
        projectNumber: projectNumber 
      });
      
      this.projectData = response.projects || [];
      this.equipmentData = [];
      this.selectedProject = null;
      
      console.log('获取到的项目数据:', response);
    } catch (error) {
      console.error('调用API失败:', error);
      alert('查询失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  clearForm() {
    this.projectData = [];
    this.equipmentData = [];
    this.selectedProject = null;
  }
  
  async selectProject(project: ProjectInfo) {
    this.selectedProject = project;
    await this.loadEquipmentData(project.station_name);
  }
  
  async loadEquipmentData(stationName: string) {
    this.isLoading = true;
    this.equipmentData = [];
    
    try {
      const response: EquipmentQueryResponse = await invoke('query_equipment_by_station', { 
        stationName: stationName 
      });
      
      this.equipmentData = response.equipment_list || [];
    } catch (error) {
      console.error('查询设备清单失败:', error);
      alert('查询设备清单失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  async generatePointTable() {
    if (!this.selectedProject) {
      alert('请先选择一个项目');
      return;
    }
    
    this.isLoading = true;
    try {
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
      alert(`IO点表已生成: ${filePath}`);
    } catch (error) {
      console.error('生成IO点表失败:', error);
      alert('生成IO点表失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  uploadPointTable(type: string) {
    if (!this.selectedProject) {
      alert('请先选择一个项目');
      return;
    }
    console.log(`上传${type}点表`);
  }
}
