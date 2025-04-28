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
import { ProjectStateService } from '../services/project-state.service';
import { EquipmentStateService } from '../services/equipment-state.service';
import { ApiService } from '../services/api.service';
import { ProjectInfo } from '../models/project.model';
import { EquipmentItem } from '../models/equipment.model';
import { finalize } from 'rxjs';

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
  title = 'table-generation-tool';
  projectData: ProjectInfo[] = [];
  selectedProject: ProjectInfo | null = null;
  equipmentData: EquipmentItem[] = [];
  isLoading: boolean = false;
  stationNumber: string = '';
  isStationValid: boolean = false;

  constructor(
    private iconService: NzIconService,
    private message: NzMessageService,
    private router: Router,
    private projectStateService: ProjectStateService,
    private equipmentStateService: EquipmentStateService,
    private apiService: ApiService
  ) {
    this.iconService.addIcon(SearchOutline);
  }

  ngOnInit() {
    // 从状态服务中恢复数据
    this.loadSavedData();

    // 初始化验证状态
    this.validateStation();
  }

  /**
   * 从状态服务中加载已保存的数据
   */
  loadSavedData(): void {
    this.selectedProject = this.projectStateService.getSelectedProject();
    this.projectData = this.projectStateService.getProjectData();
    this.equipmentData = this.equipmentStateService.getEquipmentData();
    this.stationNumber = this.projectStateService.getStationNumber();
  }

  /**
   * 查询项目
   * @param projectNumber 项目编号
   */
  queryProject(projectNumber?: string) {
    if (!projectNumber) {
      this.message.warning('请输入项目编号');
      return;
    }

    this.isLoading = true;
    this.apiService.queryProjectByNumber(projectNumber)
      .pipe(finalize(() => this.isLoading = false))
      .subscribe({
        next: (response) => {
          this.projectData = response.projects || [];
          this.equipmentData = [];
          this.selectedProject = null;

          // 保存到状态服务
          this.projectStateService.setProjectData(this.projectData);
          this.projectStateService.setSelectedProject(null);
          this.equipmentStateService.setEquipmentData([]);

          console.log('获取到的项目数据:', response);
        },
        error: (error) => {
          console.error('调用API失败:', error);
          this.message.error('查询失败: ' + error);
        }
      });
  }

  /**
   * 清空表单
   */
  clearForm() {
    this.projectData = [];
    this.equipmentData = [];
    this.selectedProject = null;
    this.stationNumber = '';

    // 清空状态服务中的数据
    this.projectStateService.clearAll();
    this.equipmentStateService.clearAll();
  }

  /**
   * 选择项目并加载设备清单
   * @param project 选中的项目
   */
  selectProject(project: ProjectInfo) {
    this.selectedProject = project;
    // 保存到状态服务
    this.projectStateService.setSelectedProject(project);
    this.loadEquipmentData(project.station_name);
  }

  /**
   * 加载设备数据
   * @param stationName 场站名称
   */
  loadEquipmentData(stationName: string) {
    this.isLoading = true;
    this.equipmentData = [];

    this.apiService.queryEquipmentByStation(stationName)
      .pipe(finalize(() => this.isLoading = false))
      .subscribe({
        next: (response) => {
          this.equipmentData = response.equipment_list || [];

          // 保存到状态服务
          this.equipmentStateService.setEquipmentData(this.equipmentData);

          console.log('获取到的设备数据:', response);
        },
        error: (error) => {
          console.error('查询设备清单失败:', error);
          this.message.error('查询设备清单失败: ' + error);
        }
      });
  }

  /**
   * 验证场站号
   */
  validateStation() {
    this.isStationValid = this.stationNumber.trim() !== '';
    // 保存场站号到状态服务
    this.projectStateService.setStationNumber(this.stationNumber);
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

    // 保存当前状态到状态服务
    this.projectStateService.setProjectData(this.projectData);
    this.projectStateService.setSelectedProject(this.selectedProject);
    this.equipmentStateService.setEquipmentData(this.equipmentData);
    this.projectStateService.setStationNumber(this.stationNumber);

    // 跳转到设备表页面
    this.router.navigate(['/device-table']);
  }
}
