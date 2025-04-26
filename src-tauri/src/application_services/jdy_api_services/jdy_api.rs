use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/*
JiandaoyunApiClient: 简道云API客户端，用于与简道云接口进行交互
- 封装对简道云API的请求处理
- 提供项目查询和设备查询等功能
- 支持分页获取大量数据
- 处理认证和错误情况
*/

// 简道云API基础常量
const API_BASE_URL: &str = "https://api.jiandaoyun.com/api/v5"; // 简道云API基础URL
const API_KEY: &str = "WuVMLm7r6s1zzFTkGyEYXQGxEZ9mLj3h"; // API访问密钥

// 应用和表单ID常量
const APP_ID: &str = "67d13e0bb840cdf11eccad1e"; // 应用ID："深化设计（B1）"
const ENTRY_ID: &str = "67d7f0ed97abe5bfc70d8aed"; // 表单ID："深化设计（B1）"

/// 字段名称常量，映射简道云表单中的字段标识符
struct FieldNames;
impl FieldNames {
    // 项目相关字段
    const PROJECT_NAME: &'static str = "_widget_1635777114903"; // 项目名称（text类型）
    const PROJECT_NUMBER: &'static str = "_widget_1635777114935"; // 项目编号（text类型）
    const DESIGN_NUMBER: &'static str = "_widget_1636359817201"; // 深化设计编号（text类型）
    const CUSTOMER_NAME: &'static str = "_widget_1635777114972"; // 客户名称（text类型）
    const STATION_NAME: &'static str = "_widget_1635777114991"; // 场站（text类型）

    // 设备相关字段（子表单类型）
    const EQUIPMENT_LIST: &'static str = "_widget_1635777115095"; // 深化清单（subform类型）- 包含设备清单

    // 设备子表单中的字段
    const EQUIPMENT_NAME: &'static str = "_widget_1635777115211"; // 设备名称（text类型）
    const BRAND: &'static str = "_widget_1635777115248"; // 品牌（text类型）
    const MODEL: &'static str = "_widget_1635777115287"; // 规格型号（text类型）
    const TECH_PARAM: &'static str = "_widget_1641439264111"; // 技术参数（text类型）
    const QUANTITY: &'static str = "_widget_1635777485580"; // 数量（number类型）
    const UNIT: &'static str = "_widget_1654703913698"; // 单位（text类型）
    const EXTERNAL_PARAM: &'static str = "_widget_1641439463480"; // 技术参数(外部)（text类型）

    // 其他可能需要的字段...后期有需要可增加
}

/// 简道云查询构建器 - 用于构建API请求参数
struct JiandaoyunQueryBuilder {
    app_id: String,      // 应用ID
    entry_id: String,    // 表单ID
    data_id: String,     // 数据ID（用于分页）
    fields: Vec<String>, // 要查询的字段列表
    filter: Value,       // 过滤条件
    limit: u32,          // 每页数据量限制
}
impl JiandaoyunQueryBuilder {
    /// 创建新的查询构建器
    ///
    /// # 参数
    /// * `app_id` - 应用ID
    /// * `entry_id` - 表单ID
    fn new(app_id: &str, entry_id: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            entry_id: entry_id.to_string(),
            data_id: String::new(),
            fields: Vec::new(),
            filter: json!({
                "rel": "and",
                "cond": []
            }),
            limit: 100,
        }
    }

    /// 添加多个字段到查询
    fn add_fields(mut self, fields: &[&str]) -> Self {
        for field in fields {
            self.fields.push(field.to_string());
        }
        self
    }

    /// 设置分页data_id
    fn with_data_id(mut self, data_id: &str) -> Self {
        self.data_id = data_id.to_string();
        self
    }

    fn add_filter_condition(mut self, field: &str, value: &str) -> Self {
        let condition = json!({
            "field": field,
            "type": "String",
            "method": "eq",
            "value": [value]
        });

        let conds = self.filter["cond"].as_array_mut().unwrap();
        conds.push(condition);

        self
    }

    /// 构建最终的查询参数
    fn build(&self) -> Value {
        json!({
            "app_id": self.app_id,
            "entry_id": self.entry_id,
            "data_id": self.data_id,
            "fields": self.fields,
            "filter": self.filter,
            "limit": self.limit
        })
    }
}

/// 简道云数据查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataQueryResponse {
    pub data: Vec<Value>, // 查询结果数据数组
}

