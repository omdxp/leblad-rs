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

pub struct Wilaya {
    pub mattricule: u16,
    pub name_ar: &'static str,
    pub name_ber: &'static str,
    pub name_en: &'static str,
    pub name: &'static str,
    pub phone_codes: &'static[u16],
    pub postal_codes: &'static[u16],
    pub dairats: &'static[Daira],
    pub adjacent_wilayas: &'static[u16],
}

pub struct Daira {
    pub code: u16,
    pub name: &'static str,
    pub name_ar: &'static str,
    pub name_en: &'static str,
    pub baladyiats: Option<&'static[Baladyia]>,
}

pub struct Baladyia {
    pub code: u16,
    pub name: &'static str,
    pub name_en: &'static str,
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
                    "\npub const ALL_WILAYAS: &[Wilaya] = &[{}];\n",
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
