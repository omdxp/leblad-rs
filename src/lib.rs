#![allow(unused)]
use _auto_generated::{Baladyia, Daira, Wilaya, ALL_WILAYAS};

mod _auto_generated;
mod utils;

pub fn get_wilaya_list() -> Vec<Wilaya> {
    ALL_WILAYAS.to_vec()
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
}
