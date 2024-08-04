pub mod encoding;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum Symbol {
    Space,
    ArabicAlif,
    ArabicBa,
    ArabicTa,
    ArabicTha,
    ArabicJeem,
    ArabicHHa,
    ArabicKha,
    ArabicDal,
    ArabicThal,
    ArabicRa,
    ArabicZa,
    ArabicSeen,
    ArabicSheen,
    ArabicSod,
    ArabicDod,
    ArabicToh,
    ArabicDhoh,
    ArabicEin,
    ArabicGein,
    ArabicFa,
    ArabicQof,
    ArabicKef,
    ArabicLam,
    ArabicMeem,
    ArabicNun,
    ArabicHa,
    ArabicWaw,
    ArabicYa,

    ArabicHamza,
    ArabicFathha,
    ArabicKusra,
    ArabicDomma,

    ArabicDaggerAlif,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LookUpEntry {
    pub code: u32,
    pub utf8_char: char,
    pub is_diacritic: bool,
    pub variant: Symbol
}

#[derive(Debug, Error)]
#[error("Invalid symbol code")]
pub struct InvalidCodeError;

impl Symbol {
    pub const SPACE             : LookUpEntry = LookUpEntry { code: 0 , utf8_char: ' ',          is_diacritic: false, variant: Symbol::Space            };
    pub const ARABIC_ALIF       : LookUpEntry = LookUpEntry { code: 1 , utf8_char: 'ا',          is_diacritic: false, variant: Symbol::ArabicAlif       };
    pub const ARABIC_BA         : LookUpEntry = LookUpEntry { code: 2 , utf8_char: 'ب',          is_diacritic: false, variant: Symbol::ArabicBa         };
    pub const ARABIC_TA         : LookUpEntry = LookUpEntry { code: 3 , utf8_char: 'ت',          is_diacritic: false, variant: Symbol::ArabicTa         };
    pub const ARABIC_THA        : LookUpEntry = LookUpEntry { code: 4 , utf8_char: 'ث',          is_diacritic: false, variant: Symbol::ArabicTha        };
    pub const ARABIC_JEEM       : LookUpEntry = LookUpEntry { code: 5 , utf8_char: 'ج',          is_diacritic: false, variant: Symbol::ArabicJeem       };
    pub const ARABIC_HHA        : LookUpEntry = LookUpEntry { code: 6 , utf8_char: 'ح',          is_diacritic: false, variant: Symbol::ArabicHHa        };
    pub const ARABIC_KHA        : LookUpEntry = LookUpEntry { code: 7 , utf8_char: 'خ',          is_diacritic: false, variant: Symbol::ArabicKha        };
    pub const ARABIC_DAL        : LookUpEntry = LookUpEntry { code: 8 , utf8_char: 'د',          is_diacritic: false, variant: Symbol::ArabicDal        };
    pub const ARABIC_THAL       : LookUpEntry = LookUpEntry { code: 9 , utf8_char: 'ذ',          is_diacritic: false, variant: Symbol::ArabicThal       };
    pub const ARABIC_RA         : LookUpEntry = LookUpEntry { code: 10, utf8_char: 'ر',          is_diacritic: false, variant: Symbol::ArabicRa         };
    pub const ARABIC_ZA         : LookUpEntry = LookUpEntry { code: 11, utf8_char: 'ز',          is_diacritic: false, variant: Symbol::ArabicZa         };
    pub const ARABIC_SEEN       : LookUpEntry = LookUpEntry { code: 12, utf8_char: 'س',          is_diacritic: false, variant: Symbol::ArabicSeen       };
    pub const ARABIC_SHEEN      : LookUpEntry = LookUpEntry { code: 13, utf8_char: 'ش',          is_diacritic: false, variant: Symbol::ArabicSheen      };
    pub const ARABIC_SOD        : LookUpEntry = LookUpEntry { code: 14, utf8_char: 'ص',          is_diacritic: false, variant: Symbol::ArabicSod        };
    pub const ARABIC_DOD        : LookUpEntry = LookUpEntry { code: 15, utf8_char: 'ض',          is_diacritic: false, variant: Symbol::ArabicDod        };
    pub const ARABIC_TOH        : LookUpEntry = LookUpEntry { code: 16, utf8_char: 'ط',          is_diacritic: false, variant: Symbol::ArabicToh        };
    pub const ARABIC_DHOH       : LookUpEntry = LookUpEntry { code: 17, utf8_char: 'ظ',          is_diacritic: false, variant: Symbol::ArabicDhoh       };
    pub const ARABIC_EIN        : LookUpEntry = LookUpEntry { code: 18, utf8_char: 'ع',          is_diacritic: false, variant: Symbol::ArabicEin        };
    pub const ARABIC_GEIN       : LookUpEntry = LookUpEntry { code: 19, utf8_char: 'غ',          is_diacritic: false, variant: Symbol::ArabicGein       };
    pub const ARABIC_FA         : LookUpEntry = LookUpEntry { code: 20, utf8_char: 'ف',          is_diacritic: false, variant: Symbol::ArabicFa         };
    pub const ARABIC_QOF        : LookUpEntry = LookUpEntry { code: 21, utf8_char: 'ق',          is_diacritic: false, variant: Symbol::ArabicQof        };
    pub const ARABIC_KEF        : LookUpEntry = LookUpEntry { code: 22, utf8_char: 'ك',          is_diacritic: false, variant: Symbol::ArabicKef        };
    pub const ARABIC_LAM        : LookUpEntry = LookUpEntry { code: 23, utf8_char: 'ل',          is_diacritic: false, variant: Symbol::ArabicLam        };
    pub const ARABIC_MEEM       : LookUpEntry = LookUpEntry { code: 24, utf8_char: 'م',          is_diacritic: false, variant: Symbol::ArabicMeem       };
    pub const ARABIC_NUN        : LookUpEntry = LookUpEntry { code: 25, utf8_char: 'ن',          is_diacritic: false, variant: Symbol::ArabicNun        };
    pub const ARABIC_HA         : LookUpEntry = LookUpEntry { code: 26, utf8_char: 'ه',          is_diacritic: false, variant: Symbol::ArabicHa         };
    pub const ARABIC_WAW        : LookUpEntry = LookUpEntry { code: 27, utf8_char: 'و',          is_diacritic: false, variant: Symbol::ArabicWaw        };
    pub const ARABIC_YA         : LookUpEntry = LookUpEntry { code: 28, utf8_char: 'ي',          is_diacritic: false, variant: Symbol::ArabicYa         };
    pub const ARABIC_HAMZA      : LookUpEntry = LookUpEntry { code: 29, utf8_char: 'ء',          is_diacritic: false, variant: Symbol::ArabicHamza      };
    pub const ARABIC_FATHHA     : LookUpEntry = LookUpEntry { code: 30, utf8_char: '\u{064E}',   is_diacritic: true,  variant: Symbol::ArabicFathha     };
    pub const ARABIC_KUSRA      : LookUpEntry = LookUpEntry { code: 31, utf8_char: '\u{0650}',   is_diacritic: true,  variant: Symbol::ArabicKusra      };
    pub const ARABIC_DOMMA      : LookUpEntry = LookUpEntry { code: 32, utf8_char: '\u{064F}',   is_diacritic: true,  variant: Symbol::ArabicDomma      };
    pub const ARABIC_DAGGER_ALIF: LookUpEntry = LookUpEntry { code: 33, utf8_char: '\u{0670}',   is_diacritic: false, variant: Symbol::ArabicDaggerAlif };

    pub const LOOKUP_TABLE: [LookUpEntry; 34] = [
        Self::SPACE,
        Self::ARABIC_ALIF,
        Self::ARABIC_BA,
        Self::ARABIC_TA,
        Self::ARABIC_THA,
        Self::ARABIC_JEEM,
        Self::ARABIC_HHA,
        Self::ARABIC_KHA,
        Self::ARABIC_DAL,
        Self::ARABIC_THAL,
        Self::ARABIC_RA,
        Self::ARABIC_ZA,
        Self::ARABIC_SEEN,
        Self::ARABIC_SHEEN,
        Self::ARABIC_SOD,
        Self::ARABIC_DOD,
        Self::ARABIC_TOH,
        Self::ARABIC_DHOH,
        Self::ARABIC_EIN,
        Self::ARABIC_GEIN,
        Self::ARABIC_FA,
        Self::ARABIC_QOF,
        Self::ARABIC_KEF,
        Self::ARABIC_LAM,
        Self::ARABIC_MEEM,
        Self::ARABIC_NUN,
        Self::ARABIC_HA,
        Self::ARABIC_WAW,
        Self::ARABIC_YA,
        Self::ARABIC_HAMZA,
        Self::ARABIC_FATHHA,
        Self::ARABIC_KUSRA,
        Self::ARABIC_DOMMA,
        Self::ARABIC_DAGGER_ALIF,
    ];
    
    pub fn to_utf8_char(self) -> char {
        Self::LOOKUP_TABLE.get(self as usize).unwrap().utf8_char
    }
}