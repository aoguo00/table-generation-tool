import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { NzTableModule } from 'ng-zorro-antd/table';
import { NzButtonModule } from 'ng-zorro-antd/button';
import { NzInputModule } from 'ng-zorro-antd/input';
import { NzIconModule } from 'ng-zorro-antd/icon';
import { NzSpinModule } from 'ng-zorro-antd/spin';
import { NzInputNumberModule } from 'ng-zorro-antd/input-number';
import { NzMessageModule, NzMessageService } from 'ng-zorro-antd/message';
import { Router } from '@angular/router';
import { SharedDataService } from '../shared-data.service';
import { NzSwitchModule } from 'ng-zorro-antd/switch';
import { NzTagModule } from 'ng-zorro-antd/tag';
import { NzTransferModule, TransferChange, TransferItem, TransferSelectChange } from 'ng-zorro-antd/transfer';
import { SelectDeviceComponent } from './select-device/select-device.component';

/**
 * 设备项接口
 */
interface DeviceItem {
  id: number;
  name: string;      // 设备名称
  tagNumber: string; // 位号
  quantity: number;  // 数量
  isEditing?: boolean; // 是否处于编辑状态
}

/**
 * 设备表组件
 * 显示项目中的设备列表，允许编辑位号和数量
 */
@Component({
  selector: 'app-device-table',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    NzTableModule,
    NzButtonModule,
    NzInputModule,
    NzIconModule,
    NzSpinModule,
    NzInputNumberModule,
    NzMessageModule,
    NzSwitchModule,
    NzTagModule,
    NzTransferModule,
    SelectDeviceComponent
  ],
  templateUrl: './device-table.component.html',
  styleUrl: './device-table.component.scss'
})
export class DeviceTableComponent implements OnInit {
  // 设备数据
  deviceData: DeviceItem[] = [];
  // 加载状态
  isLoading = false;
  // Tauri应用标志
  isTauriApp = false;
  
  constructor(
    private message: NzMessageService,
    private router: Router,
    private sharedDataService: SharedDataService
  ) { }

  ngOnInit(): void {
    // 检查是否在Tauri环境中
    this.checkTauriEnvironment();
    
    // 先打印共享服务中的数据，方便调试
    console.log('初始化设备表组件, 共享服务数据：', {
      selectedProject: this.sharedDataService.getSelectedProject(),
      equipmentData: this.sharedDataService.getEquipmentData(),
      deviceTableData: this.sharedDataService.getDeviceTableData(),
      stationNumber: this.sharedDataService.getStationNumber()
    });
    
    // 初始化加载数据
    this.loadDeviceData();
  }

  /**
   * 检查是否在Tauri环境中运行
   */
  async checkTauriEnvironment(): Promise<void> {
    try {
      const { getVersion } = await import('@tauri-apps/api/app');
      await getVersion();
      this.isTauriApp = true;
    } catch (error) {
      this.isTauriApp = false;
      console.log('Running in web environment');
    }
  }

  /**
   * 加载设备数据
   */
  loadDeviceData(): void {
    this.isLoading = true;
    
    // 从共享服务获取设备表数据
    const savedTableData = this.sharedDataService.getDeviceTableData();
    if (savedTableData && savedTableData.length > 0) {
      this.deviceData = savedTableData;
      this.isLoading = false;
      return;
    }
    
    // 如果没有设备表数据，尝试从设备清单中转换
    const equipmentData = this.sharedDataService.getEquipmentData();
    if (equipmentData && equipmentData.length > 0) {
      console.log('从设备清单中加载数据', equipmentData);
      // 转换格式
      this.deviceData = equipmentData.map((item, index) => ({
        id: index + 1,
        name: item.name,
        tagNumber: `TAG-${item.id}`, // 默认位号
        quantity: item.quantity
      }));
      
      // 保存到共享服务
      this.sharedDataService.setDeviceTableData([...this.deviceData]);
      this.isLoading = false;
      return;
    }
    
    // 如果没有保存的数据，则加载模拟数据
    setTimeout(() => {
      this.deviceData = [
        { id: 1, name: '阀门', tagNumber: 'INV-001', quantity: 10 },
        { id: 2, name: '流量计', tagNumber: 'DCB-001', quantity: 20 },
        { id: 3, name: '压缩机', tagNumber: 'PV-001', quantity: 500 },
        { id: 4, name: '可燃气体探测器', tagNumber: 'BOX-001', quantity: 1 },
        { id: 5, name: '干燥器', tagNumber: 'SUB-001', quantity: 1 }
      ];
      // 保存到共享服务
      this.sharedDataService.setDeviceTableData([...this.deviceData]);
      this.isLoading = false;
    }, 100);
  }

