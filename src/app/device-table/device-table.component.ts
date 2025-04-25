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
    NzMessageModule
  ],
  templateUrl: './device-table.component.html',
  styleUrl: './device-table.component.scss'
})
export class DeviceTableComponent implements OnInit {
  // 设备数据
  deviceData: DeviceItem[] = [];
  // 加载状态
  isLoading = false;
  // 表格滚动配置 - 设置高度，使用滚轮查看
  scrollConfig = { x: '800px', y: '600px' };
  
  constructor(
    private message: NzMessageService,
    private router: Router,
    private sharedDataService: SharedDataService
  ) { }

  ngOnInit(): void {
    // 初始化加载数据
    this.loadDeviceData();
  }

  /**
   * 加载设备数据
   */
  loadDeviceData(): void {
    this.isLoading = true;
    
    // 从共享服务获取数据
    const savedData = this.sharedDataService.getDeviceTableData();
    if (savedData && savedData.length > 0) {
      this.deviceData = savedData;
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
    }, 1000);
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
    // 如果是新添加的记录，则直接删除
    if (data.id < 0) {
      this.deviceData.splice(index, 1);
    } else {
      // 否则退出编辑状态
      data.isEditing = false;
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
}
