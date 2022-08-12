use anyhow::anyhow;
use serde::Serialize;
use serde_json::{Map, Value};

/// 複数のJSONを1つにまとめる
pub struct JsonBuilder {
    node: Map<String, Value>,
}

impl JsonBuilder {
    /// コンストラクタ
    pub fn new() -> Self {
        Self { node: Map::new() }
    }

    /// Node追加
    pub fn add<T: Serialize>(mut self, value: T) -> anyhow::Result<Self> {
        let value = serde_json::to_value(value)?;
        let map = if let Value::Object(map) = value {
            map
        } else {
            return Err(anyhow!(
                "The value cannot be converted to json object map ({:?})",
                value
            ));
        };

        self.node.extend(map.into_iter());

        Ok(self)
    }

    /// JSON生成
    pub fn build(self) -> Value {
        Value::Object(self.node)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use super::JsonBuilder;

    #[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
    struct ErrorInfo {
        code: i32,
        description: String,
        detail: String,
    }

    #[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
    struct Errors {
        occured: Vec<ErrorInfo>,
    }

    #[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
    struct UserInfo {
        name: String,
        authenticated: bool,
        other: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
    struct All {
        name: String,
        authenticated: bool,
        other: HashMap<String, String>,
        occured: Vec<ErrorInfo>,
    }

    #[test]
    fn combine_structs_into_json() -> anyhow::Result<()> {
        let error1 = ErrorInfo {
            code: 1,
            description: "Something is wrong".to_string(),
            detail: "for more information here...".to_string(),
        };
        let error2 = ErrorInfo {
            code: 2,
            description: "NG".into(),
            detail: "http://www.com".into(),
        };
        let errors = Errors {
            occured: vec![error1, error2],
        };
        let mut user = UserInfo {
            name: "David".to_string(),
            authenticated: false,
            other: HashMap::new(),
        };
        user.other
            .insert("address".to_string(), 12345678.to_string());

        let json = JsonBuilder::new()
            .add(errors.clone())?
            .add(user.clone())?
            .build();
        let json: All = serde_json::from_value(json)?;
        let expected = All {
            name: user.name,
            authenticated: user.authenticated,
            other: user.other,
            occured: errors.occured,
        };

        assert_eq!(json, expected);

        Ok(())
    }
}