/// 简道云API客户端
pub struct JiandaoyunApiClient {
    client: reqwest::Client, // HTTP客户端
}

impl JiandaoyunApiClient {
    /// 创建新的API客户端
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// 准备HTTP请求头
    fn prepare_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", API_KEY);

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&auth_value)
                .map_err(|e| anyhow!("创建Authorization头失败: {}", e))?,
        );

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(headers)
    }

    /// 通用的分页查询方法
    ///
    /// 处理分页逻辑，自动获取所有满足条件的数据
    async fn paginated_query(
        &self,
        mut query_builder: JiandaoyunQueryBuilder,
    ) -> Result<DataQueryResponse> {
        let url = format!("{}/app/entry/data/list", API_BASE_URL);
        // 提前准备headers，只准备一次
        let headers = self.prepare_headers()?;

        let mut all_data = vec![];
        let mut last_data_id = String::new();
        let limit = query_builder.limit;

        loop {
            // 更新查询的data_id用于分页
            query_builder = query_builder.with_data_id(&last_data_id);
            let payload = query_builder.build();

            let response = self
                .client
                .post(&url)
                .headers(headers.clone()) // reqwest需要HeaderMap值而不是引用，所以我们仍需克隆，但至少只在循环内部进行
                .json(&payload)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(anyhow!("API请求失败: {}", response.status()));
            }

            let response_text = response.text().await?;

            let page_data: DataQueryResponse = match serde_json::from_str(&response_text) {
                Ok(data) => data,
                Err(err) => {
                    return Err(anyhow!("解析响应失败: {}", err));
                }
            };

            // 如果没有数据或者数据量小于limit，说明已经查询完毕
            if page_data.data.is_empty() || page_data.data.len() < limit as usize {
                all_data.extend(page_data.data);
                break;
            }

            // 获取最后一条数据的_id作为下一次查询的起点
            if let Some(last_item) = page_data.data.last() {
                if let Some(id) = last_item.get("_id") {
                    if let Some(id_str) = id.as_str() {
                        last_data_id = id_str.to_string();
                        all_data.extend(page_data.data);
                    } else {
                        return Err(anyhow!("无法获取数据ID"));
                    }
                } else {
                    return Err(anyhow!("数据中缺少_id字段"));
                }
            } else {
                break;
            }
        }

        Ok(DataQueryResponse { data: all_data })
    }

    /// 通过项目编号查询数据
    ///
    /// # 参数
    /// * `project_number` - 项目编号，可选，默认为"OPP.23110200272"
    pub async fn query_by_project_number(
        &self,
        project_number: Option<String>,
    ) -> Result<DataQueryResponse> {
        let project_number = project_number.unwrap_or_else(|| "OPP.23110200272".to_string());

        let query_builder = JiandaoyunQueryBuilder::new(APP_ID, ENTRY_ID)
            .add_fields(&[
                FieldNames::PROJECT_NAME,   // 项目名称
                FieldNames::PROJECT_NUMBER, // 项目编号
                FieldNames::DESIGN_NUMBER,  // 深化设计编号
                FieldNames::CUSTOMER_NAME,  // 客户名称
                FieldNames::STATION_NAME,   // 场站
            ])
            .add_filter_condition(FieldNames::PROJECT_NUMBER, &project_number);

        self.paginated_query(query_builder).await
    }

    /// 根据场站名称查询设备清单
    ///
    /// # 参数
    /// * `station_name` - 场站名称
    pub async fn query_equipment_by_station(
        &self,
        station_name: String,
    ) -> Result<DataQueryResponse> {
        let query_builder = JiandaoyunQueryBuilder::new(APP_ID, ENTRY_ID)
            .add_fields(&[
                FieldNames::EQUIPMENT_LIST, // 深化清单(子表单类型)
                FieldNames::EQUIPMENT_NAME, // 设备名称
                FieldNames::BRAND,          // 品牌
                FieldNames::MODEL,          // 规格型号
                FieldNames::TECH_PARAM,     // 技术参数
                FieldNames::QUANTITY,       // 数量
                FieldNames::UNIT,           // 单位
                FieldNames::EXTERNAL_PARAM, // 技术参数(外部)
            ])
            .add_filter_condition(FieldNames::STATION_NAME, &station_name);

        self.paginated_query(query_builder).await
    }
}

/// 创建一个默认客户端的快捷函数
pub fn create_jiandaoyun_client() -> JiandaoyunApiClient {
    JiandaoyunApiClient::new()
}
