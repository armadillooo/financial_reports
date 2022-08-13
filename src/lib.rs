#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    struct Serialization {
        name: String,
        age: i32,
    }

    #[test]
    fn a() -> anyhow::Result<()> {
        let s = Serialization {
            name: "hello".to_string(),
            age: 45,
        };

        let json = serde_json::to_value(s)?;
        let map = match json {
            Value::Object(map) => map,
            _ => return Err(anyhow!("Json couldn't convert to map object")),
        };

        Ok(())
    }
}
