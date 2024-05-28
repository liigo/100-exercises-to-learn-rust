// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)] // Liigo: this is required by Result::unwrap()
struct StatusError;

impl TryFrom<&str> for Status {
    type Error = StatusError;

    fn try_from(value: &str) -> Result<Status, Self::Error> {
        // 官方答案是先得到全小写字符然后match（需要创建String对象，多了内存分配）
        // let value = value.to_lowercase();
        if value.eq_ignore_ascii_case("ToDo") {
            return Ok(Status::ToDo);
        } else if value.eq_ignore_ascii_case("InProgress") {
            return Ok(Status::InProgress);
        } else if value.eq_ignore_ascii_case("Done") {
            return Ok(Status::Done);
        } else {
            return Err(StatusError);
        }
    }
}

impl TryFrom<String> for Status {
    type Error = StatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test] // liigo added this test
    fn test_try_from_bad_string() {
        let status = Status::try_from("bad stirng");
        assert!(status.is_err());
    }

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
