use failure::Fail;
pub use failure::Fallible;

pub const SERVER_PORT: u16 = 50051;

#[derive(Fail, Debug)]
#[fail(display = "Option error")]
pub struct OptionError;

pub trait OkOrErr<T> {
    fn ok_or_err(self) -> Fallible<T>;
}

impl<T> OkOrErr<T> for Option<T> {
    fn ok_or_err(self) -> Fallible<T> {
        self.ok_or_else(|| OptionError.into())
    }
}
