import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from 'rxjs';
import { ProjectInfo } from '../models/project.model';

/**
 * 项目状态服务
 * 负责管理项目相关的状态
 */
@Injectable({
  providedIn: 'root'
})
export class ProjectStateService {
  // 项目数据
  private projectDataSubject = new BehaviorSubject<ProjectInfo[]>([]);
  public projectData$: Observable<ProjectInfo[]> = this.projectDataSubject.asObservable();

  // 选中的项目
  private selectedProjectSubject = new BehaviorSubject<ProjectInfo | null>(null);
  public selectedProject$: Observable<ProjectInfo | null> = this.selectedProjectSubject.asObservable();

  // 场站号
  private stationNumberSubject = new BehaviorSubject<string>('');
  public stationNumber$: Observable<string> = this.stationNumberSubject.asObservable();

  constructor() { }

  /**
   * 设置项目数据
   * @param data 项目数据
   */
  setProjectData(data: ProjectInfo[]): void {
    this.projectDataSubject.next(data);
  }

  /**
   * 获取项目数据
   * @returns 项目数据
   */
  getProjectData(): ProjectInfo[] {
    return this.projectDataSubject.value;
  }

  /**
   * 设置选中的项目
   * @param project 选中的项目
   */
  setSelectedProject(project: ProjectInfo | null): void {
    this.selectedProjectSubject.next(project);
  }

  /**
   * 获取选中的项目
   * @returns 选中的项目
   */
  getSelectedProject(): ProjectInfo | null {
    return this.selectedProjectSubject.value;
  }

  /**
   * 设置场站号
   * @param number 场站号
   */
  setStationNumber(number: string): void {
    this.stationNumberSubject.next(number);
  }

  /**
   * 获取场站号
   * @returns 场站号
   */
  getStationNumber(): string {
    return this.stationNumberSubject.value;
  }

  /**
   * 清空所有项目相关数据
   */
  clearAll(): void {
    this.projectDataSubject.next([]);
    this.selectedProjectSubject.next(null);
    this.stationNumberSubject.next('');
  }
}
