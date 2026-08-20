//! Shared substring search for `str before`, `str after`, and `str between`.

pub fn before<'a>(haystack: &'a str, needle: &str, last: bool, inclusive: bool) -> Option<&'a str> {
    if needle.is_empty() {
        return None;
    }
    let pos = if last {
        haystack.rfind(needle)
    } else {
        haystack.find(needle)
    }?;
    let end = if inclusive {
        pos.checked_add(needle.len())?
    } else {
        pos
    };
    haystack.get(..end)
}

pub fn after<'a>(haystack: &'a str, needle: &str, last: bool, inclusive: bool) -> Option<&'a str> {
    if needle.is_empty() {
        return None;
    }
    let pos = if last {
        haystack.rfind(needle)
    } else {
        haystack.find(needle)
    }?;
    let start = if inclusive {
        pos
    } else {
        pos.checked_add(needle.len())?
    };
    haystack.get(start..)
}

pub fn between<'a>(haystack: &'a str, left: &str, right: &str, last: bool) -> Option<&'a str> {
    if left.is_empty() || right.is_empty() {
        return None;
    }
    let left_pos = if last {
        haystack.rfind(left)
    } else {
        haystack.find(left)
    }?;
    let start = left_pos.checked_add(left.len())?;
    let rel = haystack.get(start..)?.find(right)?;
    haystack.get(start..start.checked_add(rel)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn before_first_and_last() {
        assert_eq!(before("key=value=extra", "=", false, false), Some("key"));
        assert_eq!(before("a/b/c.txt", ".", true, false), Some("a/b/c"));
        assert_eq!(before("key=value", "=", false, true), Some("key="));
        assert_eq!(before("abc", "=", false, false), None);
        assert_eq!(before("abc", "", false, false), None);
    }

    #[test]
    fn after_first_and_last() {
        assert_eq!(
            after("key=value=extra", "=", false, false),
            Some("value=extra")
        );
        assert_eq!(after("a/b/c.txt", "/", true, false), Some("c.txt"));
        assert_eq!(after("key=value", "=", false, true), Some("=value"));
        assert_eq!(after("abc", "=", false, false), None);
    }

    #[test]
    fn between_first_and_last() {
        assert_eq!(between("foo[bar]baz", "[", "]", false), Some("bar"));
        assert_eq!(between("id=42;", "id=", ";", false), Some("42"));
        assert_eq!(between("[a] [b]", "[", "]", true), Some("b"));
        assert_eq!(between("nope", "[", "]", false), None);
        assert_eq!(between("[no close", "[", "]", false), None);
    }

    #[test]
    fn empty_haystack() {
        assert_eq!(before("", "=", false, false), None);
        assert_eq!(after("", "=", false, false), None);
        assert_eq!(between("", "[", "]", false), None);
    }

    #[test]
    fn delimiter_at_start_and_end() {
        assert_eq!(before("=value", "=", false, false), Some(""));
        assert_eq!(after("key=", "=", false, false), Some(""));
        assert_eq!(between("[]", "[", "]", false), Some(""));
    }
}