  /**
   * 返回上一页
   */
  goBack(): void {
    // 保存当前数据到共享服务
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
    // 返回上一页
    window.history.back();
  }

  /**
   * 开始编辑设备项
   * @param data 设备项数据
   */
  startEdit(data: DeviceItem): void {
    // 标记为编辑状态
    data.isEditing = true;
  }

  /**
   * 保存编辑内容
   * @param data 设备项数据
   */
  saveEdit(data: DeviceItem): void {
    // 验证位号不为空
    if (!data.tagNumber || data.tagNumber.trim() === '') {
      this.message.warning('位号不能为空');
      return;
    }

    // 验证数量大于0
    if (data.quantity <= 0) {
      this.message.warning('数量必须大于0');
      return;
    }

    // 退出编辑状态
    data.isEditing = false;
    
    // 触发变更检测
    this.deviceData = [...this.deviceData];
    
    // 更新保存的数据
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
    this.message.success('保存成功');
  }

  /**
   * 取消编辑
   * @param data 设备项数据
   * @param index 设备项索引
   */
  cancelEdit(data: DeviceItem, index: number): void {
    // 如果是新添加的行（ID为负数），则删除该行
    if (data.id < 0) {
      // 删除该行
      this.deviceData.splice(index, 1);
      this.deviceData = [...this.deviceData]; // 创建新数组以触发变更检测
      this.message.info('已取消添加');
    } else {
      // 如果是编辑现有行，则只退出编辑状态，不删除数据
      data.isEditing = false;
      
      // 恢复原始数据（从服务中获取）
      const savedData = this.sharedDataService.getDeviceTableData();
      const originalItem = savedData.find(item => item.id === data.id);
      if (originalItem) {
        // 恢复原始数据
        data.name = originalItem.name;
        data.tagNumber = originalItem.tagNumber;
        data.quantity = originalItem.quantity;
      }
      
      this.message.info('已取消编辑');
    }
    
    // 更新保存的数据
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
  }

  /**
   * 删除设备
   * @param index 设备项索引
   */
  deleteDevice(index: number): void {
    // 删除指定索引的设备
    this.deviceData.splice(index, 1);
    // 创建新数组以触发变更检测
    this.deviceData = [...this.deviceData];
    // 更新保存的数据
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
    this.message.success('设备已删除');
  }

  /**
   * 添加新设备
   */
  addDevice(): void {
    // 创建一个新的设备记录并置为编辑状态
    const newDevice: DeviceItem = {
      id: -this.deviceData.length - 1, // 临时ID为负数
      name: '',
      tagNumber: '',
      quantity: 1,
      isEditing: true
    };
    
    // 添加到列表开头
    this.deviceData = [newDevice, ...this.deviceData];
    // 更新保存的数据
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
  }

  /**
   * 保存所有设备数据
   */
  saveAllDevices(): void {
    // 检查是否有正在编辑的项
    const editingItem = this.deviceData.find(item => item.isEditing);
    if (editingItem) {
      this.message.warning('请先完成编辑中的项目');
      return;
    }

    // 更新保存的数据
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
    
    // 这里可以发送数据到后端保存
    this.message.success('设备清单已保存');
    console.log('设备数据：', this.deviceData);
  }

