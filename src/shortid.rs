use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefKind {
    Task,
    Project,
}

impl RefKind {
    fn from_prefix(c: char) -> Option<Self> {
        match c {
            't' => Some(Self::Task),
            'p' => Some(Self::Project),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefEntry {
    pub kind: RefKind,
    pub uuid: String,
}

fn cache_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("things-cli");
    Ok(config_dir.join("refs.json"))
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct RefCache {
    refs: HashMap<String, RefEntry>,
}

/// Assign typed refs to a list of (kind, uuid) entries.
/// Reuses existing refs for UUIDs already in the cache.
/// New items get the next available number for their kind.
pub fn assign(entries: &[(RefKind, &str)]) -> Vec<String> {
    let mut cache = load_cache().unwrap_or_default();

    let mut task_max = max_num(&cache, 't');
    let mut project_max = max_num(&cache, 'p');

    let mut assigned: Vec<String> = Vec::with_capacity(entries.len());

    for &(kind, uuid) in entries {
        if let Some(existing) = find_existing_ref(&cache, kind, uuid) {
            assigned.push(existing);
            continue;
        }

        let (prefix, n) = match kind {
            RefKind::Task => {
                task_max += 1;
                ('t', task_max)
            }
            RefKind::Project => {
                project_max += 1;
                ('p', project_max)
            }
        };
        let ref_id = format!("{prefix}{n}");
        cache.refs.insert(
            ref_id.clone(),
            RefEntry {
                kind,
                uuid: uuid.to_owned(),
            },
        );
        assigned.push(ref_id);
    }

    let _ = save_cache(&cache);
    assigned
}

fn find_existing_ref(cache: &RefCache, kind: RefKind, uuid: &str) -> Option<String> {
    cache
        .refs
        .iter()
        .find(|(_, entry)| entry.kind == kind && entry.uuid == uuid)
        .map(|(ref_id, _)| ref_id.clone())
}

fn max_num(cache: &RefCache, prefix: char) -> usize {
    cache
        .refs
        .keys()
        .filter_map(|k| {
            let mut chars = k.chars();
            if chars.next() == Some(prefix) {
                chars.as_str().parse::<usize>().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

/// Parse a ref from user input. Accepts `@t1`, `ref=t1`, or bare `t1`.
pub fn parse_ref(input: &str) -> Option<String> {
    let trimmed = input.trim();

    let bare = if let Some(stripped) = trimmed.strip_prefix('@') {
        stripped
    } else if let Some(stripped) = trimmed.strip_prefix("ref=") {
        stripped
    } else {
        trimmed
    };

    let mut chars = bare.chars();
    let prefix = chars.next()?;
    RefKind::from_prefix(prefix)?;
    let rest: String = chars.collect();
    if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
        Some(bare.to_owned())
    } else {
        None
    }
}

/// Look up a ref from the cache.
/// Accepts any format that `parse_ref` handles.
pub fn resolve(input: &str) -> Option<RefEntry> {
    let ref_id = parse_ref(input)?;
    load_cache().ok().and_then(|c| c.refs.get(&ref_id).cloned())
}

/// Clear the ref cache.
pub fn clear() {
    let _ = save_cache(&RefCache::default());
}

/// Return all cached refs sorted by ref ID.
pub fn dump() -> Vec<(String, RefEntry)> {
    let cache = load_cache().unwrap_or_default();
    let mut entries: Vec<(String, RefEntry)> = cache.refs.into_iter().collect();
    entries.sort_by(|(a, _), (b, _)| {
        let a_prefix = a.chars().next().unwrap_or('z');
        let b_prefix = b.chars().next().unwrap_or('z');
        a_prefix.cmp(&b_prefix).then_with(|| {
            let a_num: usize = a[1..].parse().unwrap_or(0);
            let b_num: usize = b[1..].parse().unwrap_or(0);
            a_num.cmp(&b_num)
        })
    });
    entries
}

fn save_cache(cache: &RefCache) -> Result<()> {
    let path = cache_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string(cache)?;
    fs::write(&path, json)?;
    Ok(())
}

fn load_cache() -> Result<RefCache> {
    let path = cache_path()?;
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("No recent ref cache at {}", path.display()))?;
    let cache: RefCache = serde_json::from_str(&contents)?;
    Ok(cache)
}
