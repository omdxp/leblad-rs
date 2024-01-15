#![warn(missing_docs)]
//! leblad
//!
//! Leblad is a library that provides an easy API to get all the information you need for Algerian administrative areas.
//!
//! # Example
//!
//! ```
//! use leblad::get_wilaya_list;
//!
//! let wilayas = get_wilaya_list();
//!
//! assert_eq!(wilayas[0].name, "Adrar");
//! ```

use _auto_generated::ALL_WILAYAS;
pub use _auto_generated::{Baladyia, Daira, Wilaya};

mod _auto_generated;

/// Get all wilayas.
/// # Example
/// ```
/// use leblad::get_wilaya_list;
/// let wilayas = get_wilaya_list();
/// assert_eq!(wilayas[0].name, "Adrar");
/// ```
/// - Results can be filtered by defining a struct that implements From trait:
/// ```
/// use leblad::{get_wilaya_list, Wilaya};
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct FilteredWilaya {
///     pub mattricule: u16,
///     pub name_ar: &'static str,
///     pub name_ber: &'static str,
///     pub name_en: &'static str,
/// }
///
/// impl From<Wilaya> for FilteredWilaya {
///     fn from(wilaya: Wilaya) -> Self {
///         Self {
///             mattricule: wilaya.mattricule,
///             name_ar: wilaya.name_ar,
///             name_ber: wilaya.name_ber,
///             name_en: wilaya.name_en,
///         }
///     }
/// }
///
/// let wilayas = get_wilaya_list();
///
/// let filtered_wilayas: Vec<FilteredWilaya> = wilayas
///     .iter()
///     .map(|e| FilteredWilaya {
///         mattricule: e.mattricule,
///         name_ar: e.name_ar,
///         name_ber: e.name_ber,
///         name_en: e.name_en,
///     })
///     .collect();
///
/// assert_eq!(filtered_wilayas[0].name_en, "Adrar");
/// ```
pub fn get_wilaya_list<'a>() -> &'a [Wilaya] {
    ALL_WILAYAS
}

/// Get wilaya by zip code.
/// # Example
/// ```
/// use leblad::get_wilaya_by_zip_code;
/// let wilaya = get_wilaya_by_zip_code(1_000);
/// assert_eq!(wilaya.unwrap().name, "Adrar");
/// ```
pub fn get_wilaya_by_zip_code<'a>(zip_code: u16) -> Option<&'a Wilaya> {
    ALL_WILAYAS.iter().find(|wilaya| {
        wilaya
            .postal_codes
            .iter()
            .find(|pc| **pc == zip_code)
            .is_some()
    })
}

/// Get wilaya by code.
/// # Example
/// ```
/// use leblad::get_wilaya_by_code;
/// let wilaya = get_wilaya_by_code(1);
/// assert_eq!(wilaya.unwrap().name, "Adrar");
/// ```
pub fn get_wilaya_by_code<'a>(mattricule: u16) -> Option<&'a Wilaya> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.mattricule == mattricule)
}

/// Get adjacent wilayas.
/// # Example
/// ```
/// use leblad::get_adjacent_wilayas;
/// let adjacent_wilayas = get_adjacent_wilayas(1);
/// assert_eq!(adjacent_wilayas.unwrap(), vec![37, 8, 32, 3, 47, 11]);
/// ```
pub fn get_adjacent_wilayas<'a>(mattricule: u16) -> Option<&'a [u16]> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.mattricule == mattricule)
        .map(|wilaya| wilaya.adjacent_wilayas)
}

/// Get zip codes for wilaya.
/// # Example
/// ```
/// use leblad::get_zip_codes_for_wilaya;
/// let zip_codes = get_zip_codes_for_wilaya(1);
/// assert_eq!(zip_codes.unwrap()[0], 1_000);
/// ```
pub fn get_zip_codes_for_wilaya<'a>(mattricule: u16) -> Option<&'a [u16]> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.mattricule == mattricule)
        .map(|wilaya| wilaya.postal_codes)
}

/// Get dairats for wilaya.
/// # Example
/// ```
/// use leblad::get_dairats_for_wilaya;
/// let dairats = get_dairats_for_wilaya(1);
/// assert_eq!(dairats.unwrap()[0].name, "ADRAR");
/// ```
pub fn get_dairats_for_wilaya<'a>(mattricule: u16) -> Option<&'a [Daira]> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.mattricule == mattricule)
        .map(|wilaya| wilaya.dairats)
}

