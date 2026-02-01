//! Template system for generating diverse password structures.
//!
//! This module provides 10 different sentence templates that create
//! varied and memorable passwords while maintaining grammatical correctness.

use rand::Rng;

use crate::generator::WordConfig;
use crate::words::*;

/// Marker token indicating a comma should follow the previous word.
/// Used for introductory phrases like "In the shadows, the penguin..."
pub const COMMA_MARKER: &str = "__COMMA__";

/// Convert comma markers into actual commas attached to the preceding word.
/// ["In", "the", "shadows", "__COMMA__", "the"] becomes ["In", "the", "shadows,", "the"]
pub fn render_commas(words: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for word in words {
        if word == COMMA_MARKER {
            // Attach comma to the last word
            if let Some(last) = result.last_mut() {
                last.push(',');
            }
        } else {
            result.push(word);
        }
    }

    result
}

/// The available password generation templates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Template {
    /// "the grumpy walrus juggles pickles"
    Declarative,
    /// "never trust a caffeinated accountant"
    Imperative,
    /// "why does the walrus overthink"
    Question,
    /// "do not provoke the sleepy wizard"
    Warning,
    /// "three wizards debating entropy"
    Fragment,
    /// "penguin versus bureaucracy"
    Versus,
    /// "desperately hoarding pickles since 1987"
    Since,
    /// "somewhere a penguin is plotting"
    Somewhere,
    /// "llamas should not negotiate"
    ShouldNot,
    /// "chaos requires more coffee"
    Requires,
}

impl Template {
    /// All available templates.
    pub const ALL: &'static [Template] = &[
        Template::Declarative,
        Template::Imperative,
        Template::Question,
        Template::Warning,
        Template::Fragment,
        Template::Versus,
        Template::Since,
        Template::Somewhere,
        Template::ShouldNot,
        Template::Requires,
    ];

    /// Select a random template.
    pub fn random(rng: &mut impl Rng) -> Self {
        let idx = rng.gen_range(0..Self::ALL.len());
        Self::ALL[idx]
    }

    /// Parse a template name from a string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "declarative" => Some(Template::Declarative),
            "imperative" => Some(Template::Imperative),
            "question" => Some(Template::Question),
            "warning" => Some(Template::Warning),
            "fragment" => Some(Template::Fragment),
            "versus" => Some(Template::Versus),
            "since" => Some(Template::Since),
            "somewhere" => Some(Template::Somewhere),
            "shouldnot" | "should-not" | "should_not" => Some(Template::ShouldNot),
            "requires" => Some(Template::Requires),
            _ => None,
        }
    }

    /// Get the name of this template.
    pub fn name(&self) -> &'static str {
        match self {
            Template::Declarative => "declarative",
            Template::Imperative => "imperative",
            Template::Question => "question",
            Template::Warning => "warning",
            Template::Fragment => "fragment",
            Template::Versus => "versus",
            Template::Since => "since",
            Template::Somewhere => "somewhere",
            Template::ShouldNot => "shouldnot",
            Template::Requires => "requires",
        }
    }

    /// Generate a password using this template.
    ///
    /// The `min_words` parameter is a hint for how many content words to include.
    /// Some templates have fixed structures and may not honor this exactly.
    pub fn generate(&self, rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<String> {
        // Since template returns Vec<String> directly due to dynamic year generation
        if matches!(self, Template::Since) {
            return generate_since(rng, min_words, config);
        }

        let words = match self {
            Template::Declarative => generate_declarative(rng, min_words, config),
            Template::Imperative => generate_imperative(rng, min_words, config),
            Template::Question => generate_question(rng, min_words, config),
            Template::Warning => generate_warning(rng, min_words, config),
            Template::Fragment => generate_fragment(rng, min_words, config),
            Template::Versus => generate_versus(rng, min_words, config),
            Template::Since => unreachable!(), // Handled above
            Template::Somewhere => generate_somewhere(rng, min_words, config),
            Template::ShouldNot => generate_should_not(rng, min_words, config),
            Template::Requires => generate_requires(rng, min_words, config),
        };
        fix_articles(&words)
    }
}

/// Helper to pick a random word from a slice.
fn pick<'a>(rng: &mut impl Rng, words: &[&'a str]) -> &'a str {
    words[rng.gen_range(0..words.len())]
}

