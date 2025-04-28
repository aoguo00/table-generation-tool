import { Component, OnInit, ViewChild } from '@angular/core';
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
import { NzSwitchModule } from 'ng-zorro-antd/switch';
import { NzTagModule } from 'ng-zorro-antd/tag';
import { NzTransferModule } from 'ng-zorro-antd/transfer';
import { SelectDeviceComponent } from './select-device/select-device.component';
import { DeviceItem } from '../models/device-table.model';
import { DeviceTableService } from '../services/device-table.service';
import { IoTableService } from '../services/io-table.service';
import { finalize } from 'rxjs';

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

  // 引用穿梭框组件
  @ViewChild(SelectDeviceComponent) selectDeviceComponent!: SelectDeviceComponent;

  constructor(
    private message: NzMessageService,
    private router: Router,
    private deviceTableService: DeviceTableService,
    private ioTableService: IoTableService
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
    this.deviceData = this.deviceTableService.loadDeviceTableData();
    this.isLoading = false;
  }

  /**
   * 返回上一页
   */
  goBack(): void {
    // 保存当前数据
    this.deviceTableService.updateDeviceTableData([...this.deviceData]);
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

    // 保存编辑
    const success = this.deviceTableService.saveDeviceEdit(data);
    if (success) {
      // 触发变更检测
      this.deviceData = [...this.deviceData];

      // 同步更新穿梭框数据
      if (this.selectDeviceComponent) {
        this.selectDeviceComponent.updateTransferData(this.deviceData);
      }

      this.message.success('保存成功');
    }
  }

  /**
   * 取消编辑
   * @param data 设备项数据
   * @param index 设备项索引
   */
  cancelEdit(data: DeviceItem, index: number): void {
    // 取消编辑
    this.deviceData = this.deviceTableService.cancelDeviceEdit(data, index);

    // 同步更新穿梭框数据
    if (this.selectDeviceComponent) {
      this.selectDeviceComponent.updateTransferData(this.deviceData);
    }

    // 显示消息
    if (data.id < 0) {
      this.message.info('已取消添加');
    } else {
      this.message.info('已取消编辑');
    }
  }

  /**
   * 删除设备
   * @param index 设备项索引
   */
  deleteDevice(index: number): void {
    // 删除设备
    this.deviceData = this.deviceTableService.deleteDevice(index);

    // 同步更新穿梭框数据
    if (this.selectDeviceComponent) {
      this.selectDeviceComponent.updateTransferData(this.deviceData);
    }

    this.message.success('设备已删除');
  }

  /**
   * 添加新设备
   */
  addDevice(): void {
    // 添加新设备
    const newDevice = this.deviceTableService.addDevice();

    // 更新视图
    this.deviceData = [newDevice, ...this.deviceData.filter(item => item.id !== newDevice.id)];

    this.message.info('请完成设备信息填写');
  }

  /**
   * 保存所有设备数据
   * 收集表格中的设备数据并准备发送到后端
   */
  saveAllDevices(): void {
    // 检查是否有正在编辑的项
    const editingItem = this.deviceData.find(item => item.isEditing);
    if (editingItem) {
      this.message.warning('请先完成编辑中的项目');
      return;
    }

    // 更新保存的数据
    this.deviceTableService.updateDeviceTableData([...this.deviceData]);

    // 准备要发送到后端的数据
    const tableData = this.deviceTableService.prepareDeviceTableDataForBackend();
    if (tableData.length === 0) {
      this.message.warning('没有可保存的数据');
      return;
    }

    this.message.success('设备清单已保存');

    // 打印收集到的表格数据，方便调试
    console.log('准备发送到后端的表格数据：', tableData);
  }

  /**
   * 生成点表
   */
  generatePointTable(): void {
    this.isLoading = true;

    this.ioTableService.generateIOPointTable()
      .pipe(finalize(() => this.isLoading = false))
      .subscribe({
        next: (filePath) => {
          if (filePath) {
            console.log('生成的IO点表路径:', filePath);
          }
        },
        error: (error) => {
          console.error('生成IO点表失败:', error);
          this.message.error('生成IO点表失败: ' + error);
        }
      });
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
      description: item.description || '', // 确保描述字段有值
      quantity: item.quantity || 1,
      isEditing: false
    }));

    // 只保留新设备列表中的设备和手动添加的设备（ID为负数的）
    this.deviceData = [
      ...this.deviceData.filter(item => item.id < 0), // 保留手动添加的设备
      ...newDevices // 添加从穿梭框来的设备
    ];

    // 保存到服务
    this.deviceTableService.updateDeviceTableData([...this.deviceData]);

    // 触发变更检测（防止视图不更新）
    setTimeout(() => {
      this.deviceData = [...this.deviceData];
    }, 0);
  }
}
