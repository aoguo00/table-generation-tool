import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
// 在需要使用的组件中导入
import { invoke } from '@tauri-apps/api/core';
import { CommonModule } from '@angular/common';

interface JdyDataQueryResponse {
  data: any[];
}

interface EquipmentItem {
  id: string;         // 数据ID
  name: string;       // 设备名称
  brand: string;      // 品牌
  model: string;      // 规格型号
  techParam: string;  // 技术参数
  quantity: number;   // 数量
  unit: string;       // 单位
  externalParam: string; // 技术参数(外部)
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
  projectData: any[] = [];
  response: JdyDataQueryResponse | null = null;
  selectedProject: any = null;
  equipmentData: EquipmentItem[] = [];
  isLoading: boolean = false;
  
  // 字段映射
  fieldMapping = {
    name: '_widget_1635777115211',          // 设备名称
    brand: '_widget_1635777115248',         // 品牌
    model: '_widget_1635777115287',         // 规格型号
    techParam: '_widget_1641439264111',     // 技术参数
    quantity: '_widget_1635777485580',      // 数量
    unit: '_widget_1654703913698',          // 单位
    externalParam: '_widget_1641439463480'  // 技术参数(外部)
  };
  
  async queryProject(projectNumber?: string) {
    if (!projectNumber) {
      alert('请输入项目编号');
      return;
    }
    
    this.isLoading = true;
    try {
      this.response = await invoke('query_jdy_data_by_project_number', { 
        project_number: projectNumber 
      });
      if (this.response) {
        this.projectData = this.response.data;
        // 清空设备数据
        this.equipmentData = [];
        this.selectedProject = null;
      }
      console.log('获取到的项目数据:', this.response);
    } catch (error) {
      console.error('调用API失败:', error);
      alert('查询失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  clearForm() {
    // 清空表单和数据
    this.projectData = [];
    this.equipmentData = [];
    this.response = null;
    this.selectedProject = null;
  }
  
  async selectProject(project: any) {
    this.selectedProject = project;
    
    // 获取场站名称
    const stationName = project._widget_1635777114991;
    console.log('选中场站:', stationName);
    
    // 查询设备清单
    await this.loadEquipmentData(stationName);
  }
  
  async loadEquipmentData(stationName: string) {
    this.isLoading = true;
    this.equipmentData = [];
    
    try {
      console.log(`开始查询场站"${stationName}"的设备清单`);
      const response: JdyDataQueryResponse = await invoke('query_equipment_by_station', { 
        stationName: stationName 
      });
      
      console.log('获取到的设备数据:', response);
      
      if (response && response.data && response.data.length > 0) {
        // 处理嵌套的数据结构
        for (const record of response.data) {
          console.log('当前记录:', record);
          // 首先检查是否有特定的字段包含设备列表
          const equipmentField = '_widget_1635777115095';
          
          if (record && Array.isArray(record[equipmentField])) {
            // 直接访问设备数组
            const equipmentList = record[equipmentField];
            console.log('找到设备列表:', equipmentList);
            
            // 将API返回数据转换为前端展示需要的格式
            const formattedEquipment = equipmentList.map((item: any) => ({
              id: item._id || '',
              name: item._widget_1635777115211 || '',
              brand: item._widget_1635777115248 || '',
              model: item._widget_1635777115287 || '',
              techParam: item._widget_1641439264111 || '',
              quantity: item._widget_1635777485580 || 0,
              unit: item._widget_1654703913698 || '',
              externalParam: item._widget_1641439463480 || ''
            }));
            
            this.equipmentData.push(...formattedEquipment);
          } else {
            // 尝试直接处理记录本身
            console.log(`尝试直接处理记录`);
            const item = {
              id: record._id || '',
              name: record[this.fieldMapping.name] || '',
              brand: record[this.fieldMapping.brand] || '',
              model: record[this.fieldMapping.model] || '',
              techParam: record[this.fieldMapping.techParam] || '',
              quantity: record[this.fieldMapping.quantity] || 0,
              unit: record[this.fieldMapping.unit] || '',
              externalParam: record[this.fieldMapping.externalParam] || ''
            };
            
            // 只有当至少name字段有值时才添加
            if (item.name) {
              this.equipmentData.push(item);
            } else {
              console.log(`记录中没有找到有效的设备数据`);
            }
          }
        }
      } else {
        console.log('未找到设备数据');
      }
      
      console.log('格式化后的设备数据:', this.equipmentData);
    } catch (error) {
      console.error('查询设备清单失败:', error);
      alert('查询设备清单失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }
  
  generatePointTable() {
    if (!this.selectedProject) {
      alert('请先选择一个项目');
      return;
    }
    console.log('生成点表');
    // 这里实现生成点表的逻辑
  }
  
  uploadPointTable(type: string) {
    if (!this.selectedProject) {
      alert('请先选择一个项目');
      return;
    }
    console.log(`上传${type}点表`);
    // 这里实现上传点表的逻辑
  }
}