/// Helper to pick a random word from a Vec.
fn pick_vec<'a>(rng: &mut impl Rng, words: &'a [&'static str]) -> &'static str {
    words[rng.gen_range(0..words.len())]
}

/// Fix "a" to "an" when followed by a word starting with a vowel.
fn fix_articles(words: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for (i, word) in words.iter().enumerate() {
        if *word == "a" {
            if let Some(next_word) = words.get(i + 1) {
                if starts_with_vowel_sound(next_word) {
                    result.push("an".to_string());
                    continue;
                }
            }
        }
        result.push(word.to_string());
    }

    result
}

/// Fix "a" to "an" when followed by a word starting with a vowel (for owned strings).
fn fix_articles_owned(words: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for (i, word) in words.iter().enumerate() {
        if word == "a" {
            if let Some(next_word) = words.get(i + 1) {
                if starts_with_vowel_sound(next_word) {
                    result.push("an".to_string());
                    continue;
                }
            }
        }
        result.push(word.clone());
    }

    result
}

fn starts_with_vowel_sound(word: &str) -> bool {
    word.chars()
        .next()
        .map(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .unwrap_or(false)
}

/// Pick a subject (animal, profession, or noun).
fn pick_subject(rng: &mut impl Rng, config: &WordConfig) -> &'static str {
    let choice = rng.gen_range(0..3);
    match choice {
        0 => pick(rng, ANIMALS),
        1 => {
            let professions = get_professions(config.nsfw);
            pick_vec(rng, &professions)
        }
        _ => {
            let nouns = get_nouns(config.nsfw);
            pick_vec(rng, &nouns)
        }
    }
}

/// Pick an object (from OBJECTS or NOUNS).
fn pick_object(rng: &mut impl Rng, config: &WordConfig) -> &'static str {
    if rng.gen_bool(0.7) {
        let objects = get_objects(config.nsfw);
        pick_vec(rng, &objects)
    } else {
        let nouns = get_nouns(config.nsfw);
        pick_vec(rng, &nouns)
    }
}

/// Generate a random year between 1950 and 2030.
fn random_year(rng: &mut impl Rng) -> String {
    rng.gen_range(1950..=2030).to_string()
}

// =============================================================================
// Template Generators
// =============================================================================

/// Count content words (non-function words)
fn count_content_words(words: &[&str]) -> usize {
    words.iter().filter(|w| !is_function_word(w)).count()
}

/// Check if a word is a function word (article, preposition, connector)
fn is_function_word(word: &str) -> bool {
    ARTICLES.contains(&word)
        || PREPOSITIONS.contains(&word)
        || CONNECTORS_ACTIVE.contains(&word)
}

/// Declarative: "the grumpy walrus juggles pickles"
/// Structure: [article] [adjective?] [subject] [verb] [prep?] [object] [adverb?]
fn generate_declarative(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adjectives = get_adjectives(config.nsfw);
    let present_verbs = get_present_verbs(config.nsfw);
    let adverbs = get_adverbs(config.nsfw);

    // Article
    words.push(pick(rng, ARTICLES));

    // Optional adjective (more likely if we need more words)
    if min_words > 3 || rng.gen_bool(0.6) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject
    words.push(pick_subject(rng, config));

    // Verb (present tense)
    let verb = pick_vec(rng, &present_verbs);
    words.push(verb);

    // Add preposition if verb is intransitive
    if let Some(prep) = get_verb_preposition(verb) {
        words.push(prep);
    }

    // Object
    words.push(pick_object(rng, config));

    // Optional adverb for extra length
    if count_content_words(&words) < min_words && rng.gen_bool(0.5) {
        words.push(pick_vec(rng, &adverbs));
    }

    // Extend with prepositional phrase if still too short
    while count_content_words(&words) < min_words {
        words.push(pick(rng, PREPOSITIONS));
        words.push(pick(rng, ARTICLES));
        // Always add adjective to maximize content words
        words.push(pick_vec(rng, &adjectives));
        words.push(pick_subject(rng, config));
    }

    words
}

