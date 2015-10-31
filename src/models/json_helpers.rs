use rustc_serialize::json::Json;

/// Given a Json object, check to see if it's a string. If it is, then clone and return that
/// string.
pub struct JsonHelper;

impl JsonHelper {
    #[inline]
    pub fn string_or_empty(s: Option<&Json>) -> String {
        match s {
            Some(v) => match v {
                &Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        }
    }

    #[inline]
    pub fn number_or_zero(s: Option<&Json>) -> u64 {
        match s {
            Some(v) => match v {
                &Json::U64(num) => num,
                _ => 0,
            },
            None => 0,
        }
    }

    #[inline]
    pub fn boolean_or_false(s: Option<&Json>) -> bool {
        match s {
            Some(v) => match v {
                &Json::Boolean(b) => b,
                _ => false,
            },
            None => false,
        }
    }
}
