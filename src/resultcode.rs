use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ResultCode {
    #[deku(id = 0)]
    NOERROR,
    #[deku(id = 1)]
    FORMERR,
    #[deku(id = 2)]
    SERVFAIL,
    #[deku(id = 3)]
    NXDOMAIN,
    #[deku(id = 4)]
    NOTIMP,
    #[deku(id = 5)]
    REFUSED,
}