/// Imperative: "never trust a caffeinated accountant"
/// Structure: [imperative phrase] [article?] [adjective?] [subject]
fn generate_imperative(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let imperatives = get_imperatives(config.nsfw);
    let adjectives = get_adjectives(config.nsfw);

    // Imperative starter (counts as multiple words for display but single for logic)
    let imperative = pick_vec(rng, &imperatives);
    let imperative_words: Vec<&str> = imperative.split_whitespace().collect();
    let ends_with_article = imperative_words.last()
        .map(|w| ARTICLES.contains(w))
        .unwrap_or(false);

    for w in imperative_words {
        words.push(w);
    }

    // Only add article if the imperative phrase doesn't already end with one
    if !ends_with_article {
        words.push(pick(rng, ARTICLES));
    }

    // Adjective (more likely if we need more words)
    if min_words > 4 || rng.gen_bool(0.7) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject
    words.push(pick_subject(rng, config));

    // Extend with prepositional phrase if needed
    while words.len() < min_words {
        words.push(pick(rng, PREPOSITIONS));
        words.push(pick_vec(rng, &adjectives));
        words.push(pick_subject(rng, config));
    }

    words
}

/// Question: "why does the walrus overthink"
/// Structure: [question starter] [article] [subject] [base verb]
fn generate_question(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adjectives = get_adjectives(config.nsfw);
    let base_verbs = get_base_verbs(config.nsfw);

    // Question starter (multiple words)
    let starter = pick(rng, QUESTION_STARTERS);
    for w in starter.split_whitespace() {
        words.push(w);
    }

    // Article
    words.push(pick(rng, ARTICLES));

    // Optional adjective
    if min_words > 5 || rng.gen_bool(0.5) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject
    words.push(pick_subject(rng, config));

    // Base verb
    let verb = pick_vec(rng, &base_verbs);
    words.push(verb);

    // Optional object for length (with preposition if verb is intransitive)
    if words.len() < min_words {
        if let Some(prep) = get_verb_preposition(verb) {
            words.push(prep);
        }
        words.push(pick_object(rng, config));
    }

    words
}

/// Warning: "do not provoke the sleepy wizard"
/// This is similar to imperative but specifically uses warning-style phrases
/// Structure: [warning phrase] [verb] [article] [adjective] [subject]
fn generate_warning(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let base_verbs = get_base_verbs(config.nsfw);
    let adjectives = get_adjectives(config.nsfw);

    // Use warning starters from pool
    let warning = pick(rng, WARNINGS);
    for w in warning.split_whitespace() {
        words.push(w);
    }

    // Add a base verb after the warning phrase
    words.push(pick_vec(rng, &base_verbs));

    // Article
    words.push(pick(rng, ARTICLES));

    // Adjective (almost always for warnings)
    if min_words > 5 || rng.gen_bool(0.8) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject
    words.push(pick_subject(rng, config));

    // Extend if needed
    while words.len() < min_words {
        words.push(pick(rng, PREPOSITIONS));
        words.push(pick_vec(rng, &adjectives));
        words.push(pick_object(rng, config));
    }

    words
}

/// Fragment: "three wizards juggle entropy"
/// Structure: [number/adverb] [adjective?] [subject(s)] [present verb] [object]
fn generate_fragment(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adverbs = get_adverbs(config.nsfw);
    let adjectives = get_adjectives(config.nsfw);
    let present_verbs = get_present_verbs(config.nsfw);

    // Start with number or adverb
    if rng.gen_bool(0.5) {
        let number = pick(rng, NUMBERS);
        for w in number.split_whitespace() {
            words.push(w);
        }
    } else {
        words.push(pick_vec(rng, &adverbs));
    }

    // Optional adjective
    if min_words > 4 || rng.gen_bool(0.5) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject (could be plural conceptually)
    words.push(pick_subject(rng, config));

    // Present verb (singular form for grammatical consistency)
    let verb = pick_vec(rng, &present_verbs);
    words.push(verb);

    // Add preposition if verb is intransitive
    if let Some(prep) = get_verb_preposition(verb) {
        words.push(prep);
    }

    // Object
    words.push(pick_object(rng, config));

    words
}

/// Versus: "penguin versus bureaucracy"
/// Structure: [subject] [conflict word] [subject/noun]
fn generate_versus(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adjectives = get_adjectives(config.nsfw);
    let nouns = get_nouns(config.nsfw);

    // Optional adjective for first subject
    if min_words > 3 || rng.gen_bool(0.4) {
        words.push(pick_vec(rng, &adjectives));
    }

    // First subject
    words.push(pick_subject(rng, config));

    // Conflict word (randomized from pool)
    words.push(pick(rng, CONFLICTS));

    // Optional adjective for second subject
    if min_words > 4 || rng.gen_bool(0.4) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Second subject (often abstract noun for humor)
    if rng.gen_bool(0.6) {
        words.push(pick_vec(rng, &nouns));
    } else {
        words.push(pick_subject(rng, config));
    }

    words
}

