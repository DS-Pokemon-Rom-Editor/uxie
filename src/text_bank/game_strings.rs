use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::io;
use std::path::Path;

use crate::GameFamily;
use crate::game::GameLanguage;

/// Archive indices for game string tables in the message NARC.
///
/// Encryption keys do not identify archive categories.
#[derive(Debug, Clone, Copy)]
pub struct TextBankIds {
    pub species: u16,
    pub items: u16,
    pub moves: u16,
    pub abilities: u16,
    pub types: u16,
    pub locations: u16,
    pub trainers: u16,
    pub trainer_classes: u16,
    pub sounds: u16,
}

impl TextBankIds {
    /// Return archive indices for `family` and `language`.
    ///
    /// These indices are the same in both games of each pair and in the
    /// checked revisions. English USA and Europe use the same indices.
    /// See `docs/text_archives.md` for the checked ROM identities.
    #[allow(clippy::too_many_lines)] // One named record for each regional layout.
    pub fn for_game(family: GameFamily, language: GameLanguage) -> Self {
        match (family, language) {
            (GameFamily::DP, GameLanguage::Japanese) => Self {
                species: 356,
                items: 341,
                moves: 575,
                abilities: 544,
                types: 555,
                locations: 374,
                trainers: 550,
                trainer_classes: 551,
                sounds: 482,
            },
            (GameFamily::DP, GameLanguage::Korean) => Self {
                species: 357,
                items: 342,
                moves: 577,
                abilities: 546,
                types: 557,
                locations: 376,
                trainers: 552,
                trainer_classes: 553,
                sounds: 484,
            },
            (GameFamily::DP, _) => Self {
                species: 362,
                items: 344,
                moves: 588,
                abilities: 552,
                types: 565,
                locations: 382,
                trainers: 559,
                trainer_classes: 560,
                sounds: 490,
            },
            (GameFamily::Platinum, GameLanguage::Japanese) => Self {
                species: 408,
                items: 390,
                moves: 636,
                abilities: 604,
                types: 616,
                locations: 427,
                trainers: 611,
                trainer_classes: 612,
                sounds: 539,
            },
            (GameFamily::Platinum, GameLanguage::Korean) => Self {
                species: 408,
                items: 390,
                moves: 637,
                abilities: 605,
                types: 617,
                locations: 428,
                trainers: 612,
                trainer_classes: 613,
                sounds: 540,
            },
            (GameFamily::Platinum, _) => Self {
                species: 412,
                items: 392,
                moves: 647,
                abilities: 610,
                types: 624,
                locations: 433,
                trainers: 618,
                trainer_classes: 619,
                sounds: 545,
            },
            (GameFamily::HGSS, GameLanguage::Japanese) => Self {
                species: 232,
                items: 219,
                moves: 739,
                abilities: 711,
                types: 724,
                locations: 272,
                trainers: 719,
                trainer_classes: 720,
                sounds: 430,
            },
            (GameFamily::HGSS, GameLanguage::Korean) => Self {
                species: 233,
                items: 220,
                moves: 743,
                abilities: 715,
                types: 728,
                locations: 274,
                trainers: 723,
                trainer_classes: 724,
                sounds: 432,
            },
            (GameFamily::HGSS, _) => Self {
                species: 237,
                items: 222,
                moves: 750,
                abilities: 720,
                types: 735,
                locations: 279,
                trainers: 729,
                trainer_classes: 730,
                sounds: 437,
            },
        }
    }
}

