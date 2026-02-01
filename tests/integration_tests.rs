use funny_password_generator::entropy::{
    calculate_entropy, estimate_entropy_from_words, estimate_password_entropy, format_entropy,
};
use funny_password_generator::generator::{generate_password, generate_password_with_template, WordConfig};
use funny_password_generator::templates::Template;
use funny_password_generator::transitions::{get_transitions, Category};
use funny_password_generator::words::*;

fn default_config() -> WordConfig {
    WordConfig { nsfw: false }
}

// ============================================================================
// PASSWORD GENERATION TESTS
// ============================================================================

#[test]
fn test_generate_password_returns_non_empty() {
    let password = generate_password(4, &default_config());
    assert!(!password.is_empty(), "Password should not be empty");
}

#[test]
fn test_generate_password_minimum_words_4() {
    // Run multiple times to account for randomness
    // Use declarative template which guarantees article-first structure
    for _ in 0..10 {
        let password = generate_password_with_template(Template::Declarative, 4, &default_config());
        // Count content words (excluding articles, prepositions, connectors)
        let content_words: Vec<_> = password
            .iter()
            .filter(|w| !is_function_word(w))
            .collect();
        assert!(
            content_words.len() >= 4,
            "Password with min_words=4 should have at least 4 content words, got: {:?}",
            password
        );
    }
}

#[test]
fn test_generate_password_minimum_words_1() {
    for _ in 0..10 {
        let password = generate_password(1, &default_config());
        let content_words: Vec<_> = password
            .iter()
            .filter(|w| !is_function_word(w))
            .collect();
        assert!(
            content_words.len() >= 1,
            "Password with min_words=1 should have at least 1 content word, got: {:?}",
            password
        );
    }
}

#[test]
fn test_generate_password_minimum_words_0() {
    // min_words=0 should still generate something (starts with article)
    for _ in 0..10 {
        let password = generate_password(0, &default_config());
        // Should at least have the starting article
        assert!(
            !password.is_empty(),
            "Password with min_words=0 should not be empty"
        );
    }
}

#[test]
fn test_generate_password_large_minimum() {
    // Test with a larger minimum to ensure it extends properly
    // Use declarative template which supports proper extension
    for _ in 0..5 {
        let password = generate_password_with_template(Template::Declarative, 10, &default_config());
        let content_words: Vec<_> = password
            .iter()
            .filter(|w| !is_function_word(w))
            .collect();
        assert!(
            content_words.len() >= 10,
            "Password with min_words=10 should have at least 10 content words, got {} in: {:?}",
            content_words.len(),
            password
        );
    }
}

#[test]
fn test_generate_password_starts_with_article() {
    // Test specifically with declarative template which always starts with article
    for _ in 0..10 {
        let password = generate_password_with_template(Template::Declarative, 4, &default_config());
        let first_word = password.first().expect("Password should not be empty");
        // The first word should be "a", "an", or "the"
        // Note: "an" is transformed from "a" by fix_articles when followed by a vowel
        assert!(
            ARTICLES.contains(&first_word.as_str()) || first_word == "an",
            "Declarative password should start with an article, got: {}",
            first_word
        );
    }
}

#[test]
fn test_generate_password_produces_different_results() {
    // Generate multiple passwords and verify they're not all identical
    let config = default_config();
    let passwords: Vec<_> = (0..10).map(|_| generate_password(4, &config)).collect();
    let unique_count = passwords
        .iter()
        .map(|p| p.join(" "))
        .collect::<std::collections::HashSet<_>>()
        .len();

    // With randomness, we should get at least 2 different passwords in 10 tries
    assert!(
        unique_count >= 2,
        "Should generate different passwords, but got {} unique out of 10",
        unique_count
    );
}

// ============================================================================
// ARTICLE FIXING TESTS ("a" -> "an" before vowels)
// ============================================================================

