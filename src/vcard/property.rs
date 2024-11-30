pub enum Property {
    Version(String),
    Name(Name),
    FormattedName(String),
    Address(Address),
    Birthday(u16, u8, u8), // year, month, day
    Email(Email),
    Geolocation(f64, f64),
    Label(Label),
    Language(String),
    Note(String),
    Organization(String),
    Revision(u16, u8, u8), // year, month, day
    Role(String),
    Title(String),
    Timezone(String),
    UniqueId(String),
    Url(String),
    X(String, String),
}

pub struct Name {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub additional_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
}

pub struct Address {
    pub type_: Option<AddressType>,
    pub post_office_address: Option<String>,
    pub extended_address: Option<String>,
    pub street: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

pub enum AddressType {
    Domestic,
    International,
    Postal,
    Parcel,
    Home,
    Work,
    Other(String),
}

pub struct Email {
    pub type_: Option<EmailType>,
    pub email: String,
}

pub enum EmailType {
    Aol,
    AppleLink,
    AttMail,
    Cis,
    EWorld,
    Internet,
    IBMMail,
    MCIMail,
    Powershare,
    Prodigy,
    Tlx,
    X400,
    Other(String),
}

pub struct Label {
    pub type_: AddressType,
    pub text: String,
}
