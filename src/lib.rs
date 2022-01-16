//! Minimal library to provide clap-style "Did you mean?" suggestions.
//!
//! The implementation is copied directly from clap.
use std::cmp::Ordering;
use std::convert::AsRef;

/// The confidence interval used to detect similarities.
const CONFIDENCE_THRESHOLD: f64 = 0.8;

/// Suggest a string that is similar to `target`,
/// but is actually present in the `possible_values`.
///
/// Returns `None` if there is nothing reasonably similar.
///
/// For example, if the user types `--fod` but the possible values are `["foo", "bar"]`,
/// then this function will suggest `Some("foo")`. If the user types 
/// `--something-completely-different`, then this will return `None`.
///
/// See also [provide_suggestions] to consider multiple possiblities.
///
/// If multiple possible values have the same level of (internal) similarity, this picks one
/// arbitrarily (which is not the best UI).
pub fn provide_suggestion<I, T>(target: &str, possible_values: I) -> Option<String>
    where T: AsRef<str>, I: IntoIterator<Item=T> {
    provide_suggestions(target, possible_values).pop()
}

/// Suggest strings that are similar to `target`,
/// but are actually present in the `possible_values`.
///
/// Returns an empty vector `vec![]` if there is nothing reasonbly similar.
///
/// For example, if the user types `--fod` but the possible values are `["foo", "bar"]`,
/// then this function will suggest `["foo"]`. If the user types 
/// `--something-completely-different`, then this will return `vec![]`.
///
/// See also [provide_suggestions], which only picks a single suggesetion.
pub fn provide_suggestions<I, T>(target: &str, possible_values: I) -> Vec<String> 
    where T: AsRef<str>, I: IntoIterator<Item=T> {
    // Implementation copied directly from clap
    // See here: https://github.com/clap-rs/clap/blob/7b7c76e3d0279b474c774ea738aecb1d77251df8/src/parse/features/suggestions.rs#L17-L23
    let mut candidates: Vec<(f64, String)> = possible_values.into_iter()
        .map(|pv| (strsim::jaro_winkler(target, pv.as_ref()), pv.as_ref().into()))
        .filter(|(confidence, _)| *confidence > CONFIDENCE_THRESHOLD)
        .collect();
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    candidates.into_iter().map(|(_, pv)| pv).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Copied directly from Clap, with `did_you_mean` -> `provide_suggestions`
    // See original: https://github.com/clap-rs/clap/blob/7b7c76e3d0279b474c774ea738aecb1d77251df8/src/parse/features/suggestions.rs#L78-L94

    #[test]
    fn possible_values_match() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(provide_suggestions("tst", p_vals.iter()), vec!["test"]);
    }

    #[test]
    fn possible_values_match_multiple() {
        let p_vals = ["test", "temp"];
        assert_eq!(provide_suggestions("te", p_vals.iter()), vec!["test", "temp"]);
    }

    #[test]
    fn possible_values_nomatch() {
        let p_vals = ["test", "possible", "values"];
        assert_eq!(provide_suggestions("hahaahahah", p_vals.iter()), Vec::<String>::new());
    }

}