#[test]
fn test_article_a_before_consonant_unchanged() {
    // Generate passwords and check that "a" before consonants stays "a"
    for _ in 0..50 {
        let password = generate_password(4, &default_config());
        for (i, word) in password.iter().enumerate() {
            if word == "a" {
                if let Some(next_word) = password.get(i + 1) {
                    let first_char = next_word.chars().next().unwrap_or(' ');
                    if !is_vowel(first_char) {
                        // "a" before consonant is correct
                        assert_eq!(word, "a");
                    }
                }
            }
        }
    }
}

#[test]
fn test_article_an_before_vowel() {
    // Generate passwords and check that "an" appears before vowel-starting words
    for _ in 0..50 {
        let password = generate_password(4, &default_config());
        for (i, word) in password.iter().enumerate() {
            if word == "an" {
                if let Some(next_word) = password.get(i + 1) {
                    let first_char = next_word.chars().next().unwrap_or(' ');
                    assert!(
                        is_vowel(first_char),
                        "\"an\" should only appear before vowel words, but found before: {}",
                        next_word
                    );
                }
            }
        }
    }
}

#[test]
fn test_no_incorrect_a_before_vowel() {
    // Ensure "a" never appears directly before a vowel-starting word
    for _ in 0..100 {
        let password = generate_password(4, &default_config());
        for (i, word) in password.iter().enumerate() {
            if word == "a" {
                if let Some(next_word) = password.get(i + 1) {
                    let first_char = next_word.chars().next().unwrap_or(' ');
                    assert!(
                        !is_vowel(first_char),
                        "\"a\" should not appear before vowel word: {}, in password: {:?}",
                        next_word,
                        password
                    );
                }
            }
        }
    }
}

// ============================================================================
// TRANSITION LOGIC TESTS
// ============================================================================

#[test]
fn test_start_transitions_lead_to_article() {
    let transitions = get_transitions(None);
    assert!(!transitions.is_empty(), "START should have transitions");

    // All start transitions should go to Article
    for t in transitions {
        assert_eq!(
            t.category,
            Category::Article,
            "START should only transition to Article"
        );
    }
}

#[test]
fn test_article_transitions_valid() {
    let transitions = get_transitions(Some(Category::Article));
    assert!(!transitions.is_empty(), "Article should have transitions");

    let valid_categories = [
        Category::Adjective,
        Category::Animal,
        Category::Profession,
        Category::Noun,
    ];

    for t in transitions {
        assert!(
            valid_categories.contains(&t.category),
            "Article transition {:?} not in valid set",
            t.category
        );
    }
}

#[test]
fn test_adjective_transitions_to_subject() {
    let transitions = get_transitions(Some(Category::Adjective));
    assert!(!transitions.is_empty(), "Adjective should have transitions");

    let valid_categories = [Category::Animal, Category::Profession, Category::Noun];

    for t in transitions {
        assert!(
            valid_categories.contains(&t.category),
            "Adjective transition {:?} not in valid set (should go to subject)",
            t.category
        );
    }
}

#[test]
fn test_connector_transitions_to_gerund() {
    let transitions = get_transitions(Some(Category::Connector));
    assert!(!transitions.is_empty(), "Connector should have transitions");

    for t in transitions {
        assert_eq!(
            t.category,
            Category::Gerund,
            "Connector should only transition to Gerund"
        );
    }
}

#[test]
fn test_preposition_transitions_to_article() {
    let transitions = get_transitions(Some(Category::Preposition));
    assert!(!transitions.is_empty(), "Preposition should have transitions");

    for t in transitions {
        assert_eq!(
            t.category,
            Category::Article,
            "Preposition should only transition to Article"
        );
    }
}

#[test]
fn test_adverb_transitions_to_end() {
    let transitions = get_transitions(Some(Category::Adverb));
    assert!(!transitions.is_empty(), "Adverb should have transitions");

    for t in transitions {
        assert_eq!(
            t.category,
            Category::End,
            "Adverb should only transition to End"
        );
    }
}

#[test]
fn test_end_has_no_transitions() {
    let transitions = get_transitions(Some(Category::End));
    assert!(
        transitions.is_empty(),
        "End should have no transitions, got {:?}",
        transitions.len()
    );
}

