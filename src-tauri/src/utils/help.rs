use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now() -> usize {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as _
}

/// generate the uid
pub fn get_uid(prefix: &str) -> String {
  let now = get_now();
  format!("{prefix}{now}")
}

/// parse the string
/// xxx=123123; => 123123
pub fn parse_str<T: FromStr>(target: &str, key: &str) -> Option<T> {
  match target.find(key) {
    Some(idx) => {
      let idx = idx + key.len();
      let value = &target[idx..];
      match match value.split(';').nth(0) {
        Some(value) => value.trim().parse(),
        None => value.trim().parse(),
      } {
        Ok(r) => Some(r),
        Err(_) => None,
      }
    }
    None => None,
  }
}

#[macro_export]
macro_rules! log_if_err {
  ($result: expr) => {
    if let Err(err) = $result {
      log::error!("{err}");
    }
  };
}

/// wrap the anyhow error
/// transform the error to String
#[macro_export]
macro_rules! wrap_err {
  ($stat: expr) => {
    match $stat {
      Ok(a) => Ok(a),
      Err(err) => {
        log::error!("{}", err.to_string());
        Err(format!("{}", err.to_string()))
      }
    }
  };
}

/// return the string literal error
#[macro_export]
macro_rules! ret_err {
  ($str: literal) => {
    return Err($str.into())
  };
}

#[test]
fn test_parse_value() {
  let test_1 = "upload=111; download=2222; total=3333; expire=444";
  let test_2 = "attachment; filename=Clash.yaml";

  assert_eq!(parse_str::<usize>(test_1, "upload=").unwrap(), 111);
  assert_eq!(parse_str::<usize>(test_1, "download=").unwrap(), 2222);
  assert_eq!(parse_str::<usize>(test_1, "total=").unwrap(), 3333);
  assert_eq!(parse_str::<usize>(test_1, "expire=").unwrap(), 444);
  assert_eq!(
    parse_str::<String>(test_2, "filename=").unwrap(),
    format!("Clash.yaml")
  );

  assert_eq!(parse_str::<usize>(test_1, "aaa="), None);
  assert_eq!(parse_str::<usize>(test_1, "upload1="), None);
  assert_eq!(parse_str::<usize>(test_1, "expire1="), None);
  assert_eq!(parse_str::<usize>(test_2, "attachment="), None);
}
