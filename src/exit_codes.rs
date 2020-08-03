#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExitCode {
    Success,
    GeneralError,
    KilledBySigint,
}

impl From<ExitCode> for i32 {
    fn from(item: ExitCode) -> Self {
        use ExitCode::*;

        match item {
            Success => 0,
            GeneralError => 1,
            KilledBySigint => 130,
        }
    }
}