/// Get wilaya by phone code.
/// # Example
/// ```
/// use leblad::get_wilaya_by_phone_code;
/// let wilaya = get_wilaya_by_phone_code(49);
/// assert_eq!(wilaya.unwrap().name, "Adrar");
/// ```
pub fn get_wilaya_by_phone_code<'a>(phone_code: u16) -> Option<&'a Wilaya> {
    ALL_WILAYAS.iter().find(|wilaya| {
        wilaya
            .phone_codes
            .iter()
            .find(|pc| **pc == phone_code)
            .is_some()
    })
}

/// Get wilaya by daira name.
/// # Example
/// ```
/// use leblad::get_wilaya_by_daira_name;
/// let wilaya = get_wilaya_by_daira_name("ADRAR");
/// assert_eq!(wilaya.unwrap().name, "Adrar");
/// ```
pub fn get_wilaya_by_daira_name<'a>(daira_name: &str) -> Option<&'a Wilaya> {
    ALL_WILAYAS.iter().find(|wilaya| {
        wilaya
            .dairats
            .iter()
            .find(|daira| daira.name == daira_name)
            .is_some()
    })
}

/// Get baladyiats for daira.
/// # Example
/// ```
/// use leblad::get_baladyiats_for_daira;
/// let baladyiats = get_baladyiats_for_daira("ADRAR");
/// assert_eq!(baladyiats.unwrap()[0].name, "ADRAR");
/// ```
pub fn get_baladyiats_for_daira(daira_name: &str) -> Option<&[Baladyia]> {
    ALL_WILAYAS.iter().find_map(|wilaya| {
        wilaya.dairats.iter().find_map(|daira| {
            if daira.name == daira_name {
                daira.baladyiats.as_ref().map(|baladyiats| *baladyiats)
            } else {
                None
            }
        })
    })
}

/// Get baladyiats for daira code.
/// # Example
/// ```
/// use leblad::get_baladyiats_for_daira_code;
/// let baladyiats = get_baladyiats_for_daira_code(101);
/// assert_eq!(baladyiats.unwrap()[0].name, "ADRAR");
/// ```
pub fn get_baladyiats_for_daira_code<'a>(daira_code: u16) -> Option<&'a [Baladyia]> {
    ALL_WILAYAS.iter().find_map(|wilaya| {
        wilaya.dairats.iter().find_map(|daira| {
            if daira.code == daira_code {
                daira.baladyiats.as_ref().map(|baladyiats| *baladyiats)
            } else {
                None
            }
        })
    })
}

/// Get phone codes for wilaya.
/// # Example
/// ```
/// use leblad::get_phone_codes_for_wilaya;
/// let phone_codes = get_phone_codes_for_wilaya("Adrar");
/// assert_eq!(phone_codes.unwrap(), vec![49]);
/// ```
pub fn get_phone_codes_for_wilaya(wilaya_name: &str) -> Option<&[u16]> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.name == wilaya_name)
        .map(|wilaya| wilaya.phone_codes)
}

/// Get first phone code for wilaya.
/// # Example
/// ```
/// use leblad::get_first_phone_code_for_wilaya;
/// let first_phone_code = get_first_phone_code_for_wilaya("Adrar");
/// assert_eq!(*first_phone_code.unwrap(), 49);
/// ```
/// # Panics
/// This function panics if somehow the first phone code of a given wilaya was not found.
pub fn get_first_phone_code_for_wilaya(wilaya_name: &str) -> Option<&u16> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.name == wilaya_name)
        .map(|wilaya| {
            wilaya
                .phone_codes
                .first()
                .unwrap_or_else(|| panic!("phone code was not found"))
        })
}