/// Since: "desperately hoarding pickles since 1987"
/// Structure: [adverb?] [gerund] [prep?] [object] [time marker or "since" + dynamic year]
fn generate_since(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let adverbs = get_adverbs(config.nsfw);
    let gerunds = get_gerunds(config.nsfw);
    let objects = get_objects(config.nsfw);
    let nouns = get_nouns(config.nsfw);

    // Optional adverb
    if min_words > 4 || rng.gen_bool(0.6) {
        words.push(pick_vec(rng, &adverbs).to_string());
    }

    // Gerund
    let gerund = pick_vec(rng, &gerunds);
    words.push(gerund.to_string());

    // Add preposition if verb is intransitive
    if let Some(prep) = get_verb_preposition(gerund) {
        words.push(prep.to_string());
    }

    // Object (inline pick_object logic for owned strings)
    if rng.gen_bool(0.7) {
        words.push(pick_vec(rng, &objects).to_string());
    } else {
        words.push(pick_vec(rng, &nouns).to_string());
    }

    // 50% chance to use dynamic year, 50% chance to use TIME_MARKERS pool
    if rng.gen_bool(0.5) {
        // Use dynamic year generation
        words.push("since".to_string());
        words.push(random_year(rng));
    } else {
        // Use time marker from pool
        let time_marker = pick(rng, TIME_MARKERS);
        for w in time_marker.split_whitespace() {
            words.push(w.to_string());
        }
    }

    // Apply article fixing (a -> an before vowels)
    fix_articles_owned(&words)
}

/// Somewhere: "Somewhere, a penguin is plotting"
/// Structure: [location], [article] [adjective?] [subject] [connector] [gerund]
fn generate_somewhere(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adjectives = get_adjectives(config.nsfw);
    let gerunds = get_gerunds(config.nsfw);

    // Location (randomized from pool)
    let location = pick(rng, LOCATIONS);
    for w in location.split_whitespace() {
        words.push(w);
    }

    // Comma after introductory location phrase
    words.push(COMMA_MARKER);

    // Article
    words.push(pick(rng, ARTICLES));

    // Optional adjective
    if min_words > 5 || rng.gen_bool(0.5) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject
    words.push(pick_subject(rng, config));

    // Connector (randomized from pool)
    words.push(pick(rng, CONNECTORS_ACTIVE));

    // Gerund
    let gerund = pick_vec(rng, &gerunds);
    words.push(gerund);

    // Optional object for length (with preposition if verb is intransitive)
    if words.len() < min_words {
        if let Some(prep) = get_verb_preposition(gerund) {
            words.push(prep);
        }
        words.push(pick_object(rng, config));
    }

    words
}

/// ShouldNot: "llamas should not negotiate"
/// Structure: [subject] [prohibition] [base verb]
fn generate_should_not(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adjectives = get_adjectives(config.nsfw);
    let prohibitions = get_prohibitions(config.nsfw);
    let base_verbs = get_base_verbs(config.nsfw);

    // Optional adjective
    if min_words > 4 || rng.gen_bool(0.4) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject
    words.push(pick_subject(rng, config));

    // Prohibition (randomized from pool)
    let prohibition = pick_vec(rng, &prohibitions);
    for w in prohibition.split_whitespace() {
        words.push(w);
    }

    // Base verb
    let verb = pick_vec(rng, &base_verbs);
    words.push(verb);

    // Optional object (with preposition if verb is intransitive)
    if words.len() < min_words || rng.gen_bool(0.5) {
        if let Some(prep) = get_verb_preposition(verb) {
            words.push(prep);
        }
        words.push(pick_object(rng, config));
    }

    words
}