/// JSON structure for Chatot text archives
#[derive(Debug, Deserialize)]
struct ChatotTextArchive {
    #[allow(dead_code)]
    key: u16,
    messages: Vec<ChatotMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatotMessage {
    #[allow(dead_code)]
    id: String,
    #[serde(flatten)]
    lang_content: FxHashMap<String, MessageContent>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MessageContent {
    Single(String),
    Multi(Vec<String>),
}

/// Holds all game string tables loaded from expanded/textArchives (DSPRE text export).
/// Provides name-to-ID and ID-to-name resolution for species, items, moves, abilities, and types.
#[derive(Debug, Clone, Default)]
pub struct GameStrings {
    /// Species names indexed by species ID
    pub species: Vec<String>,
    /// Lowercase species name -> ID
    species_to_id: FxHashMap<String, u16>,

    /// Item names indexed by item ID
    pub items: Vec<String>,
    /// Lowercase item name -> ID
    items_to_id: FxHashMap<String, u16>,

    /// Move names indexed by move ID
    pub moves: Vec<String>,
    /// Lowercase move name -> ID
    moves_to_id: FxHashMap<String, u16>,

    /// Ability names indexed by ability ID
    pub abilities: Vec<String>,
    /// Lowercase ability name -> ID
    abilities_to_id: FxHashMap<String, u16>,

    /// Type names indexed by type ID
    pub types: Vec<String>,
    /// Lowercase type name -> ID
    types_to_id: FxHashMap<String, u16>,
}

impl GameStrings {
    /// Create a new empty GameStrings
    pub fn new() -> Self {
        Self::default()
    }

    /// Load game strings from expanded/textArchives (DSPRE text export folder)
    pub fn load_from_dspre(
        project_path: impl AsRef<Path>,
        family: GameFamily,
        language: GameLanguage,
    ) -> std::io::Result<Self> {
        let text_archives_path = project_path.as_ref().join("expanded/textArchives");

        if !text_archives_path.exists() {
            return Ok(Self::new());
        }

        let bank_ids = TextBankIds::for_game(family, language);

        let mut gs = Self::new();

        let names = load_text_archive_if_present(&text_archives_path, bank_ids.species, "species")?;
        if !names.is_empty() {
            gs.species_to_id = build_lookup_table(&names);
            gs.species = names;
        }

        let names = load_text_archive_if_present(&text_archives_path, bank_ids.items, "items")?;
        if !names.is_empty() {
            gs.items_to_id = build_lookup_table(&names);
            gs.items = names;
        }

        let names = load_text_archive_if_present(&text_archives_path, bank_ids.moves, "moves")?;
        if !names.is_empty() {
            gs.moves_to_id = build_lookup_table(&names);
            gs.moves = names;
        }

        let names =
            load_text_archive_if_present(&text_archives_path, bank_ids.abilities, "abilities")?;
        if !names.is_empty() {
            gs.abilities_to_id = build_lookup_table(&names);
            gs.abilities = names;
        }

        let names = load_text_archive_if_present(&text_archives_path, bank_ids.types, "types")?;
        if !names.is_empty() {
            gs.types_to_id = build_lookup_table(&names);
            gs.types = names;
        }

        Ok(gs)
    }

    /// Get species name by ID
    pub fn get_species_name(&self, id: u16) -> Option<&str> {
        self.species.get(id as usize).map(|s| s.as_str())
    }

    /// Get species ID by name (case-insensitive)
    pub fn get_species_id(&self, name: &str) -> Option<u16> {
        self.species_to_id.get(&normalize_name(name)).copied()
    }

    /// Get item name by ID
    pub fn get_item_name(&self, id: u16) -> Option<&str> {
        self.items.get(id as usize).map(|s| s.as_str())
    }

    /// Get item ID by name (case-insensitive)
    pub fn get_item_id(&self, name: &str) -> Option<u16> {
        self.items_to_id.get(&normalize_name(name)).copied()
    }

    /// Get move name by ID
    pub fn get_move_name(&self, id: u16) -> Option<&str> {
        self.moves.get(id as usize).map(|s| s.as_str())
    }

    /// Get move ID by name (case-insensitive)
    pub fn get_move_id(&self, name: &str) -> Option<u16> {
        self.moves_to_id.get(&normalize_name(name)).copied()
    }

    /// Get ability name by ID
    pub fn get_ability_name(&self, id: u16) -> Option<&str> {
        self.abilities.get(id as usize).map(|s| s.as_str())
    }

    /// Get ability ID by name (case-insensitive)
    pub fn get_ability_id(&self, name: &str) -> Option<u16> {
        self.abilities_to_id.get(&normalize_name(name)).copied()
    }

    /// Get type name by ID
    pub fn get_type_name(&self, id: u16) -> Option<&str> {
        self.types.get(id as usize).map(|s| s.as_str())
    }

    /// Get type ID by name (case-insensitive)
    pub fn get_type_id(&self, name: &str) -> Option<u16> {
        self.types_to_id.get(&normalize_name(name)).copied()
    }

    /// Check if any strings are loaded
    pub fn is_empty(&self) -> bool {
        self.species.is_empty()
            && self.items.is_empty()
            && self.moves.is_empty()
            && self.abilities.is_empty()
            && self.types.is_empty()
    }
}

fn load_text_archive_if_present(
    base_path: &Path,
    bank_id: u16,
    bank_name: &str,
) -> io::Result<Vec<String>> {
    let json_path = base_path.join(format!("{:04}.json", bank_id));
    if !json_path.exists() {
        return Ok(Vec::new());
    }

    let names = load_text_archive(base_path, bank_id).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!(
                "Failed to load {} text archive {:04} at {}: {}",
                bank_name,
                bank_id,
                json_path.display(),
                err
            ),
        )
    })?;
    Ok(names)
}