/// Get baladyiats for wilaya.
/// # Example
/// ```
/// use leblad::get_baladyiats_for_wilaya;
/// let baladyiats = get_baladyiats_for_wilaya("Adrar");
/// assert_eq!(baladyiats.unwrap()[0].name, "ADRAR");
/// ```
pub fn get_baladyiats_for_wilaya(wilaya_name: &str) -> Option<Vec<Baladyia>> {
    ALL_WILAYAS
        .iter()
        .find(|wilaya| wilaya.name == wilaya_name)
        .map(|wilaya| {
            wilaya
                .dairats
                .iter()
                .filter_map(|daira| {
                    Some(
                        daira
                            .baladyiats
                            .as_ref()
                            .map(|baladyiats| baladyiats.iter().cloned())
                            .into_iter()
                            .flatten()
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten()
                .collect::<Vec<_>>()
        })
}

/// Get wilaya by baladyia name.
/// # Example
/// ```
/// use leblad::get_wilaya_by_baladyia_name;
/// let wilaya = get_wilaya_by_baladyia_name("ADRAR");
/// assert_eq!(wilaya.unwrap().name, "Adrar");
/// ```
pub fn get_wilaya_by_baladyia_name(baladyia_name: &str) -> Option<&Wilaya> {
    ALL_WILAYAS.iter().find(|wilaya| {
        wilaya
            .dairats
            .iter()
            .find(|daira| {
                daira.baladyiats.as_ref().map_or(false, |baladyiats| {
                    baladyiats
                        .iter()
                        .any(|baladyia| baladyia.name == baladyia_name)
                })
            })
            .is_some()
    })
}

/// Get daira by baladyia name.
/// # Example
/// ```
/// use leblad::get_daira_by_baladyia_name;
/// let daira = get_daira_by_baladyia_name("ADRAR");
/// assert_eq!(daira.unwrap().name, "ADRAR");
/// ```
pub fn get_daira_by_baladyia_name(baladyia_name: &str) -> Option<&Daira> {
    ALL_WILAYAS.iter().find_map(|wilaya| {
        wilaya.dairats.iter().find_map(|daira| {
            daira.baladyiats.as_ref().and_then(|baladyiats| {
                baladyiats
                    .iter()
                    .find(|baladyia| baladyia.name == baladyia_name)
                    .map(|_| daira)
            })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_wilaya_list_without_filters() {
        let res = get_wilaya_list();
        let actual_first_element = res[0].clone();
        let expected_first_element = ALL_WILAYAS[0].clone();
        assert_eq!(res.len(), ALL_WILAYAS.len());
        assert_eq!(actual_first_element, expected_first_element);
    }

    #[test]
    fn get_wilaya_list_with_filters() {
        #[derive(Debug, Clone, PartialEq)]
        struct FilteredWilaya {
            pub mattricule: u16,
            pub name_ar: &'static str,
            pub name_ber: &'static str,
            pub name_en: &'static str,
        }

        impl From<Wilaya> for FilteredWilaya {
            fn from(wilaya: Wilaya) -> Self {
                Self {
                    mattricule: wilaya.mattricule,
                    name_ar: wilaya.name_ar,
                    name_ber: wilaya.name_ber,
                    name_en: wilaya.name_en,
                }
            }
        }

        let res = get_wilaya_list();
        let filtered_res: Vec<FilteredWilaya> = res
            .iter()
            .map(|e| FilteredWilaya {
                mattricule: e.mattricule,
                name_ar: e.name_ar,
                name_ber: e.name_ber,
                name_en: e.name_en,
            })
            .collect();

        let actual_first_element = filtered_res[0].clone();
        let expected_first_element = ALL_WILAYAS[0].clone();
        assert_eq!(res.len(), ALL_WILAYAS.len());
        assert_eq!(actual_first_element, expected_first_element.into());
    }

    #[test]
    fn get_existing_wilaya_by_zip_code() {
        let res = get_wilaya_by_zip_code(1_000);
        assert!(res.is_some());
        assert_eq!(res.unwrap().name, ALL_WILAYAS[0].name);
    }

    #[test]
    fn get_non_existing_wilaya_by_zip_code() {
        let res = get_wilaya_by_zip_code(12_345);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_wilaya_by_code() {
        let res = get_wilaya_by_code(1);
        assert!(res.is_some());
        assert_eq!(res.unwrap().name, ALL_WILAYAS[0].name);
    }

    #[test]
    fn get_non_existing_wilaya_by_code() {
        let res = get_wilaya_by_code(100);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_adjacent_wilayas() {
        let res = get_adjacent_wilayas(1);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), ALL_WILAYAS[0].adjacent_wilayas.to_vec());
    }

    #[test]
    fn get_non_existing_adjacent_wilayas() {
        let res = get_adjacent_wilayas(100);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_zip_codes_for_wilaya() {
        let res = get_zip_codes_for_wilaya(1);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), ALL_WILAYAS[0].postal_codes.to_vec());
    }

    #[test]
    fn get_non_existing_zip_codes_for_wilaya() {
        let res = get_zip_codes_for_wilaya(100);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_dairats_for_wilaya() {
        let res = get_dairats_for_wilaya(1);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), ALL_WILAYAS[0].dairats.to_vec());
    }

    #[test]
    fn get_non_existing_dairats_for_wilaya() {
        let res = get_dairats_for_wilaya(100);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_wilaya_by_phone_code() {
        let res = get_wilaya_by_phone_code(49);
        assert!(res.is_some());
        assert_eq!(res.unwrap().name, ALL_WILAYAS[0].name);
    }

    #[test]
    fn get_non_existing_wilaya_by_phone_code() {
        let res = get_wilaya_by_phone_code(100);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_wilaya_by_daira_name() {
        let res = get_wilaya_by_daira_name("ADRAR");
        assert!(res.is_some());
        assert_eq!(res.unwrap().name, ALL_WILAYAS[0].name);
    }

    #[test]
    fn get_non_existing_wilaya_by_daira_name() {
        let res = get_wilaya_by_daira_name("TIZELABINE");
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_baladyiats_for_daira() {
        let res = get_baladyiats_for_daira("ADRAR");
        assert!(res.is_some());
        let mut baladyiats = vec![];
        for daira in ALL_WILAYAS[0].dairats.iter() {
            if daira.name == "ADRAR" {
                baladyiats = daira.baladyiats.unwrap().to_vec();
            }
        }
        assert_eq!(res.unwrap(), baladyiats);
    }

    #[test]
    fn get_non_existing_baladyiats_for_daira() {
        let res = get_baladyiats_for_daira("TIZELABINE");
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_baladyiats_for_daira_code() {
        let res = get_baladyiats_for_daira_code(101);
        assert!(res.is_some());
        let mut baladyiats = vec![];
        for daira in ALL_WILAYAS[0].dairats.iter() {
            if daira.code == 101 {
                baladyiats = daira.baladyiats.unwrap().to_vec();
            }
        }
        assert_eq!(res.unwrap(), baladyiats);
    }

    #[test]
    fn get_non_existing_baladyiats_for_daira_code() {
        let res = get_baladyiats_for_daira_code(1_000);
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_phone_codes_for_wilaya() {
        let res = get_phone_codes_for_wilaya("Adrar");
        assert!(res.is_some());
        assert_eq!(res.unwrap(), ALL_WILAYAS[0].phone_codes.to_vec());
    }

    #[test]
    fn get_non_existing_phone_codes_for_wilaya() {
        let res = get_phone_codes_for_wilaya("Tizelabine");
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_first_phone_code_for_wilaya() {
        let res = get_first_phone_code_for_wilaya("Adrar");
        assert!(res.is_some());
        assert_eq!(*res.unwrap(), ALL_WILAYAS[0].phone_codes[0]);
    }

    #[test]
    fn get_non_existing_first_phone_code_for_wilaya() {
        let res = get_first_phone_code_for_wilaya("Tizelabine");
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_baladyiats_for_wilaya() {
        let res = get_baladyiats_for_wilaya("Adrar");
        assert!(res.is_some());
        let mut baladyiats = vec![];
        for daira in ALL_WILAYAS[0].dairats.iter() {
            if daira.baladyiats.is_some() {
                for baladyia in daira.baladyiats.unwrap().iter() {
                    baladyiats.push(baladyia.clone());
                }
            }
        }
        assert_eq!(res.unwrap().to_vec(), baladyiats);
    }

    #[test]
    fn get_non_existing_baladyiats_for_wilaya() {
        let res = get_baladyiats_for_wilaya("Tizelabine");
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_wilaya_by_baladyia_name() {
        let res = get_wilaya_by_baladyia_name("OULED AHMED TIMMI");
        assert!(res.is_some());
        assert_eq!(*res.unwrap(), ALL_WILAYAS[0]);
    }

    #[test]
    fn get_non_existing_wilaya_by_baladyia_name() {
        let res = get_wilaya_by_baladyia_name("TIZELABINE");
        assert!(res.is_none());
    }

    #[test]
    fn get_existing_daira_by_baladyia_name() {
        let res = get_daira_by_baladyia_name("OULED AHMED TIMMI");
        assert!(res.is_some());
        assert_eq!(*res.unwrap(), ALL_WILAYAS[0].dairats[0]);
    }

    #[test]
    fn get_non_existing_daira_by_baladyia_name() {
        let res = get_daira_by_baladyia_name("TIZELABINE");
        assert!(res.is_none());
    }
}
