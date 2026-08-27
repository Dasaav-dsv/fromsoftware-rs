use pelite::pe::{Pe, PeView};

const LANG_ID_EN: u16 = 0x0009;
const LANG_ID_JP: u16 = 0x0011;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVersion {
    Ww270,
    Jp2701,
}

impl GameVersion {
    pub fn detect(module: &PeView<'_>) -> Option<Self> {
        let resources = module.resources().ok()?;
        let info = resources.version_info().ok()?;

        // Extract version info
        let product_version = info.fixed()?.dwProductVersion;
        let version = format!(
            "{}.{}.{}.{}",
            product_version.Major,
            product_version.Minor,
            product_version.Patch,
            product_version.Build,
        );

        // Extract product name
        let language = *info.translation().first()?;
        let mut product_name: Option<String> = None;
        info.strings(language, |k, v| {
            if k == "ProductName" {
                product_name = Some(v.to_string());
            }
        });

        let product = product_name?;
        let lang_id_base = language.lang_id & 0x03FF;

        // Detect version and return appropriate RVAs
        GameVersion::from_metadata(&product, lang_id_base, &version)
    }

    fn from_metadata(product: &str, lang_id: u16, version: &str) -> Option<Self> {
        match (product, lang_id, version) {
            ("ELDEN RING™", LANG_ID_EN, "2.7.0.0") => Some(Self::Ww270),
            ("ELDEN RING", LANG_ID_JP, "2.7.0.1") => Some(Self::Jp2701),
            _ => None,
        }
    }
}
