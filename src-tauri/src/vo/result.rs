// use type_build::TS;

use type_build::TS;

pub enum ResultCode {
    Fail(i8),
    Success(i8),
}

#[TS(name = "result")]
pub struct ResultBody {
    code: ResultCode,
    data: String,

}