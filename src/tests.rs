#[cfg(test)]
mod tests {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Mine {
        #[serde(skip_serializing_if = "crate::Option3::is_none")]
        #[serde(default = "crate::Option3::default")]
        m: crate::Option3<String>,
    }

    #[test]
    fn serialize_some() {
        let mine = Mine{m: crate::Option3::Some("foo".to_string())};
        let result = serde_json::to_string(&mine).unwrap();
        assert_eq!(r#"{"m":"foo"}"#, result);
    }

    #[test]
    fn serialize_null() {
        
        let mine = Mine{m: crate::Option3::Null};
        let result = serde_json::to_string(&mine).unwrap();
        assert_eq!(r#"{"m":null}"#, result);
    }

    #[test]
    fn serialize_none() {
        let mine = Mine{m: crate::Option3::None};
        let result = serde_json::to_string(&mine).unwrap();
        assert_eq!(r#"{}"#, result);
    }

    #[test]
    fn deserialize_some() {
        let mine = Mine{m: crate::Option3::Some("foo".to_string())};
        let s = r#"{"m":"foo"}"#;
        let result: Mine = serde_json::from_str(s).unwrap();
        assert_eq!(mine, result);
    }

    #[test]
    fn deserialize_none() {
        let mine = Mine{m: crate::Option3::None};
        let s = r#"{}"#;
        let result: Mine = serde_json::from_str(s).unwrap();
        assert_eq!(mine, result);
    }
}