  /**
   * 生成点表 - 严格按照home.component.ts中的实现
   */
  async generatePointTable() {
    if (!this.validateBeforeGeneration()) {
      return;
    }
    
    this.isLoading = true;
    try {
      // 加载Tauri API
      const { invoke } = await import('@tauri-apps/api/core');
      const { getCurrentWindow } = await import('@tauri-apps/api/window');

      // 准备数据和生成点表
      const equipmentItems = this.prepareEquipmentData();
      const filePath = await this.callGeneratePointTable(invoke, getCurrentWindow, equipmentItems);
      
      // 处理生成结果
      await this.handleGeneratedFile(invoke, filePath);
    } catch (error) {
      console.error('生成IO点表失败:', error);
      this.message.error('生成IO点表失败: ' + error);
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * 验证生成点表前的条件
   * @returns 是否通过验证
   */
  private validateBeforeGeneration(): boolean {
    const selectedProject = this.sharedDataService.getSelectedProject();
    if (!selectedProject) {
      this.message.warning('请先选择一个项目');
      return false;
    }
    
    if (!this.isTauriApp) {
      this.message.warning('此功能仅在桌面应用中可用');
      return false;
    }
    
    return true;
  }

  /**
   * 准备设备数据
   * @returns 处理后的设备数据
   */
  private prepareEquipmentData() {
    const selectedProject = this.sharedDataService.getSelectedProject();
    const equipmentData = this.sharedDataService.getEquipmentData();
    
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
   * 调用后端生成IO点表
   * @param invoke Tauri invoke函数
   * @param getCurrentWindow 获取当前窗口函数
   * @param equipmentItems 设备数据
   * @returns 生成的文件路径
   */
  private async callGeneratePointTable(invoke: any, getCurrentWindow: any, equipmentItems: any[]) {
    // 获取当前窗口
    const currentWindow = await getCurrentWindow();
    const selectedProject = this.sharedDataService.getSelectedProject();
    
    // 调用后端生成IO点表，严格按照原来的参数格式
    console.log('发送到后端的参数:', {
      equipmentData: equipmentItems,
      stationName: selectedProject!.station_name
    });
    
    const filePath: string = await invoke('generate_io_point_table', {
      equipmentData: equipmentItems,
      stationName: selectedProject!.station_name,
      window: currentWindow
    });
    
    console.log('生成的IO点表路径:', filePath);
    return filePath;
  }

  /**
   * 处理生成的文件
   * @param invoke Tauri invoke函数
   * @param filePath 文件路径
   */
  private async handleGeneratedFile(invoke: any, filePath: string) {
    // 自动打开生成的文件
    await invoke('open_file', { path: filePath });
    this.message.success(`IO点表已生成并打开: ${filePath}`);
  }

  /**
   * 处理穿梭框右侧数据变化的事件
   * @param rightItems 穿梭框右侧的数据项
   */
  handleRightItemsChange(rightItems: any[]): void {
    console.log('穿梭框右侧数据变化:', rightItems);
    
    // 将穿梭框数据转换为DeviceItem格式
    const newDevices: DeviceItem[] = rightItems.map((item, index) => ({
      id: parseInt(item.key) + 1000, // 使用一个不同的ID范围，避免冲突
      name: item.title,
      tagNumber: item.tag || '',
      quantity: item.quantity || 1,
      isEditing: false
    }));
    
    // 只保留新设备列表中的设备和手动添加的设备（ID为负数的）
    this.deviceData = [
      ...this.deviceData.filter(item => item.id < 0), // 保留手动添加的设备
      ...newDevices // 添加从穿梭框来的设备
    ];
    
    // 保存到共享服务
    this.sharedDataService.setDeviceTableData([...this.deviceData]);
    
    // 触发变更检测（防止视图不更新）
    setTimeout(() => {
      this.deviceData = [...this.deviceData];
    }, 0);
  }
}