#[test]
fn test_all_transitions_have_positive_weights() {
    let categories = [
        None,
        Some(Category::Article),
        Some(Category::Adjective),
        Some(Category::Animal),
        Some(Category::Profession),
        Some(Category::Noun),
        Some(Category::Connector),
        Some(Category::Gerund),
        Some(Category::PresentVerb),
        Some(Category::Object),
        Some(Category::Adverb),
        Some(Category::Preposition),
    ];

    for from in categories {
        let transitions = get_transitions(from);
        for t in transitions {
            assert!(
                t.weight > 0,
                "Transition from {:?} to {:?} has non-positive weight: {}",
                from,
                t.category,
                t.weight
            );
        }
    }
}

// ============================================================================
// CATEGORY METHODS TESTS
// ============================================================================

#[test]
fn test_category_words_returns_correct_arrays() {
    assert_eq!(Category::Adjective.words(), ADJECTIVES);
    assert_eq!(Category::Animal.words(), ANIMALS);
    assert_eq!(Category::Profession.words(), PROFESSIONS);
    assert_eq!(Category::Noun.words(), NOUNS);
    assert_eq!(Category::Gerund.words(), GERUNDS);
    assert_eq!(Category::PresentVerb.words(), PRESENT_VERBS);
    assert_eq!(Category::Object.words(), OBJECTS);
    assert_eq!(Category::Adverb.words(), ADVERBS);
    assert_eq!(Category::Article.words(), ARTICLES);
    assert_eq!(Category::Preposition.words(), PREPOSITIONS);
    assert_eq!(Category::Connector.words(), CONNECTORS);
}

#[test]
fn test_category_end_has_empty_words() {
    assert!(
        Category::End.words().is_empty(),
        "End category should have empty word list"
    );
}

#[test]
fn test_category_words_not_empty_except_end() {
    let categories = [
        Category::Adjective,
        Category::Animal,
        Category::Profession,
        Category::Noun,
        Category::Gerund,
        Category::PresentVerb,
        Category::Object,
        Category::Adverb,
        Category::Article,
        Category::Preposition,
        Category::Connector,
    ];

    for cat in categories {
        assert!(
            !cat.words().is_empty(),
            "Category {:?} should have non-empty word list",
            cat
        );
    }
}

#[test]
fn test_counts_toward_minimum_content_words() {
    // These should count toward minimum
    assert!(Category::Adjective.counts_toward_minimum());
    assert!(Category::Animal.counts_toward_minimum());
    assert!(Category::Profession.counts_toward_minimum());
    assert!(Category::Noun.counts_toward_minimum());
    assert!(Category::Gerund.counts_toward_minimum());
    assert!(Category::PresentVerb.counts_toward_minimum());
    assert!(Category::Object.counts_toward_minimum());
    assert!(Category::Adverb.counts_toward_minimum());
}

#[test]
fn test_counts_toward_minimum_function_words() {
    // These should NOT count toward minimum
    assert!(!Category::Article.counts_toward_minimum());
    assert!(!Category::Preposition.counts_toward_minimum());
    assert!(!Category::Connector.counts_toward_minimum());
    assert!(!Category::End.counts_toward_minimum());
}

// ============================================================================
// WORD LIST TESTS
// ============================================================================

#[test]
fn test_word_lists_not_empty() {
    assert!(!ADJECTIVES.is_empty());
    assert!(!ANIMALS.is_empty());
    assert!(!PROFESSIONS.is_empty());
    assert!(!NOUNS.is_empty());
    assert!(!GERUNDS.is_empty());
    assert!(!PRESENT_VERBS.is_empty());
    assert!(!OBJECTS.is_empty());
    assert!(!ADVERBS.is_empty());
    assert!(!ARTICLES.is_empty());
    assert!(!PREPOSITIONS.is_empty());
    assert!(!CONNECTORS.is_empty());
}

#[test]
fn test_articles_contains_expected_values() {
    assert!(ARTICLES.contains(&"a"));
    assert!(ARTICLES.contains(&"the"));
}

#[test]
fn test_connectors_contains_expected_values() {
    assert!(CONNECTORS.contains(&"is"));
    assert!(CONNECTORS.contains(&"was"));
}

