use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};

const JDY_API_BASE_URL: &str = "https://api.jiandaoyun.com/api/v5";
const JDY_API_KEY: &str = "WuVMLm7r6s1zzFTkGyEYXQGxEZ9mLj3h";

// 简道云数据查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct JdyDataQueryResponse {
    pub data: Vec<Value>,
}

pub struct JdyApiClient {
    client: reqwest::Client,
}

impl JdyApiClient {
    // 创建新的API客户端
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    // 准备请求头
    fn prepare_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", JDY_API_KEY);
        headers.insert("Authorization", HeaderValue::from_str(&auth_value).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    // 通过项目编号查询数据
    pub async fn query_by_project_number(&self, project_number: Option<String>) -> Result<JdyDataQueryResponse> {
        let project_number = project_number.unwrap_or_else(|| "OPP.23110200272".to_string());
        let url = format!("{}/app/entry/data/list", JDY_API_BASE_URL);
        let headers = self.prepare_headers();
        
        let payload = json!({
            "app_id": "67d13e0bb840cdf11eccad1e",
            "entry_id": "67d7f0ed97abe5bfc70d8aed",
            "data_id": "",
            "fields": [
                "_widget_1635777114903",
                "_widget_1635777114935",
                "_widget_1636359817201",
                "_widget_1635777114972",
                "_widget_1635777114991"
            ],
            "filter": {
                "rel": "and",
                "cond": [
                    {
                        "field": "_widget_1635777114935",
                        "type": "String",
                        "method": "eq",
                        "value": [project_number]
                    }
                ]
            },
            "limit": 100
        });
        
        let response = self.client
            .post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("API请求失败: {}", response.status()));
        }
        
        let data = response.json::<JdyDataQueryResponse>().await?;
        Ok(data)
    }
   
}

// 创建一个默认客户端的快捷函数
pub fn create_jdy_client() -> JdyApiClient {
    JdyApiClient::new()
}
