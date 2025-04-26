import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';

import { NzSwitchModule } from 'ng-zorro-antd/switch';
import { NzTableModule } from 'ng-zorro-antd/table';
import { NzTagModule } from 'ng-zorro-antd/tag';
import { NzTransferModule, TransferChange, TransferItem, TransferSelectChange } from 'ng-zorro-antd/transfer';
import { NzInputModule } from 'ng-zorro-antd/input';
import { NzInputNumberModule } from 'ng-zorro-antd/input-number';
import { NzDividerModule } from 'ng-zorro-antd/divider';
import { NzMessageModule, NzMessageService } from 'ng-zorro-antd/message';

/**
 * 设备选择穿梭框组件
 * 允许用户在两个列表之间选择和移动设备，并编辑相关信息
 */
@Component({
  selector: 'app-select-device',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    NzSwitchModule,
    NzTableModule,
    NzTagModule,
    NzTransferModule,
    NzInputModule,
    NzInputNumberModule,
    NzDividerModule,
    NzMessageModule
  ],
  templateUrl: './select-device.component.html',
  styleUrl: './select-device.component.scss'
})
export class SelectDeviceComponent implements OnInit {
  /**
   * 穿梭框数据列表
   */
  list: TransferItem[] = [];

  /**
   * 将数据转换为TransferItems类型
   */
  $asTransferItems = (data: unknown): TransferItem[] => data as TransferItem[];

  /**
   * 禁用状态
   */
  disabled = false;

  /**
   * 是否显示搜索框
   */
  showSearch = true;

  /**
   * 当右侧穿梭框数据变化时发出事件
   */
  @Output() rightItemsChange = new EventEmitter<any[]>();

  /**
   * 固定的设备列表
   */
  readonly fixedDevices = [
    { name: '阀门', placeholder: 'INV-xxx' },
    { name: '流量计', placeholder: 'DCB-xxx' },
    { name: '压缩机', placeholder: 'PV-xxx' },
    { name: '可燃气体探测器', placeholder: 'BOX-xxx' },
    { name: '干燥器', placeholder: 'SUB-xxx' }
  ];

  constructor(private message: NzMessageService) { }

  ngOnInit(): void {
    // 初始化设备列表数据
    this.fixedDevices.forEach((device, index) => {
      this.list.push({
        key: index.toString(),
        title: device.name,
        description: '',  // 初始描述为空
        disabled: false,
        tag: '',          // 初始位号为空
        tagPlaceholder: device.placeholder, // 保存位号提示
        quantity: null,   // 初始数量为空
        checked: false,
        isEditing: false
      });
    });

    // 首次加载时发送右侧数据
    this.emitRightItems();
  }

  /**
   * 发送右侧穿梭框的数据给父组件
   */
  private emitRightItems(): void {
    const rightItems = this.list.filter(item => item.direction === 'right');
    this.rightItemsChange.emit(rightItems);
  }

  /**
   * 选择事件处理函数
   * @param ret 选择改变事件对象
   */
  select(ret: TransferSelectChange): void {
    console.log('nzSelectChange', ret);
  }

  /**
   * 穿梭框数据变更事件处理函数
   * @param ret 穿梭框改变事件对象
   */
  change(ret: TransferChange): void {
    console.log('nzChange', ret);

    // 处理穿梭
    const listKeys = ret.list.map(l => l['key']);
    const hasOwnKey = (e: TransferItem): boolean => e.hasOwnProperty('key');

    this.list = this.list.map(e => {
      if (listKeys.includes(e['key']) && hasOwnKey(e)) {
        if (ret.to === 'left') {
          delete e.hide;
        } else if (ret.to === 'right') {
          e.hide = false;
        }
      }
      return e;
    });

    // 当数据变化时，立即发送右侧数据给父组件
    this.emitRightItems();
  }

  /**
   * 开始编辑设备项
   * @param data 设备项数据
   */
  startEdit(data: TransferItem): void {
    // 设置当前项为编辑状态
    data['isEditing'] = true;
  }

  /**
   * 保存编辑设备项
   * @param data 设备项数据
   */
  saveEdit(data: TransferItem): void {
    // 设置为非编辑状态
    data['isEditing'] = false;
    this.message.success('保存成功');

    // 编辑完成后，更新右侧数据
    this.emitRightItems();
  }

  /**
   * 取消编辑设备项
   * @param data 设备项数据
   */
  cancelEdit(data: TransferItem): void {
    // 取消编辑状态
    data['isEditing'] = false;
    this.message.info('已取消编辑');
  }

  /**
   * 更新穿梭框数据
   * 用于从外部组件同步数据到穿梭框
   * @param deviceData 设备表数据
   */
  updateTransferData(deviceData: any[]): void {
    if (!deviceData || deviceData.length === 0) {
      // 如果没有设备数据，只保留左侧未选中的项目
      this.list = this.list.map(item => {
        if (item.direction === 'right') {
          delete item.direction;
          delete item.hide;
        }
        return item;
      });
      return;
    }

    // 更新穿梭框数据
    const deviceMap = new Map(deviceData.map(item => [item.name, item]));

    this.list = this.list.map(item => {
      const deviceItem = deviceMap.get(item['title']);

      // 如果在设备表中找到对应项，更新数据并标记为右侧
      if (deviceItem) {
        item['tag'] = deviceItem.tagNumber || '';
        item['description'] = deviceItem.description || '';
        item['quantity'] = deviceItem.quantity || 1;
        item.direction = 'right';
      } else if (item.direction === 'right') {
        // 如果在设备表中没找到，但之前在右侧，移回左侧
        delete item.direction;
        delete item.hide;
      }

      return item;
    });

    console.log('穿梭框数据已更新', this.list);
  }
}