#[test]
fn test_no_empty_strings_in_word_lists() {
    let all_lists: Vec<(&str, &[&str])> = vec![
        ("ADJECTIVES", ADJECTIVES),
        ("ANIMALS", ANIMALS),
        ("PROFESSIONS", PROFESSIONS),
        ("NOUNS", NOUNS),
        ("GERUNDS", GERUNDS),
        ("PRESENT_VERBS", PRESENT_VERBS),
        ("OBJECTS", OBJECTS),
        ("ADVERBS", ADVERBS),
        ("ARTICLES", ARTICLES),
        ("PREPOSITIONS", PREPOSITIONS),
        ("CONNECTORS", CONNECTORS),
    ];

    for (name, list) in all_lists {
        for word in list {
            assert!(
                !word.is_empty(),
                "Found empty string in {}",
                name
            );
        }
    }
}

#[test]
fn test_gerunds_end_with_ing() {
    for word in GERUNDS {
        assert!(
            word.ends_with("ing"),
            "Gerund '{}' should end with 'ing'",
            word
        );
    }
}

#[test]
fn test_adverbs_end_with_ly() {
    for word in ADVERBS {
        assert!(
            word.ends_with("ly"),
            "Adverb '{}' should end with 'ly'",
            word
        );
    }
}

// ============================================================================
// ENTROPY TESTS
// ============================================================================

#[test]
fn test_entropy_is_positive_for_generated_passwords() {
    for _ in 0..10 {
        let password = generate_password(4, &default_config());
        let entropy = estimate_entropy_from_words(&password);
        assert!(
            entropy > 0.0,
            "Entropy should be positive, got: {} for {:?}",
            entropy,
            password
        );
    }
}

#[test]
fn test_entropy_increases_with_more_words() {
    // On average, more words should mean more entropy
    let mut short_entropies = Vec::new();
    let mut long_entropies = Vec::new();
    let config = default_config();

    for _ in 0..20 {
        let short_password = generate_password(2, &config);
        let long_password = generate_password(8, &config);

        short_entropies.push(estimate_entropy_from_words(&short_password));
        long_entropies.push(estimate_entropy_from_words(&long_password));
    }

    let avg_short: f64 = short_entropies.iter().sum::<f64>() / short_entropies.len() as f64;
    let avg_long: f64 = long_entropies.iter().sum::<f64>() / long_entropies.len() as f64;

    assert!(
        avg_long > avg_short,
        "Longer passwords should have more entropy on average: short={}, long={}",
        avg_short,
        avg_long
    );
}

#[test]
fn test_entropy_calculation_with_known_categories() {
    // Test with known categories to verify calculation
    let categories = vec![Category::Article, Category::Adjective, Category::Animal];
    let entropy = calculate_entropy(&categories);

    // Should be positive and reasonable
    assert!(entropy > 0.0);
    // Should be less than some reasonable maximum (e.g., 100 bits for 3 categories)
    assert!(entropy < 100.0);
}

#[test]
fn test_entropy_format() {
    assert_eq!(format_entropy(0.0), "~0 bits");
    assert_eq!(format_entropy(45.7), "~46 bits");
    assert_eq!(format_entropy(32.2), "~32 bits");
    assert_eq!(format_entropy(99.9), "~100 bits");
}

#[test]
fn test_entropy_from_password_string() {
    let password = "the grumpy walrus juggles pickles";
    let entropy = estimate_password_entropy(password);

    // Should be positive
    assert!(entropy > 0.0);
}

#[test]
fn test_entropy_handles_an_article() {
    // "an" should be recognized and handled as an article
    let password = "an elegant octopus debates spaghetti";
    let entropy = estimate_password_entropy(password);

    // Should be positive (meaning "an" was properly recognized)
    assert!(entropy > 0.0);
}

