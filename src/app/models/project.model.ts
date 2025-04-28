/**
 * 项目信息接口
 */
export interface ProjectInfo {
  id: string;
  project_name: string;
  project_number: string;
  design_number: string;
  customer_name: string;
  station_name: string;
  checked?: boolean;
}

/**
 * 项目查询响应
 */
export interface ProjectQueryResponse {
  projects: ProjectInfo[];
}