/// Load a text archive JSON and extract all message strings
fn load_text_archive(base_path: &Path, bank_id: u16) -> std::io::Result<Vec<String>> {
    let json_path = base_path.join(format!("{:04}.json", bank_id));
    let content = std::fs::read_to_string(&json_path)?;

    let archive: ChatotTextArchive = serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut names = Vec::with_capacity(archive.messages.len());

    for msg in archive.messages {
        // Prefer en_US, fall back to first available language
        let text = if let Some(content) = msg.lang_content.get("en_US") {
            extract_text(content)
        } else if let Some((_, content)) = msg.lang_content.iter().next() {
            extract_text(content)
        } else {
            String::new()
        };
        names.push(text);
    }

    Ok(names)
}

/// Extract plain text from MessageContent
fn extract_text(content: &MessageContent) -> String {
    match content {
        MessageContent::Single(s) => s.clone(),
        MessageContent::Multi(parts) => parts.join(""),
    }
}

/// Normalize a name for lookup (lowercase, remove spaces, strip accents)
fn normalize_name(name: &str) -> String {
    deunicode::deunicode(name)
        .to_lowercase()
        .replace([' ', '-'], "")
}

/// Build a lookup table from name -> index
fn build_lookup_table(names: &[String]) -> FxHashMap<String, u16> {
    let mut map = FxHashMap::default();
    for (i, name) in names.iter().enumerate() {
        let normalized = normalize_name(name);
        if !normalized.is_empty() && !map.contains_key(&normalized) {
            map.insert(normalized, i as u16);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_normalize_name() {
        assert_eq!(normalize_name("Bulbasaur"), "bulbasaur");
        assert_eq!(normalize_name("MR. MIME"), "mr.mime");
        assert_eq!(normalize_name("Poké Ball"), "pokeball");
        assert_eq!(normalize_name("Ho-Oh"), "hooh");
    }

    #[test]
    fn test_load_from_dspre_missing_text_archives_is_empty() {
        let dir = tempdir().unwrap();
        let gs =
            GameStrings::load_from_dspre(dir.path(), GameFamily::Platinum, GameLanguage::English)
                .unwrap();
        assert!(gs.is_empty());
    }

    #[test]
    fn test_load_from_dspre_invalid_existing_archive_errors() {
        let dir = tempdir().unwrap();
        let archives = dir.path().join("expanded/textArchives");
        fs::create_dir_all(&archives).unwrap();

        // Platinum species bank.
        fs::write(archives.join("0412.json"), "{ invalid json ").unwrap();

        let err =
            GameStrings::load_from_dspre(dir.path(), GameFamily::Platinum, GameLanguage::English)
                .unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("species"));
        assert!(err.to_string().contains("0412"));
    }

    #[test]
    fn test_load_from_dspre_loads_existing_archives_and_skips_missing() {
        let dir = tempdir().unwrap();
        let archives = dir.path().join("expanded/textArchives");
        fs::create_dir_all(&archives).unwrap();

        // Platinum species bank.
        fs::write(
            archives.join("0412.json"),
            r#"
            {
              "key": 0,
              "messages": [
                { "id": "0000", "en_US": "Bulbasaur" },
                { "id": "0001", "en_US": "Ivysaur" }
              ]
            }
            "#,
        )
        .unwrap();

        let gs =
            GameStrings::load_from_dspre(dir.path(), GameFamily::Platinum, GameLanguage::English)
                .unwrap();

        assert_eq!(gs.get_species_name(0), Some("Bulbasaur"));
        assert_eq!(gs.get_species_name(1), Some("Ivysaur"));
        assert_eq!(gs.get_species_id("bulbasaur"), Some(0));
        // Missing optional archives remain empty.
        assert!(gs.items.is_empty());
        assert!(gs.moves.is_empty());
    }

    #[test]
    #[ignore = "requires local Platinum DSPRE fixture via UXIE_TEST_PLATINUM_DSPRE_PATH"]
    fn integration_load_from_dspre_platinum_real_fixture() {
        let Some(project_path) = crate::test_env::existing_path_from_env(
            "UXIE_TEST_PLATINUM_DSPRE_PATH",
            "game strings integration test (Platinum DSPRE)",
        ) else {
            return;
        };

        let gs =
            GameStrings::load_from_dspre(project_path, GameFamily::Platinum, GameLanguage::English)
                .unwrap();

        assert!(
            !gs.is_empty(),
            "expected at least one populated game-strings bank from Platinum DSPRE fixture"
        );
        assert!(
            !gs.species.is_empty(),
            "expected non-empty species names for Platinum DSPRE fixture"
        );

        let first_non_empty_species = gs.species.iter().position(|name| !name.trim().is_empty());
        let Some(idx) = first_non_empty_species else {
            panic!("expected at least one non-empty species name");
        };
        let species_name = gs.species[idx].clone();
        assert_eq!(gs.get_species_name(idx as u16), Some(species_name.as_str()));
        assert!(
            gs.get_species_id(&species_name).is_some(),
            "expected reverse lookup for non-empty species name '{}'",
            species_name
        );
    }

    #[test]
    #[ignore = "requires local HGSS DSPRE fixture via UXIE_TEST_HGSS_DSPRE_PATH"]
    fn integration_load_from_dspre_hgss_real_fixture() {
        let Some(project_path) = crate::test_env::existing_path_from_env(
            "UXIE_TEST_HGSS_DSPRE_PATH",
            "game strings integration test (HGSS DSPRE)",
        ) else {
            return;
        };

        let gs =
            GameStrings::load_from_dspre(project_path, GameFamily::HGSS, GameLanguage::English)
                .unwrap();

        assert!(
            !gs.is_empty(),
            "expected at least one populated game-strings bank from HGSS DSPRE fixture"
        );
        assert!(
            !gs.species.is_empty(),
            "expected non-empty species names for HGSS DSPRE fixture"
        );

        let first_non_empty_species = gs.species.iter().position(|name| !name.trim().is_empty());
        let Some(idx) = first_non_empty_species else {
            panic!("expected at least one non-empty species name");
        };
        let species_name = gs.species[idx].clone();
        assert_eq!(gs.get_species_name(idx as u16), Some(species_name.as_str()));
        assert!(
            gs.get_species_id(&species_name).is_some(),
            "expected reverse lookup for non-empty species name '{}'",
            species_name
        );
    }

    fn language_strategy() -> impl Strategy<Value = GameLanguage> {
        prop_oneof![
            Just(GameLanguage::English),
            Just(GameLanguage::Japanese),
            Just(GameLanguage::French),
            Just(GameLanguage::German),
            Just(GameLanguage::Italian),
            Just(GameLanguage::Spanish),
            Just(GameLanguage::Korean),
        ]
    }

    fn raw_name_strategy() -> impl Strategy<Value = String> {
        (any::<u16>(), 0u8..6).prop_map(|(id, variant)| {
            let base = format!("Poke Name {}", id % 24);
            match variant {
                0 => base,
                1 => base.replace(' ', ""),
                2 => base.to_lowercase(),
                3 => base.to_uppercase(),
                4 => base.replace(' ', "-"),
                _ => format!(" {} ", base),
            }
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 64,
            .. ProptestConfig::default()
        })]

        #[test]
        fn prop_normalize_name_idempotent(name in any::<String>()) {
            let normalized = normalize_name(&name);
            prop_assert_eq!(normalize_name(&normalized), normalized);
        }

        #[test]
        fn prop_normalize_name_removes_spaces_and_hyphens(name in any::<String>()) {
            let normalized = normalize_name(&name);
            prop_assert!(!normalized.contains(' '));
            prop_assert!(!normalized.contains('-'));
            let lower = normalized.to_lowercase();
            prop_assert_eq!(normalized, lower);
        }

        #[test]
        fn prop_build_lookup_table_matches_first_normalized_occurrence(names in prop::collection::vec(raw_name_strategy(), 0..96)) {
            let actual = build_lookup_table(&names);

            let mut expected = FxHashMap::default();
            for (idx, name) in names.iter().enumerate() {
                let normalized = normalize_name(name);
                if !normalized.is_empty() && !expected.contains_key(&normalized) {
                    expected.insert(normalized, idx as u16);
                }
            }

            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn prop_text_bank_ids_family_language_rules(lang in language_strategy()) {
            for family in [GameFamily::DP, GameFamily::Platinum, GameFamily::HGSS] {
                let ids = TextBankIds::for_game(family, lang);
                // Archive indices checked against the regional retail ROMs.
                let expected = match (family, lang) {
                    (GameFamily::DP, GameLanguage::Japanese) => [356, 341, 575, 544, 555, 374, 550, 551, 482],
                    (GameFamily::DP, GameLanguage::Korean) => [357, 342, 577, 546, 557, 376, 552, 553, 484],
                    (GameFamily::DP, _) => [362, 344, 588, 552, 565, 382, 559, 560, 490],
                    (GameFamily::Platinum, GameLanguage::Japanese) => [408, 390, 636, 604, 616, 427, 611, 612, 539],
                    (GameFamily::Platinum, GameLanguage::Korean) => [408, 390, 637, 605, 617, 428, 612, 613, 540],
                    (GameFamily::Platinum, _) => [412, 392, 647, 610, 624, 433, 618, 619, 545],
                    (GameFamily::HGSS, GameLanguage::Japanese) => [232, 219, 739, 711, 724, 272, 719, 720, 430],
                    (GameFamily::HGSS, GameLanguage::Korean) => [233, 220, 743, 715, 728, 274, 723, 724, 432],
                    (GameFamily::HGSS, _) => [237, 222, 750, 720, 735, 279, 729, 730, 437],
                };
                prop_assert_eq!(
                    [ids.species, ids.items, ids.moves, ids.abilities, ids.types,
                     ids.locations, ids.trainers, ids.trainer_classes, ids.sounds],
                    expected
                );
            }
        }

        #[test]
        fn prop_game_strings_species_lookup_consistency(names in prop::collection::vec(raw_name_strategy(), 0..96)) {
            let species_to_id = build_lookup_table(&names);
            let gs = GameStrings {
                species: names.clone(),
                species_to_id: species_to_id.clone(),
                ..GameStrings::default()
            };

            for (idx, name) in names.iter().enumerate() {
                prop_assert_eq!(gs.get_species_name(idx as u16), Some(name.as_str()));
                let normalized = normalize_name(name);
                if normalized.is_empty() {
                    prop_assert_eq!(gs.get_species_id(name), None);
                } else {
                    prop_assert_eq!(gs.get_species_id(name), species_to_id.get(&normalized).copied());
                }
            }
        }
    }
}
