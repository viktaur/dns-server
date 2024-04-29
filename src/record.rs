pub enum Record {
    UNKNOWN {

    },
    A {
        domain: String,

    },
    AAAA {
        domain: String,

    },
    MX {
        hello: u8
    },
    TXT {

    }
}

pub enum QueryType {
    UNKNOWN,
    A,
    AAAA,
    MX,
    TXT
}
