 use ckb_std::error::SysError;
#[derive(Debug)]
// #[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Error {
    IndexOutOfBound = 1,
    LengthNotEnough,
    ItemMissing,
    Encoding,

    InvalidTypeIDCellNum,
    TypeIDNotMatch,


    InvalidOracleCellData,
    IncorrectCellAmount,
    IncorrectCellInputAmount,
    IncorrectCellOutputAmount,
    WrongOracleData,

    Unknown,
}

 impl From<SysError> for Error {
     fn from(err: SysError) -> Self {
         use SysError::*;
         match err {
             IndexOutOfBound=> Self::IndexOutOfBound,
             ItemMissing=> Self::InvalidTypeIDCellNum,
             LengthNotEnough(_)=> Self::LengthNotEnough,
             Encoding=> Self::Encoding,

             Unknown(err) => panic!("expected err: {}", err),
             // _ => panic!("expected err"),
         }
     }
 }