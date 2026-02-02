use clap::Parser;
use colored::Colorize;
use funny_password_generator::entropy::{
    calculate_bruteforce_entropy, calculate_formatting_entropy, estimate_crack_time,
    estimate_entropy_from_words_with_nsfw,
};
use funny_password_generator::generator::{generate_password, generate_password_with_template, WordConfig};
use funny_password_generator::templates::{render_commas, Template};
use funny_password_generator::words::ADJECTIVES;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Parser)]
#[command(name = "mkpass")]
#[command(about = "Generate memorable, funny passwords using Markov-style word chaining")]
struct Args {
    /// Minimum number of words in the password
    #[arg(default_value = "5")]
    min_words: usize,

    /// Number of passwords to generate
    #[arg(short = 'n', long, default_value = "3")]
    count: usize,

    /// Custom separator between words (default: space)
    #[arg(short, long, default_value = " ")]
    separator: String,

    /// Capitalize each word for CamelCase style
    #[arg(short, long)]
    capitalize: bool,

    /// Join words without spaces (equivalent to --separator "")
    #[arg(long)]
    no_spaces: bool,

    /// Show entropy (bits of randomness) alongside the password
    #[arg(long, short = 'e', default_value = "true")]
    show_entropy: bool,

    /// Use a specific template (declarative, imperative, question, warning,
    /// fragment, versus, since, somewhere, shouldnot, requires)
    #[arg(long, short = 't')]
    template: Option<String>,

    /// List all available templates
    #[arg(long)]
    list_templates: bool,

    /// Use only safe-for-work words (NSFW is enabled by default)
    #[arg(long)]
    sfw: bool,
}

/// Capitalize the first letter of a word
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Capitalize only the first letter of the entire string (skipping non-letter characters like quotes)
fn capitalize_first(s: &str) -> String {
    let mut result = String::new();
    let mut capitalized = false;

    for c in s.chars() {
        if !capitalized && c.is_alphabetic() {
            result.extend(c.to_uppercase());
            capitalized = true;
        } else {
            result.push(c);
        }
    }

    result
}

/// Get appropriate ending punctuation based on sentence type
fn get_punctuation(words: &[String]) -> &'static str {
    let mut rng = rand::thread_rng();

    if words.is_empty() {
        return ".";
    }

    // Get the first few words (lowercased) to detect sentence type
    let first_word = words[0].to_lowercase();
    let first_two: String = if words.len() >= 2 {
        format!("{} {}", words[0].to_lowercase(), words[1].to_lowercase())
    } else {
        first_word.clone()
    };

    // Question words - always use "?"
    const QUESTION_WORDS: &[&str] = &[
        "why", "how", "what", "when", "where", "who", "which",
    ];
    if QUESTION_WORDS.contains(&first_word.as_str()) {
        return "?";
    }

    // Imperative/command starters - use "!" or "..."
    const IMPERATIVE_STARTERS: &[&str] = &[
        "never", "always", "beware", "avoid", "respect", "fear",
        "watch", "stay", "keep", "bow", "surrender", "obey",
        "question", "challenge", "embrace", "reject", "honor",
        "distrust", "worship", "run", "hide", "confront", "welcome",
        "do not", "never trust", "always fear", "beware of",
    ];
    for starter in IMPERATIVE_STARTERS {
        if first_word == *starter || first_two.starts_with(starter) {
            const IMPERATIVE_PUNCT: &[&str] = &["!", "..."];
            return IMPERATIVE_PUNCT.choose(&mut rng).unwrap();
        }
    }

    // Warning starters (similar to imperative but more emphatic)
    const WARNING_STARTERS: &[&str] = &[
        "stop", "cease", "halt", "prevent", "resist", "abandon",
        "forget", "forsake", "shun", "evade", "escape", "flee",
        "renounce", "discontinue", "terminate", "end", "quit",
        "banish", "prohibit", "forbid", "disallow", "outlaw",
    ];
    if WARNING_STARTERS.contains(&first_word.as_str()) {
        const WARNING_PUNCT: &[&str] = &["!", "..."];
        return WARNING_PUNCT.choose(&mut rng).unwrap();
    }

    // Statement punctuation - no "?" for statements
    const STATEMENT_PUNCT: &[&str] = &[".", "!", "..."];
    STATEMENT_PUNCT.choose(&mut rng).unwrap()
}

/// Check if a word is an adjective
fn is_adjective(word: &str) -> bool {
    ADJECTIVES.contains(&word.to_lowercase().as_str())
}

/// Check if the first word of a password needs an article prefix
/// Returns true if the password starts with an adjective (which looks incomplete without an article)
fn needs_article_prefix(words: &[String]) -> bool {
    if words.is_empty() {
        return false;
    }
    // Check if first word is an adjective (strip quotes if present)
    let first_word = words[0].trim_matches('"');
    is_adjective(first_word)
}

/// Get an appropriate article ("A", "An", or "The") for the password
fn get_article_prefix(words: &[String]) -> &'static str {
    let mut rng = rand::thread_rng();

    // Use "The" most of the time (70%) or "A/An" (30%)
    if rng.gen_bool(0.7) {
        "The"
    } else {
        // Check if first word starts with vowel (after stripping quotes)
        let first_word = if !words.is_empty() {
            words[0].trim_matches('"')
        } else {
            ""
        };

        if first_word.chars().next()
            .map(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
            .unwrap_or(false)
        {
            "An"
        } else {
            "A"
        }
    }
}

/// Maybe wrap adjectives in sarcastic "air quotes" (30% chance per adjective)
fn maybe_airquote_adjectives(words: Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();

    words.into_iter().map(|word| {
        if is_adjective(&word) && rng.gen_bool(0.3) {
            format!("\"{}\"", word)
        } else {
            word
        }
    }).collect()
}

