use std::error::Error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct CubeFace {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

pub fn extract_json<T>(json: &str) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
{
    let json_value: Value = serde_json::from_str(json)?;
    let v: T = serde_json::from_value(json_value)?;

    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_cube_face() {
        let json_data = r#"
            [
              {
                "name": "Down",
                "rows": [
                  ["W", "W", "W"],
                  ["W", "W", "W"],
                  ["W", "W", "W"]
                ]
              },
              {
                "name": "Left",
                "rows": [
                  ["R", "R", "R"],
                  ["R", "R", "R"],
                  ["R", "R", "R"]
                ]
              },
              {
                "name": "Front",
                "rows": [
                  ["B", "B", "B"],
                  ["B", "B", "B"],
                  ["B", "B", "B"]
                ]
              },
              {
                "name": "Right",
                "rows": [
                  ["O", "O", "O"],
                  ["O", "O", "O"],
                  ["O", "O", "O"]
                ]
              },
              {
                "name": "Back",
                "rows": [
                  ["G", "G", "G"],
                  ["G", "G", "G"],
                  ["G", "G", "G"]
                ]
              },
              {
                "name": "Top",
                "rows": [
                  ["Y", "Y", "Y"],
                  ["Y", "Y", "Y"],
                  ["Y", "Y", "Y"]
                ]
              }
            ]
        "#;

        let result: Result<Vec<CubeFace>, Box<dyn Error>> = extract_json(json_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 6);
    }
}
