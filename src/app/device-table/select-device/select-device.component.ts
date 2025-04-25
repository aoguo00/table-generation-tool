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
 * 允许用户在两个列表之间选择和移动设备
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

  constructor(private message: NzMessageService) {}

  ngOnInit(): void {
    // 初始化设备列表数据
    for (let i = 0; i < 20; i++) {
      this.list.push({
        key: i.toString(),
        title: `设备${i + 1}`,
        description: `设备${i + 1}的详细描述信息`,
        disabled: false, // 移除禁用设置，使所有设备都可选
        tag: `TAG-${i + 100}`,
        quantity: i + 1, // 添加数量字段
        checked: false,
        isEditing: false // 添加编辑状态字段
      });
    }

    // 初始化右侧列表数据
    [2, 3].forEach(idx => (this.list[idx].direction = 'right'));
    
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
    
    // 当数据变化时，发送右侧数据给父组件
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
    // 校验必填字段
    if (!data['tag']) {
      this.message.warning('位号不能为空');
      return;
    }

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
}
