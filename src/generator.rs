use rand::Rng;

use crate::templates::Template;
use crate::transitions::{get_transitions, Category, Transition};

/// Configuration for word selection during password generation.
#[derive(Clone, Copy, Debug, Default)]
pub struct WordConfig {
    /// Whether to include NSFW/adult words.
    pub nsfw: bool,
}

/// Generate a password using a randomly selected template.
/// This allows the caller to apply custom formatting (separators, capitalization, etc.)
pub fn generate_password(min_words: usize, config: &WordConfig) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let template = Template::random(&mut rng);
    template.generate(&mut rng, min_words, config)
}

/// Generate a password using a specific template.
pub fn generate_password_with_template(template: Template, min_words: usize, config: &WordConfig) -> Vec<String> {
    let mut rng = rand::thread_rng();
    template.generate(&mut rng, min_words, config)
}

/// Generate a password using the original declarative Markov-style generator.
/// This is the legacy implementation preserved for reference and comparison.
pub fn generate_password_declarative(min_words: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut words: Vec<&str> = Vec::new();
    let mut content_word_count = 0;
    let mut current_category: Option<Category> = None;
    let mut in_prep_phrase = false; // Track if we're inside a prepositional phrase

    loop {
        let transitions = get_transitions(current_category);
        if transitions.is_empty() {
            break;
        }

        let next_category = select_next_category(
            &mut rng,
            transitions,
            content_word_count,
            min_words,
            in_prep_phrase,
        );

        if next_category == Category::End {
            break;
        }

        // Track prepositional phrase state
        if next_category == Category::Preposition {
            in_prep_phrase = true;
        }

        let word_list = next_category.words();
        let word = word_list[rng.gen_range(0..word_list.len())];
        words.push(word);

        if next_category.counts_toward_minimum() {
            content_word_count += 1;
        }

        current_category = Some(next_category);
    }

    // Post-process to fix "a" → "an" before vowels
    fix_articles(&words)
}

/// Fix "a" to "an" when followed by a word starting with a vowel
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

fn starts_with_vowel_sound(word: &str) -> bool {
    word.chars()
        .next()
        .map(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .unwrap_or(false)
}

fn select_next_category(
    rng: &mut impl Rng,
    transitions: &[Transition],
    content_word_count: usize,
    min_words: usize,
    in_prep_phrase: bool,
) -> Category {
    let can_end = content_word_count >= min_words;

    // Filter transitions based on context
    let filtered: Vec<_> = transitions
        .iter()
        .filter(|t| {
            // Block END if we haven't reached min_words
            if !can_end && t.category == Category::End {
                return false;
            }
            // In prepositional phrases, don't allow verbs (creates run-on sentences)
            if in_prep_phrase {
                if matches!(t.category, Category::Connector | Category::PresentVerb | Category::Gerund) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // If we can't continue but haven't reached min_words, extend with preposition
    if filtered.is_empty() && !can_end {
        return Category::Preposition;
    }

    if filtered.is_empty() {
        return Category::End;
    }

    let total_weight: u32 = filtered.iter().map(|t| t.weight).sum();
    let mut roll = rng.gen_range(0..total_weight);

    for transition in &filtered {
        if roll < transition.weight {
            return transition.category;
        }
        roll -= transition.weight;
    }

    filtered.last().unwrap().category
}