/// Maybe add a dramatic one-word opener (25% chance)
fn maybe_add_opener() -> Option<String> {
    let mut rng = rand::thread_rng();

    if !rng.gen_bool(0.25) {
        return None;
    }

    const OPENERS: &[&str] = &[
        "Beware", "Listen", "Warning", "Attention", "Alert",
        "Behold", "Alas", "Indeed", "Clearly", "Obviously",
        "Seriously", "Honestly", "Tragically", "Surprisingly",
        "Allegedly", "Reportedly", "Shockingly", "Naturally",
        "Curiously", "Mysteriously", "Suspiciously", "Frankly",
        "Incredibly", "Absurdly", "Hilariously", "Sadly",
        "Fortunately", "Unfortunately", "Remarkably", "Notably",
    ];

    let opener = OPENERS.choose(&mut rng).unwrap();
    let punctuation = if rng.gen_bool(0.5) { "!" } else { "..." };

    Some(format!("{}{}", opener, punctuation))
}

/// Format a password from words based on the provided options
fn format_password(words: Vec<String>, separator: &str, capitalize_all: bool, no_spaces: bool) -> String {
    let sep = if no_spaces { "" } else { separator };

    // Render comma markers into actual commas (e.g., "In the shadows, the penguin...")
    let words = render_commas(words);

    // Check if we need to add an article prefix BEFORE adding air quotes
    // (so we check the original first word, not a quoted version)
    let needs_article = needs_article_prefix(&words);

    // Determine punctuation before modifying words (based on sentence type)
    let punctuation = get_punctuation(&words);

    // Maybe add sarcastic air quotes to some adjectives
    let words = maybe_airquote_adjectives(words);

    // Prepend article if needed (after air quotes, so the article comes before any quoted word)
    let words = if needs_article {
        let article = get_article_prefix(&words);
        let mut new_words = vec![article.to_string()];
        new_words.extend(words);
        new_words
    } else {
        words
    };

    let joined = if capitalize_all {
        // CamelCase style - capitalize each word
        words.into_iter().map(|w| capitalize_word(&w)).collect::<Vec<_>>().join(sep)
    } else {
        // Normal style - just join, we'll capitalize first letter after
        words.join(sep)
    };

    // Capitalize first letter and add punctuation
    let capitalized = capitalize_first(&joined);
    let with_punctuation = format!("{}{}", capitalized, punctuation);

    // Maybe add a dramatic opener
    match maybe_add_opener() {
        Some(opener) => format!("{} {}", opener, with_punctuation),
        None => with_punctuation,
    }
}

fn main() {
    let args = Args::parse();

    // Handle --list-templates
    if args.list_templates {
        println!("Available templates:");
        for template in Template::ALL {
            println!("  {}", template.name());
        }
        return;
    }

    // Parse template if specified
    let template = if let Some(ref name) = args.template {
        match Template::from_name(name) {
            Some(t) => Some(t),
            None => {
                eprintln!("Error: Unknown template '{}'. Use --list-templates to see available options.", name);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    // Create word config based on CLI flags (NSFW is default, --sfw disables it)
    let word_config = WordConfig { nsfw: !args.sfw };

    // Generate 10x more candidates and keep the most secure ones
    let candidates_count = args.count * 10;
    let mut candidates: Vec<(String, f64, f64)> = Vec::with_capacity(candidates_count);

    for _ in 0..candidates_count {
        let words = match template {
            Some(t) => generate_password_with_template(t, args.min_words, &word_config),
            None => generate_password(args.min_words, &word_config),
        };

        // Count adjectives before formatting (for formatting entropy calculation)
        let adjective_count = words.iter().filter(|w| is_adjective(w)).count();

        let word_entropy = estimate_entropy_from_words_with_nsfw(&words, !args.sfw);
        let password = format_password(words, &args.separator, args.capitalize, args.no_spaces);

        // Total pattern entropy = word selection + formatting choices
        let formatting_entropy = calculate_formatting_entropy(&password, adjective_count);
        let pattern_entropy = word_entropy + formatting_entropy;

        let bruteforce_entropy = calculate_bruteforce_entropy(&password);

        candidates.push((password, pattern_entropy, bruteforce_entropy));
    }

    // Sort by pattern entropy (descending) - keep the most secure
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Take top 30% and shuffle them (adds unpredictability while maintaining quality)
    let top_count = (candidates.len() as f64 * 0.3).ceil() as usize;
    let mut top_candidates: Vec<_> = candidates.into_iter().take(top_count).collect();
    top_candidates.shuffle(&mut rand::thread_rng());

    // Pick randomly from the shuffled top 30%
    for (password, pattern_entropy, bruteforce_entropy) in top_candidates.into_iter().take(args.count) {
        if args.show_entropy {
            let pattern_time = estimate_crack_time(pattern_entropy);
            let bruteforce_time = estimate_crack_time(bruteforce_entropy);

            // Color the pattern crack time based on strength (this is the realistic threat)
            let pattern_time_colored = if pattern_entropy >= 40.0 {
                pattern_time.green()
            } else if pattern_entropy >= 25.0 {
                pattern_time.yellow()
            } else {
                pattern_time.red()
            };

            println!(
                "{}\n  {} {:.0} bits (crack: ~{})\n  {} {:.0} bits (crack: ~{})",
                password.cyan().bold(),
                "Pattern attack:".dimmed(),
                pattern_entropy,
                pattern_time_colored,
                "Brute force:".dimmed(),
                bruteforce_entropy,
                bruteforce_time.green()
            );
        } else {
            println!("{}", password.cyan());
        }
    }
}
