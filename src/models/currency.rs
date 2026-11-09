use strum::{Display, EnumString};

#[derive(sqlx::Type,Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum Currency {
    // North America
    /// Canadian Dollar
    CAD,
    /// United States Dollar
    USD,
    /// Mexican Peso
    MXN,

    // Central America & Caribbean
    /// Costa Rican Colón
    CRC,
    /// Guatemalan Quetzal
    GTQ,
    /// Honduran Lempira
    HNL,
    /// Nicaraguan Córdoba
    NIO,
    /// Panamanian Balboa
    PAB,
    /// Dominican Peso
    DOP,
    /// Jamaican Dollar
    JMD,
    /// Trinidad and Tobago Dollar
    TTD,
    /// Bahamian Dollar
    BSD,
    /// Barbadian Dollar
    BBD,
    /// Haitian Gourde
    HTG,

    // South America
    /// Brazilian Real
    BRL,
    /// Argentine Peso
    ARS,
    /// Chilean Peso
    CLP,
    /// Colombian Peso
    COP,
    /// Peruvian Sol
    PEN,
    /// Uruguayan Peso
    UYU,
    /// Bolivian Boliviano
    BOB,
    /// Paraguayan Guaraní
    PYG,
    /// Venezuelan Bolívar
    VES,

    // Western & Northern Europe
    /// Euro
    EUR,
    /// Great British Pound 
    GBP,
    /// Great British pence 
    GBp,
    /// Swiss Franc
    CHF,
    /// Swedish Krona
    SEK,
    /// Norwegian Krone
    NOK,
    /// Danish Krone
    DKK,
    /// Icelandic Króna
    ISK,

    // Central & Eastern Europe
    /// Polish Złoty
    PLN,
    /// Czech Koruna
    CZK,
    /// Hungarian Forint
    HUF,
    /// Romanian Leu
    RON,
    /// Serbian Dinar
    RSD,
    /// Ukrainian Hryvnia
    UAH,
    /// Russian Ruble
    RUB,
    /// Belarusian Ruble
    BYN,
    /// Moldovan Leu
    MDL,
    /// Albanian Lek
    ALL,
    /// North Macedonian Denar
    MKD,
    /// Bosnia-Herzegovina Convertible Mark
    BAM,

    // Caucasus & Central Asia
    /// Turkish Lira
    TRY,
    /// Georgian Lari
    GEL,
    /// Armenian Dram
    AMD,
    /// Azerbaijani Manat
    AZN,
    /// Kazakhstani Tenge
    KZT,
    /// Uzbekistani Som
    UZS,

    // Middle East
    /// UAE Dirham
    AED,
    /// Saudi Riyal
    SAR,
    /// Qatari Riyal
    QAR,
    /// Kuwaiti Dinar
    KWD,
    /// Bahraini Dinar
    BHD,
    /// Omani Rial
    OMR,
    /// Israeli New Shekel
    ILS,
    /// Jordanian Dinar
    JOD,
    /// Iraqi Dinar
    IQD,
    /// Iranian Rial
    IRR,
    /// Lebanese Pound
    LBP,

    // East Asia
    /// Japanese Yen
    JPY,
    /// Chinese Yuan (Renminbi)
    CNY,
    /// Hong Kong Dollar
    HKD,
    /// South Korean Won
    KRW,
    /// New Taiwan Dollar
    TWD,
    /// Macanese Pataca
    MOP,
    /// Mongolian Tögrög
    MNT,

    // Southeast Asia
    /// Singapore Dollar
    SGD,
    /// Thai Baht
    THB,
    /// Malaysian Ringgit
    MYR,
    /// Indonesian Rupiah
    IDR,
    /// Philippine Peso
    PHP,
    /// Vietnamese Đồng
    VND,
    /// Cambodian Riel
    KHR,
    /// Lao Kip
    LAK,
    /// Myanmar Kyat
    MMK,
    /// Brunei Dollar
    BND,

    // South Asia
    /// Indian Rupee
    INR,
    /// Pakistani Rupee
    PKR,
    /// Bangladeshi Taka
    BDT,
    /// Sri Lankan Rupee
    LKR,
    /// Nepalese Rupee
    NPR,

    // Oceania
    /// Australian Dollar
    AUD,
    /// New Zealand Dollar
    NZD,
    /// Fijian Dollar
    FJD,
    /// Papua New Guinean Kina
    PGK,

    // Africa
    /// South African Rand
    ZAR,
    /// Egyptian Pound
    EGP,
    /// Nigerian Naira
    NGN,
    /// Kenyan Shilling
    KES,
    /// Ghanaian Cedi
    GHS,
    /// Moroccan Dirham
    MAD,
    /// Tunisian Dinar
    TND,
    /// Algerian Dinar
    DZD,
    /// Ethiopian Birr
    ETB,
    /// Tanzanian Shilling
    TZS,
    /// Ugandan Shilling
    UGX,
    /// Rwandan Franc
    RWF,
    /// Zambian Kwacha
    ZMW,
    /// Botswana Pula
    BWP,
    /// Mauritian Rupee
    MUR,
    /// Angolan Kwanza
    AOA,
    /// Namibian Dollar
    NAD,
    /// West African CFA Franc
    XOF,
    /// Central African CFA Franc
    XAF,
}

