import { Component, OnInit, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NzTableModule } from 'ng-zorro-antd/table';
import { NzButtonModule } from 'ng-zorro-antd/button';
import { NzModalRef, NzModalModule } from 'ng-zorro-antd/modal';
import { NzSpinModule } from 'ng-zorro-antd/spin';
import { NzIconModule } from 'ng-zorro-antd/icon';
import { NzMessageModule, NzMessageService } from 'ng-zorro-antd/message';

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

/**
 * 设备清单弹窗组件
 * 使用固定表头表格显示设备清单
 */
@Component({
  selector: 'app-device-list',
  standalone: true,
  imports: [
    CommonModule,
    NzTableModule,
    NzButtonModule,
    NzModalModule,
    NzSpinModule,
    NzIconModule,
    NzMessageModule
  ],
  templateUrl: './device-list.component.html',
  styleUrl: './device-listcomponent.scss'
})
export class DeviceListComponent implements OnInit {
  // 当前场站名称
  stationName: string = '';
  // 设备数据
  equipmentData: EquipmentItem[] = [];
  // 加载状态
  isLoading = false;
  // 表格滚动配置
  scrollConfig = { x: '1200px', y: '400px' };

  // 注入的服务
  private modalRef = inject(NzModalRef);
  private message = inject(NzMessageService);

  constructor() {
    // 从modalData中获取场站名称
    const modalData = this.modalRef.getConfig().nzData as { stationName: string };
    if (modalData && modalData.stationName) {
      this.stationName = modalData.stationName;
    }
  }

  ngOnInit(): void {
    // 加载设备数据
    this.loadEquipmentData();
  }

  /**
   * 加载设备数据
   */
  async loadEquipmentData(): Promise<void> {
    if (!this.stationName) {
      this.message.warning('未指定场站名称');
      return;
    }

    this.isLoading = true;

    try {
      // 如果是Tauri应用，则调用后端接口获取数据
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const response = await invoke('query_equipment_by_station', {
          stationName: this.stationName
        });

        if (response && (response as any).equipment_list) {
          this.equipmentData = (response as any).equipment_list;
        } else {
          this.equipmentData = [];
        }
      } catch (e) {
        // 如果不是Tauri应用或调用失败，则使用模拟数据
        console.error('调用Tauri API失败:', e);

        // 模拟数据
        setTimeout(() => {
          this.equipmentData = [
            { id: '1', name: '逆变器', brand: '阳光电源', model: 'SG110CX', tech_param: '1100V/110kW', quantity: 10, unit: '台', external_param: 'RS485通讯' },
            { id: '2', name: '汇流箱', brand: '阳光电源', model: 'SB16', tech_param: '16路直流汇流', quantity: 20, unit: '台', external_param: 'RS485通讯' },
            { id: '3', name: '箱变', brand: '特变电工', model: 'ZBW-3150kVA', tech_param: '35kV/3150kVA', quantity: 1, unit: '台', external_param: 'SCADA系统' }
          ];
        }, 800);
      }
    } finally {
      this.isLoading = false;
    }
  }

  /**
   * 关闭弹窗
   */
  close(): void {
    this.modalRef.close();
  }
} 