/// Requires: "chaos requires more coffee"
/// Structure: [subject/noun] [demand phrase] [object]
fn generate_requires(rng: &mut impl Rng, min_words: usize, config: &WordConfig) -> Vec<&'static str> {
    let mut words: Vec<&str> = Vec::new();
    let adjectives = get_adjectives(config.nsfw);
    let nouns = get_nouns(config.nsfw);
    let demands = get_demands(config.nsfw);

    // Optional adjective
    if min_words > 4 || rng.gen_bool(0.3) {
        words.push(pick_vec(rng, &adjectives));
    }

    // Subject (often abstract noun)
    if rng.gen_bool(0.6) {
        words.push(pick_vec(rng, &nouns));
    } else {
        words.push(pick_subject(rng, config));
    }

    // Demand phrase (randomized from pool)
    let demand = pick_vec(rng, &demands);
    for w in demand.split_whitespace() {
        words.push(w);
    }

    // Object
    words.push(pick_object(rng, config));

    words
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn get_test_rng() -> StdRng {
        StdRng::seed_from_u64(12345)
    }

    fn get_default_config() -> WordConfig {
        WordConfig { nsfw: false }
    }

    #[test]
    fn test_template_from_name() {
        assert_eq!(Template::from_name("declarative"), Some(Template::Declarative));
        assert_eq!(Template::from_name("IMPERATIVE"), Some(Template::Imperative));
        assert_eq!(Template::from_name("should-not"), Some(Template::ShouldNot));
        assert_eq!(Template::from_name("shouldnot"), Some(Template::ShouldNot));
        assert_eq!(Template::from_name("invalid"), None);
    }

    #[test]
    fn test_template_random() {
        let mut rng = get_test_rng();
        let template = Template::random(&mut rng);
        assert!(Template::ALL.contains(&template));
    }

    #[test]
    fn test_declarative_generates_words() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Declarative.generate(&mut rng, 4, &config);
        assert!(words.len() >= 4, "Expected at least 4 words, got {}", words.len());
    }

    #[test]
    fn test_imperative_generates_words() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Imperative.generate(&mut rng, 4, &config);
        assert!(!words.is_empty());
    }

    #[test]
    fn test_question_generates_words() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Question.generate(&mut rng, 4, &config);
        assert!(!words.is_empty());
        // Should start with a question word
        let first_words: Vec<_> = QUESTION_STARTERS.iter()
            .flat_map(|s| s.split_whitespace())
            .collect();
        assert!(first_words.contains(&words[0].as_str()));
    }

    #[test]
    fn test_since_contains_time_marker() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Since.generate(&mut rng, 4, &config);
        // Should contain words from TIME_MARKERS
        let has_time_marker = words.iter().any(|w| {
            TIME_MARKERS.iter().any(|marker| marker.split_whitespace().any(|mw| mw == w))
        });
        assert!(has_time_marker, "Since template should contain a time marker, got: {:?}", words);
    }

    #[test]
    fn test_somewhere_starts_with_location() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Somewhere.generate(&mut rng, 4, &config);
        // Should start with a word from LOCATIONS
        let starts_with_location = LOCATIONS.iter().any(|loc| {
            loc.split_whitespace().next().map(|w| w == words[0]).unwrap_or(false)
        });
        assert!(starts_with_location, "Somewhere template should start with a location word, got: {:?}", words);
    }

    #[test]
    fn test_should_not_contains_prohibition() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::ShouldNot.generate(&mut rng, 4, &config);
        // Should contain words from PROHIBITIONS
        let has_prohibition = words.iter().any(|w| {
            PROHIBITIONS.iter().any(|p| p.split_whitespace().any(|pw| pw == w))
        });
        assert!(has_prohibition, "ShouldNot template should contain a prohibition word, got: {:?}", words);
    }

    #[test]
    fn test_requires_contains_demand() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Requires.generate(&mut rng, 4, &config);
        // Should contain words from DEMANDS
        let has_demand = words.iter().any(|w| {
            DEMANDS.iter().any(|d| d.split_whitespace().any(|dw| dw == w))
        });
        assert!(has_demand, "Requires template should contain a demand word, got: {:?}", words);
    }

    #[test]
    fn test_versus_contains_conflict() {
        let mut rng = get_test_rng();
        let config = get_default_config();
        let words = Template::Versus.generate(&mut rng, 3, &config);
        let has_conflict = words.iter().any(|w| CONFLICTS.contains(&w.as_str()));
        assert!(has_conflict, "Versus template should contain a conflict word, got: {:?}", words);
    }

    #[test]
    fn test_article_fixing() {
        let words = vec!["a", "elegant", "octopus"];
        let fixed = fix_articles(&words);
        assert_eq!(fixed[0], "an");
    }
}
