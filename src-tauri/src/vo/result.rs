pub enum ResultCode {
    Fail(i8),
    Success(i8),
}

pub struct ResultBody {
    code: ResultCode,
    data: String,
}