use serde::Deserialize;
use serde::Serialize;
use reqwest::blocking::Client;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeatureFlagResponse {
    pub name: String,
    pub status: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateFeatureFilterRequest {
    flag_name: String,
    rule: String,
    pattern: String,
    countries: Vec<String>,
    deactivate_redundant_filters: bool,
}

pub struct AdmintoolApi {
    client: Client,
    base_url: String,
}

impl AdmintoolApi {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url
        }
    }

    pub fn get_feature_flags(&self) -> Vec<FeatureFlagResponse> {
        let url = format!("{}/featureflags/GetFeatureFlagsWithFilters/CP", self.base_url);
        let mut result: Vec<FeatureFlagResponse> = Vec::new();
    
        match self.client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
        {
            Ok(r) => {
                let text = r.text().unwrap();
                result = serde_json::from_str(&text).unwrap();
            }
            Err(err) => {
                println!("Request failed: {}", err.to_string());
            }
        };
    
        return result;
    }

    pub fn enable_feature_flag(&self, flag_name: &String) -> bool {
        let url = format!("{}/featureflags/createFeatureFilter", self.base_url);

        let request_json = CreateFeatureFilterRequest {
            flag_name: flag_name.to_string(),
            rule: "MatchAllCustomersInAllCountries".into(),
            pattern: "*".into(),
            countries: vec!["SE".into(), "NO".into(), "NL".into(), "FI".into()],
            deactivate_redundant_filters: true,
        };
    
        let response = self.client
            .post(url)
            .json(&request_json)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send();
    
        if response.is_err() {
            println!("Request failed: {}", response.err().unwrap().to_string());
            return false;
        }
    
        return true;
    }
}





