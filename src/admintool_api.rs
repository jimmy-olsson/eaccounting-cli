use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct FeatureFlagResponse {
    pub name: String,
    pub status: String,
}

pub fn get_feature_flags() -> Vec<FeatureFlagResponse> {
    let mut result: Vec<FeatureFlagResponse> = Vec::new();
    let url = "http://192.168.1.51/admintool/api/featureflags/GetFeatureFlagsWithFilters/CP";

    match reqwest::blocking::Client::new()
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

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct CreateFeatureFilterRequest {
    pub flag_name: String,
    pub rule: String,
    pub pattern: String,
    pub countries: Vec<String>,
    pub deactivate_redundant_filters: bool,
}

pub fn enable_feature_flag(flag_name: &String) -> bool {
    let url = "http://192.168.1.51/admintool/api/featureflags/createFeatureFilter";

    let request_json = CreateFeatureFilterRequest {
        flag_name: flag_name.to_string(),
        rule: "MatchAllCustomersInAllCountries".into(),
        pattern: "*".into(),
        countries: vec!["SE".into(), "NO".into(), "NL".into(), "FI".into()],
        deactivate_redundant_filters: true,
    };

    let response = reqwest::blocking::Client::new()
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
