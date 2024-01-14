use std::fs;

const JSON_URL: &str =
    "https://raw.githubusercontent.com/dzcode-io/leblad/master/data/WilayaList.json";

#[derive(serde::Deserialize)]
pub struct Baladyia {
    pub code: u16,
    pub name: String,
    pub name_en: String,
    pub name_ar: String,
}

impl ToString for Baladyia {
    fn to_string(&self) -> String {
        format!(
            r#"Baladyia {{
    code: {},
    name: "{}",
    name_en: "{}",
    name_ar: "{}",
}}"#,
            self.code, self.name, self.name_en, self.name_ar
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Daira {
    pub code: u16,
    pub name: String,
    pub name_ar: String,
    pub name_en: String,
    pub baladyiats: Option<Vec<Baladyia>>,
}

impl ToString for Daira {
    fn to_string(&self) -> String {
        let mut s = format!(
            r#"Daira {{
    code: {},
    name: "{}",
    name_ar: "{}",
    name_en: "{}","#,
            self.code, self.name, self.name_ar, self.name_en
        );
        if self.baladyiats.is_none() {
            s.push_str(
                r#"
    baladyiats: None,
}"#,
            );
        } else {
            let baladyiats = self.baladyiats.as_ref().unwrap();
            s.push_str(
                format!(
                    r#"
    baladyiats: Some(&[{}]),
}}"#,
                    baladyiats
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
                .as_str(),
            )
        }
        s
    }
}

#[derive(serde::Deserialize)]
pub struct Wilaya {
    pub mattricule: u16,
    pub name_ar: String,
    pub name_ber: String,
    pub name_en: String,
    pub name: String,
    #[serde(rename = "phoneCodes")]
    pub phone_codes: Vec<u16>,
    #[serde(rename = "postalCodes")]
    pub postal_codes: Vec<u16>,
    pub dairats: Vec<Daira>,
    #[serde(rename = "adjacentWilayas")]
    pub adjacent_wilayas: Vec<u16>,
}

pub struct Const {
    pub name: String,
    pub format: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match reqwest::get(JSON_URL).await {
        Ok(res) => {
            let json: serde_json::Value = res.json().await?;
            let wilayas = serde_json::from_value::<Vec<Wilaya>>(json).unwrap();
            std::fs::create_dir_all("./src/_auto_generated")?;
            let mut consts: Vec<Const> = vec![];
            for (i, wilaya) in wilayas.iter().enumerate() {
                let name = format!("W{}", i + 1);
                let format = format!(
                    r#"const W{}: Wilaya = Wilaya {{
    mattricule: {},
    name_ar: "{}",
    name_ber: "{}",
    name_en: "{}",
    name: "{}",
    phone_codes: &[{}],
    postal_codes: &[{}],
    dairats: &[{}],
    adjacent_wilayas: &[{}],
}};
    "#,
                    i + 1,
                    wilaya.mattricule,
                    wilaya.name_ar,
                    wilaya.name_ber,
                    wilaya.name_en,
                    wilaya.name,
                    wilaya
                        .phone_codes
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    wilaya
                        .postal_codes
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    wilaya
                        .dairats
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    wilaya
                        .adjacent_wilayas
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                consts.push(Const { name, format });
            }
            let mut s = r#"// This is auto-generated. Do not edit manually.

/// Wilaya struct.
/// ## Description
/// This struct is used to define a wilaya from our database.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Wilaya {
    /// Wilaya mattricule or code.
    pub mattricule: u16,
    /// Wilaya's name in Arabic.
    pub name_ar: &'static str,
    /// Wilaya's name in Berber.
    pub name_ber: &'static str,
    /// Wilaya's name in English.
    pub name_en: &'static str,
    /// Wilaya's name.
    pub name: &'static str,
    /// Wilaya's phone codes.
    pub phone_codes: &'static[u16],
    /// Wilaya's postal codes or zip codes.
    pub postal_codes: &'static[u16],
    /// Wilaya's dairats.
    pub dairats: &'static[Daira],
    /// Wilaya's adjacent wilayas.
    pub adjacent_wilayas: &'static[u16],
}

/// Daira struct.
/// ## Description
/// This struct is used to define a daira from our database.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Daira {
    /// Daira's code.
    pub code: u16,
    /// Daira's name.
    pub name: &'static str,
    /// Daira's name in Arabic.
    pub name_ar: &'static str,
    /// Daira's name in English.
    pub name_en: &'static str,
    /// Daira's baladyiats.
    pub baladyiats: Option<&'static[Baladyia]>,
}

/// Baladyia struct.
/// ## Description
/// This struct is used to define a baladyia from our database.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Baladyia {
    /// Baladyia's code.
    pub code: u16,
    /// Baladyia's name.
    pub name: &'static str,
    /// Baladyia's name in English.
    pub name_en: &'static str,
    /// Baladyia's name in Arabic.
    pub name_ar: &'static str,
}
"#
            .to_string();
            s.push_str(
                consts
                    .iter()
                    .map(|c| c.format.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_str(),
            );
            s.push_str(
                format!(
                    "\npub(crate) const ALL_WILAYAS: &[Wilaya] = &[{}];\n",
                    consts
                        .iter()
                        .map(|c| c.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
                .as_str(),
            );
            fs::write("./src/_auto_generated/mod.rs", s)?;
        }
        Err(_) => panic!("Data was not received"),
    }
    Ok(())
}
