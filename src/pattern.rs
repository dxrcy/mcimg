/// Kind of pattern to test
///
/// `Exact`: Value matches exactly with pattern.
/// Use no wildcards to identify
///
/// `StartsWith`: Value starts with target string in pattern.
/// Use wildcard `*` at end of pattern to identify
///
/// `EndsWith`: Value ends with target string in pattern.
/// Use wildcard `*` at start of pattern to identify
///
/// `Contains`: Value is somewhere within target string in pattern.
/// Use wildcard `*` at start and end of pattern to identify
#[derive(Debug)]
pub enum Pattern {
  /// `Exact`: Value matches exactly with pattern.
  /// Use no wildcards to identify
  Exact(String),
  /// `StartsWith`: Value starts with target string in pattern.
  /// Use wildcard `*` at end of pattern to identify
  StartsWith(String),
  /// `EndsWith`: Value ends with target string in pattern.
  /// Use wildcard `*` at start of pattern to identify
  EndsWith(String),
  /// `Contains`: Value is somewhere within target string in pattern.
  /// Use wildcard `*` at start and end of pattern to identify
  Contains(String),
}

use Pattern::*;

impl Pattern {
  /// Parse `Pattern` from string
  ///
  /// Use wildcard `*` at start and/or end to identify `Pattern` variant
  pub fn from(pattern: &str) -> Pattern {
    if pattern.starts_with('*') {
      if pattern.ends_with('*') {
        Contains(remove_first_last_char(pattern).to_string())
      } else {
        EndsWith(remove_first_char(pattern).to_string())
      }
    } else {
      if pattern.ends_with('*') {
        StartsWith(remove_last_char(pattern).to_string())
      } else {
        Exact(pattern.to_string())
      }
    }
  }

  /// Test if string value matches target string in pattern
  pub fn matches(&self, value: &str) -> bool {
    match self {
      Exact(target) => value == target,
      StartsWith(target) => value.starts_with(target),
      EndsWith(target) => value.ends_with(target),
      Contains(target) => value.contains(target),
    }
  }
}

/// Remove first character of string
fn remove_first_char<'a>(s: &'a str) -> &'a str {
  let mut chars = s.chars();
  chars.next();
  chars.as_str()
}

/// Remove last character of string
fn remove_last_char<'a>(s: &'a str) -> &'a str {
  let mut chars = s.chars();
  chars.next_back();
  chars.as_str()
}

/// Remove first and last character of string
fn remove_first_last_char<'a>(s: &'a str) -> &'a str {
  let mut chars = s.chars();
  chars.next();
  chars.next_back();
  chars.as_str()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn remove_char_fns_work() {
    assert_eq!(remove_first_char("abcd"), "bcd");
    assert_eq!(remove_last_char("abcd"), "abc");
    assert_eq!(remove_first_last_char("abcd"), "bc");
  }

  #[test]
  fn pattern_init_works() {
    // Exact
    let pat = Pattern::from("TARGET");
    assert!(match pat {
      Exact(s) => s == "TARGET",
      _ => false,
    });

    // StartsWith
    let pat = Pattern::from("TARGET*");
    assert!(match pat {
      StartsWith(s) => s == "TARGET",
      _ => false,
    });

    // EndsWith
    let pat = Pattern::from("*TARGET");
    assert!(match pat {
      EndsWith(s) => s == "TARGET",
      _ => false,
    });

    // Contains
    let pat = Pattern::from("*TARGET*");
    assert!(match pat {
      Contains(s) => s == "TARGET",
      _ => false,
    });
  }

  #[test]
  /// Exact
  fn pattern_exact_works() {
    let pat = Pattern::from("TARGET");
    assert_eq!(pat.matches("TARGET"), true);
    assert_eq!(pat.matches("target"), false);
    assert_eq!(pat.matches("_TARGET"), false);
    assert_eq!(pat.matches("TARGET_"), false);
    assert_eq!(pat.matches("_TARGET_"), false);
  }

  #[test]
  /// StartsWith
  fn pattern_starts_with_works() {
    let pat = Pattern::from("TARGET*");
    assert_eq!(pat.matches("TARGET"), true);
    assert_eq!(pat.matches("target"), false);
    assert_eq!(pat.matches("_TARGET"), false);
    assert_eq!(pat.matches("TARGET_"), true);
    assert_eq!(pat.matches("_TARGET_"), false);
  }

  #[test]
  /// EndsWith
  fn pattern_ends_with_works() {
    let pat = Pattern::from("*TARGET");
    assert_eq!(pat.matches("TARGET"), true);
    assert_eq!(pat.matches("target"), false);
    assert_eq!(pat.matches("_TARGET"), true);
    assert_eq!(pat.matches("TARGET_"), false);
    assert_eq!(pat.matches("_TARGET_"), false);
  }

  #[test]
  /// Contains
  fn pattern_contains_works() {
    let pat = Pattern::from("*TARGET*");
    assert_eq!(pat.matches("TARGET"), true);
    assert_eq!(pat.matches("target"), false);
    assert_eq!(pat.matches("_TARGET"), true);
    assert_eq!(pat.matches("TARGET_"), true);
    assert_eq!(pat.matches("_TARGET_"), true);
  }
}
