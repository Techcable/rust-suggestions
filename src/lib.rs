//! Minimal library to provide clap-style "Did you mean?" suggestions.
//!
//! The implementation is copied directly from clap.
//!
//! ## Examples
//! ```rust
//! let possible_vals = vec!["test", "possible", "values"];
//! let input = "tst";
//! let suggestions = suggestions::provide_suggestions(input, &possible_vals);
//! assert_eq!(suggestions, vec!["test"]);
//! // We have a convenience function to only pick only a single suggestion, giving `Some` or `None`
//! let single_suggestion = suggestions::provide_a_suggestion(input, &possible_vals);
//! assert_eq!(single_suggestion.unwrap(), "test");
//! ```
//!
//! ### Multiple matches
//! Sometimes, there may be multiple (good) suggestions.
//!
//! Consider the following example:
//!
//! ```rust
//! let possible_vals = vec!["testing", "tempo"];
//! let input = "teso"; // Sems ambiguous. Maybe multiple suggestions?
//! let suggestions = suggestions::provide_suggestions(input, &possible_vals);
//! // The implementation trys to order matches from "best" to "wort"
//! assert_eq!(suggestions, vec!["testing", "tempo"]);
//! ```
//!
//! Asking for a single suggestion here (`provide_a_suggestion`) would attempt to return the "best" one.
//! As you can immagine, that may not be what the user expects.
//! Therefore, it is best to stick with `provide_suggesetions`.
//!
//! ### No matches
//! If nothing is reasonably similar, asking for suggestions
//! will return `vec![]` or `None`.
//!
//! ```rust
//! let possible_vals = vec!["testing", "things", "here"];
//! let input = "--something-completely_different";
//! assert_eq!(suggestions::provide_a_suggestion(&input, &possible_vals), None)
//! ```
#![deny(missing_docs)]
use std::cmp::Ordering;
use std::convert::AsRef;

/// The confidence interval used to detect similarities.
const CONFIDENCE_THRESHOLD: f64 = 0.8;

/// Suggest a string that most is similar to `target`,
/// but is actually present in the `possible_values`.
///
/// Returns `None` if there is nothing in `possible_values` that is reasonably similar.
///
/// See also [provide_suggestions] to consider multiple possible suggestions (**recommended**).
///
/// If there are multiple ,possible
pub fn provide_a_suggestion<I, T>(target: &str, possible_values: I) -> Option<String>
    where T: AsRef<str>, I: IntoIterator<Item=T> {
    provide_suggestions(target, possible_values).pop()
}

/// Suggest strings that are similar to `target`,
/// but are actually present in the `possible_values`.
///
/// Returns an empty vector `vec![]` if there is nothing reasonbly similar.
///
/// The implementation sorts suggestions based on its internal notion of similarity (from best ->
/// worst).
///
/// See also [provide_a_suggestion], which only picks a single suggesetion.
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