#[test]
fn test_entropy_consistency() {
    // Same password should always give same entropy
    let password = "the suspicious penguin hoards coffee";
    let entropy1 = estimate_password_entropy(password);
    let entropy2 = estimate_password_entropy(password);

    assert!(
        (entropy1 - entropy2).abs() < 0.0001,
        "Entropy should be consistent: {} vs {}",
        entropy1,
        entropy2
    );
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_generate_password_very_large_minimum() {
    // Test with a very large minimum (20 words)
    // Use declarative template which supports proper extension
    let password = generate_password_with_template(Template::Declarative, 20, &default_config());
    let content_words: Vec<_> = password
        .iter()
        .filter(|w| !is_function_word(w))
        .collect();

    assert!(
        content_words.len() >= 20,
        "Password with min_words=20 should have at least 20 content words, got {} in: {:?}",
        content_words.len(),
        password
    );
}

#[test]
fn test_password_words_are_valid() {
    // All words in generated passwords should be from known word lists or template-specific words
    for _ in 0..20 {
        let password = generate_password(4, &default_config());
        for word in &password {
            let word_str = word.as_str();
            let is_known = word_str == "an"
                || ARTICLES.contains(&word_str)
                || ADJECTIVES.contains(&word_str)
                || ANIMALS.contains(&word_str)
                || PROFESSIONS.contains(&word_str)
                || NOUNS.contains(&word_str)
                || GERUNDS.contains(&word_str)
                || PRESENT_VERBS.contains(&word_str)
                || OBJECTS.contains(&word_str)
                || ADVERBS.contains(&word_str)
                || PREPOSITIONS.contains(&word_str)
                || CONNECTORS.contains(&word_str)
                // New word lists for multi-template system
                || NUMBERS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || YEARS.contains(&word_str)
                || word_str.parse::<i32>().map(|y| (1950..=2030).contains(&y)).unwrap_or(false)  // Dynamic years
                || QUESTION_STARTERS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || IMPERATIVES.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || COMPARATIVES.contains(&word_str)
                || BASE_VERBS.contains(&word_str)
                // Extended word lists for diverse templates
                || WARNINGS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || CONFLICTS.contains(&word_str)
                || LOCATIONS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || PROHIBITIONS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || DEMANDS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || TIME_MARKERS.iter().any(|s| s.split_whitespace().any(|w| w == word_str))
                || STATUS_WORDS.contains(&word_str)
                || CONNECTORS_ACTIVE.contains(&word_str)
                // Template-specific fixed words
                || ["since", "somewhere", "should", "not", "requires", "more", "is"].contains(&word_str)
                // Comma marker (internal token)
                || word_str == "__COMMA__";

            assert!(
                is_known,
                "Unknown word '{}' in password: {:?}",
                word,
                password
            );
        }
    }
}

#[test]
fn test_subject_verb_transitions() {
    // After subject (animal/profession/noun), we should get connector or present verb
    let subjects = [Category::Animal, Category::Profession, Category::Noun];

    for subject in subjects {
        let transitions = get_transitions(Some(subject));
        let valid_categories = [Category::Connector, Category::PresentVerb, Category::End];

        for t in transitions {
            assert!(
                valid_categories.contains(&t.category),
                "{:?} transition {:?} not in valid set",
                subject,
                t.category
            );
        }
    }
}

#[test]
fn test_verb_transitions() {
    // After verbs, we should get object or end
    let verbs = [Category::Gerund, Category::PresentVerb];

    for verb in verbs {
        let transitions = get_transitions(Some(verb));
        let valid_categories = [Category::Object, Category::End];

        for t in transitions {
            assert!(
                valid_categories.contains(&t.category),
                "{:?} transition {:?} not in valid set",
                verb,
                t.category
            );
        }
    }
}

#[test]
fn test_object_transitions() {
    // After object, we can have adverb, preposition, or end
    let transitions = get_transitions(Some(Category::Object));
    let valid_categories = [Category::Adverb, Category::Preposition, Category::End];

    for t in transitions {
        assert!(
            valid_categories.contains(&t.category),
            "Object transition {:?} not in valid set",
            t.category
        );
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn is_function_word(word: &str) -> bool {
    ARTICLES.contains(&word)
        || PREPOSITIONS.contains(&word)
        || CONNECTORS.contains(&word)
        || word == "an" // "an" is also an article (transformed from "a")
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
