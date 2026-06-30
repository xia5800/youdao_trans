use crate::constants;
use rusqlite::Connection;
use serde::Serialize;
use std::path::PathBuf;

fn db_path() -> PathBuf {
    crate::config::default_models_dir_inner().join("dict/ecdict.db")
}

#[derive(Serialize)]
pub struct WordEntry {
    pub word: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: i32,
    pub oxford: i32,
    pub tag: String,
    pub bnc: Option<i32>,
    pub frq: Option<i32>,
    pub exchange: String,
    pub detail: String,
}

#[derive(Serialize)]
pub struct ExchangeItem {
    pub label: String,
    pub value: String,
}

fn parse_exchange(raw: &str) -> Vec<ExchangeItem> {
    let order = ["0", "1", "s", "p", "d", "i", "3", "r", "t"];
    let labels = [
        ("0", "原型"), ("1", "复数形式"), ("s", "复数"),
        ("p", "过去式"), ("d", "过去分词"), ("i", "现在分词"),
        ("3", "第三人称单数"), ("r", "比较级"), ("t", "最高级"),
    ];
    let mut seen = std::collections::HashSet::new();
    let mut map = std::collections::BTreeMap::new();
    for part in raw.split('/') {
        let mut it = part.splitn(2, ':');
        let key = it.next().unwrap_or("");
        let val = it.next().unwrap_or("").trim_start_matches('\'');
        if val.is_empty() || !seen.insert(val) { continue; }
        let idx = order.iter().position(|&k| k == key).unwrap_or(99);
        let label = labels.iter().find(|(k, _)| *k == key).map(|(_, l)| *l).unwrap_or(key);
        map.insert(idx, ExchangeItem { label: label.to_string(), value: val.to_string() });
    }
    map.into_values().collect()
}

pub fn suggestions(prefix: &str, limit: u32) -> Result<Vec<String>, String> {
    let path = db_path();
    let conn = Connection::open(&path).map_err(|e| format!("open dict: {}", e))?;
    let pattern = format!("{}%", prefix);
    let mut stmt = conn
        .prepare(
            "SELECT word FROM stardict \
             WHERE sw LIKE ?1 AND word != '' \
             ORDER BY length(word), coalesce(frq, 99999) \
             LIMIT ?2",
        )
        .map_err(|e| format!("prepare: {}", e))?;
    let rows = stmt
        .query_map(rusqlite::params![pattern, limit], |row| row.get::<_, String>(0))
        .map_err(|e| format!("query: {}", e))?;
    let mut words = Vec::new();
    for row in rows {
        words.push(row.map_err(|e| format!("row: {}", e))?);
    }
    Ok(words)
}

pub fn lookup(word: &str) -> Result<Option<WordEntry>, String> {
    let path = db_path();
    let conn = Connection::open(&path).map_err(|e| format!("open dict: {}", e))?;
    let mut stmt = conn
        .prepare(
            "SELECT word, phonetic, definition, translation, pos, \
             collins, oxford, tag, bnc, frq, exchange, detail \
             FROM stardict WHERE word = ?1 COLLATE NOCASE LIMIT 1",
        )
        .map_err(|e| format!("prepare: {}", e))?;
    let mut rows = stmt
        .query_map(rusqlite::params![word], |row| {
            Ok(WordEntry {
                word: row.get(0)?,
                phonetic: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                definition: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                translation: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                pos: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                collins: row.get::<_, Option<i32>>(5)?.unwrap_or(0),
                oxford: row.get::<_, Option<i32>>(6)?.unwrap_or(0),
                tag: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
                bnc: row.get(8)?,
                frq: row.get(9)?,
                exchange: row.get::<_, Option<String>>(10)?.unwrap_or_default(),
                detail: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
            })
        })
        .map_err(|e| format!("query: {}", e))?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| format!("row: {}", e))?)),
        None => Ok(None),
    }
}

#[derive(Serialize)]
struct LookupResult {
    word: String,
    phonetic: String,
    meanings: Vec<serde_json::Value>,
    definitions: Vec<String>,
    tags: Vec<String>,
    collins: i32,
    oxford: i32,
    bnc: Option<i32>,
    frq: Option<i32>,
    exchange: Vec<ExchangeItem>,
    detail: String,
}

