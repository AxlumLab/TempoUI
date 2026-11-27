use slint::fontique_012::{fontique, set_shared_collection};

/// Prefer the same installed CJK fallback families as FinUI without bundling platform fonts.
/// Call before constructing a Slint window.
pub fn configure_finui_fallbacks() -> Result<(), slint::PlatformError> {
    let mut collection = fontique::Collection::new(fontique::CollectionOptions {
        shared: true,
        ..Default::default()
    });
    for (script, preferences) in [
        (
            *b"Hani",
            &["Microsoft YaHei", "PingFang SC", "Noto Sans CJK SC"][..],
        ),
        (
            *b"Hira",
            &["Hiragino Sans", "Yu Gothic", "Noto Sans CJK JP"][..],
        ),
        (
            *b"Kana",
            &["Hiragino Sans", "Yu Gothic", "Noto Sans CJK JP"][..],
        ),
        (
            *b"Hang",
            &["Apple SD Gothic Neo", "Malgun Gothic", "Noto Sans CJK KR"][..],
        ),
    ] {
        let key = fontique::FallbackKey::new(fontique::Script::from_bytes(script), None);
        let mut families: Vec<_> = preferences
            .iter()
            .filter_map(|name| collection.family_by_name(name).map(|family| family.id()))
            .collect();
        if !families.is_empty() {
            let existing: Vec<_> = collection.fallback_families(key).collect();
            for id in existing {
                if !families.contains(&id) {
                    families.push(id);
                }
            }
            collection.set_fallbacks(key, families.into_iter());
        }
    }
    set_shared_collection(collection)
}