fn map_pos_label(prefix: &str) -> &str {
    match prefix.to_lowercase().trim_end_matches('.') {
        "n" => "n 名词",
        "v" => "v 动词",
        "vi" => "vi 不及物动词",
        "vt" => "vt 及物动词",
        "aux" => "aux 助动词",
        "adj" | "j" => "adj 形容词",
        "adv" | "r" => "adv 副词",
        "prep" => "prep 介词",
        "pron" => "pron 代词",
        "conj" => "conj 连词",
        "interj" | "int" | "u" => "int 感叹词",
        "num" => "num 数词",
        "art" | "a" => "art 冠词",
        "det" => "det 限定词",
        "pref" => "pref 前缀",
        "suff" => "suff 后缀",
        "pl" => "pl 复数",
        _ => prefix.trim_end_matches('.'),
    }
}

/// Maps a translation-line prefix to the code used in the `pos` ratio field
fn pos_ratio_code(prefix: &str) -> Option<&str> {
    Some(match prefix.to_lowercase().trim_end_matches('.') {
        "adj" | "j" => "j",
        "adv" | "r" => "r",
        "interj" | "int" | "u" => "u",
        "art" | "a" => "a",
        "vi" | "vt" => "v",       // pos field uses the umbrella "v"
        "v" | "n" | "prep" | "pron" | "conj" | "num"
            | "det" | "pref" | "suff" | "pl" => prefix.trim_end_matches('.'),
        _ => return None,
    })
}

fn parse_pos_ratios(raw: &str) -> std::collections::HashMap<String, i32> {
    let mut map = std::collections::HashMap::new();
    for part in raw.split('/') {
        let mut it = part.splitn(2, ':');
        if let (Some(code), Some(pct)) = (it.next(), it.next()) {
            if let Ok(n) = pct.trim().parse::<i32>() {
                map.insert(code.trim().to_string(), n);
            }
        }
    }
    map
}

fn extract_prefix(line: &str) -> Option<String> {
    let line = line.trim();
    if let Some(dot) = line.find(". ") {
        let candidate = &line[..dot];
        if !candidate.contains(' ') && candidate.len() <= 5 {
            return Some(candidate.to_string());
        }
    }
    None
}

fn parse_translation_line(line: &str) -> (String, String) {
    let line = line.trim();
    if let Some(dot) = line.find(". ") {
        let candidate = &line[..dot];
        if !candidate.contains(' ') && candidate.len() <= 5 {
            let pos = map_pos_label(candidate);
            let text = line[dot + 2..].trim().to_string();
            return (pos.to_string(), text);
        }
    }
    ("".to_string(), line.to_string())
}

pub fn lookup_formatted(word: &str) -> Result<Option<serde_json::Value>, String> {
    let entry = lookup(word)?;
    match entry {
        None => Ok(None),
        Some(e) => {
            let tags: Vec<String> = e.tag.split_whitespace()
                .filter(|t| !t.is_empty()).map(|s| s.to_string()).collect();
            let exchange_vec = parse_exchange(&e.exchange);
            let pos_ratios = parse_pos_ratios(&e.pos);

            let definitions: Vec<String> = e.definition.split('\n')
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();

            let mut meanings = Vec::new();
            for line in e.translation.split('\n') {
                let l = line.trim();
                if l.is_empty() { continue; }
                let (pos, text) = parse_translation_line(l);

                // Look up percentage from pos field
                let pct = extract_prefix(l)
                    .as_deref()
                    .and_then(pos_ratio_code)
                    .and_then(|code| pos_ratios.get(code))
                    .copied();

                let mut obj = serde_json::Map::new();
                obj.insert("pos".into(), serde_json::Value::String(pos));
                obj.insert("translation".into(), serde_json::Value::String(text));
                if let Some(p) = pct {
                    obj.insert("pct".into(), serde_json::Value::Number(p.into()));
                }
                meanings.push(serde_json::Value::Object(obj));
            }

            let result = LookupResult {
                word: e.word,
                phonetic: e.phonetic,
                meanings,
                definitions,
                tags,
                collins: e.collins,
                oxford: e.oxford,
                bnc: e.bnc,
                frq: e.frq,
                exchange: exchange_vec,
                detail: e.detail,
            };
            serde_json::to_value(result).map(Some).map_err(|e| format!("serialize: {}", e))
        }
    }
}

// ---- Tauri command wrappers ----

#[tauri::command]
pub fn dict_suggestions(prefix: String, limit: Option<u32>) -> Result<Vec<String>, String> {
    suggestions(&prefix, limit.unwrap_or(constants::DICT_DEFAULT_SUGGESTION_LIMIT))
}

#[tauri::command]
pub fn dict_lookup(word: String) -> Result<Option<serde_json::Value>, String> {
    lookup_formatted(&word)
}