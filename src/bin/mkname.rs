use clap::Parser;
use colored::Colorize;
use funny_password_generator::words::{
    get_adjectives, get_professions, get_gerunds, get_objects, get_nouns,
    ANIMALS,
};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NameStyle {
    Classic,  // "The Grumpy Walrus"
    Title,    // "Narwhal the Magnificent"
    Epithet,  // "Vengeful Pirate of Chaos"
    Action,   // "The Scheming Wizard" or "Dragon Slayer"
    Versus,   // "Penguin vs Entropy"
    Bearer,   // "Keeper of the Staplers"
}

impl NameStyle {
    const ALL: &'static [NameStyle] = &[
        NameStyle::Classic,
        NameStyle::Title,
        NameStyle::Epithet,
        NameStyle::Action,
        NameStyle::Versus,
        NameStyle::Bearer,
    ];

    fn name(&self) -> &'static str {
        match self {
            NameStyle::Classic => "classic",
            NameStyle::Title => "title",
            NameStyle::Epithet => "epithet",
            NameStyle::Action => "action",
            NameStyle::Versus => "versus",
            NameStyle::Bearer => "bearer",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            NameStyle::Classic => "the_[adjective]_[subject]",
            NameStyle::Title => "[subject]_the_[adjective]",
            NameStyle::Epithet => "[adjective]_[subject]_of_[noun]",
            NameStyle::Action => "the_[gerund]_[subject] or [subject]_[verb]er",
            NameStyle::Versus => "[subject]_vs_[noun]",
            NameStyle::Bearer => "[title]_of_the_[objects]",
        }
    }

    fn example(&self) -> &'static str {
        match self {
            NameStyle::Classic => "the_grumpy_walrus",
            NameStyle::Title => "narwhal_the_magnificent",
            NameStyle::Epithet => "vengeful_pirate_of_chaos",
            NameStyle::Action => "the_scheming_wizard",
            NameStyle::Versus => "penguin_vs_entropy",
            NameStyle::Bearer => "keeper_of_the_rubber_ducks",
        }
    }
}

impl fmt::Display for NameStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for NameStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "classic" => Ok(NameStyle::Classic),
            "title" => Ok(NameStyle::Title),
            "epithet" => Ok(NameStyle::Epithet),
            "action" => Ok(NameStyle::Action),
            "versus" | "vs" => Ok(NameStyle::Versus),
            "bearer" => Ok(NameStyle::Bearer),
            _ => Err(format!("Unknown style: '{}'. Use --list-styles to see available options.", s)),
        }
    }
}

#[derive(Parser)]
#[command(name = "mkname")]
#[command(about = "Generate memorable character names using funny word combinations")]
struct Args {
    /// Number of names to generate
    #[arg(default_value = "1")]
    count: usize,

    /// Name style (classic, title, epithet, action, versus, bearer)
    #[arg(short = 's', long)]
    style: Option<String>,

    /// List all available styles
    #[arg(long)]
    list_styles: bool,

    /// Use only safe-for-work words (NSFW is enabled by default)
    #[arg(long)]
    sfw: bool,
}

/// Convert a name to snake_case (lowercase with underscores)
fn to_snake_case(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "_")
        .replace("-", "_")
}

/// Get a random subject (animal or profession)
fn random_subject<R: Rng>(rng: &mut R, nsfw: bool) -> &'static str {
    let professions = get_professions(nsfw);

    // 60% animals, 40% professions
    if rng.gen_bool(0.6) {
        ANIMALS.choose(rng).unwrap()
    } else {
        professions.choose(rng).copied().unwrap_or("wizard")
    }
}

/// Bearer titles for the "bearer" style
const BEARER_TITLES: &[&str] = &[
    "Guardian", "Keeper", "Bearer", "Warden", "Master",
    "Lord", "Lady", "Duke", "Duchess", "Baron",
    "Baroness", "Count", "Countess", "Herald", "Champion",
    "Protector", "Defender", "Custodian", "Steward", "Sentinel",
    "Overseer", "Caretaker", "Harbinger", "Envoy", "Emissary",
];

/// Convert a gerund to an "-er" noun (e.g., "juggling" -> "Juggler")
fn gerund_to_er_noun(gerund: &str) -> String {
    // Remove "ing" and add "er"
    if gerund.ends_with("ing") {
        let base = &gerund[..gerund.len() - 3];
        // Handle doubling rules: if the base ends in a consonant that was doubled
        // for the -ing form, we keep it (e.g., "running" -> "runner")
        // Most cases just need the "er" suffix
        format!("{}er", base)
    } else {
        format!("{}er", gerund)
    }
}

/// Generate a name in Classic style: "the_[adjective]_[subject]"
fn generate_classic<R: Rng>(rng: &mut R, nsfw: bool) -> String {
    let adjectives = get_adjectives(nsfw);
    let adj = adjectives.choose(rng).unwrap();
    let subject = random_subject(rng, nsfw);

    format!("the {} {}", adj, subject)
}

/// Generate a name in Title style: "[subject]_the_[adjective]"
fn generate_title<R: Rng>(rng: &mut R, nsfw: bool) -> String {
    let adjectives = get_adjectives(nsfw);
    let subject = random_subject(rng, nsfw);
    let adj = adjectives.choose(rng).unwrap();

    format!("{} the {}", subject, adj)
}

/// Generate a name in Epithet style: "[adjective]_[subject]_of_[noun]"
fn generate_epithet<R: Rng>(rng: &mut R, nsfw: bool) -> String {
    let adjectives = get_adjectives(nsfw);
    let nouns = get_nouns(nsfw);
    let adj = adjectives.choose(rng).unwrap();
    let subject = random_subject(rng, nsfw);
    let noun = nouns.choose(rng).unwrap();

    format!("{} {} of {}", adj, subject, noun)
}

/// Generate a name in Action style: "the_[gerund]_[subject]" or "[subject]_[verb]er"
fn generate_action<R: Rng>(rng: &mut R, nsfw: bool) -> String {
    let gerunds = get_gerunds(nsfw);
    let gerund = gerunds.choose(rng).unwrap();
    let subject = random_subject(rng, nsfw);

    // 50% chance for each variant
    if rng.gen_bool(0.5) {
        // "the_[gerund]_[subject]" - e.g., "the_scheming_wizard"
        format!("the {} {}", gerund, subject)
    } else {
        // "[subject]_[verb]er" - e.g., "dragon_slayer", "pickle_juggler"
        let er_noun = gerund_to_er_noun(gerund);
        format!("{} {}", subject, er_noun)
    }
}

/// Generate a name in Versus style: "[subject]_vs_[noun]"
fn generate_versus<R: Rng>(rng: &mut R, nsfw: bool) -> String {
    let nouns = get_nouns(nsfw);
    let subject = random_subject(rng, nsfw);
    let noun = nouns.choose(rng).unwrap();

    format!("{} vs {}", subject, noun)
}

/// Generate a name in Bearer style: "[title]_of_the_[objects]"
fn generate_bearer<R: Rng>(rng: &mut R, nsfw: bool) -> String {
    let objects = get_objects(nsfw);
    let title = BEARER_TITLES.choose(rng).unwrap();
    let object = objects.choose(rng).unwrap();

    format!("{} of the {}", title, object)
}

/// Generate a name with the specified style
fn generate_name(style: NameStyle, nsfw: bool) -> String {
    let mut rng = rand::thread_rng();

    match style {
        NameStyle::Classic => generate_classic(&mut rng, nsfw),
        NameStyle::Title => generate_title(&mut rng, nsfw),
        NameStyle::Epithet => generate_epithet(&mut rng, nsfw),
        NameStyle::Action => generate_action(&mut rng, nsfw),
        NameStyle::Versus => generate_versus(&mut rng, nsfw),
        NameStyle::Bearer => generate_bearer(&mut rng, nsfw),
    }
}

/// Generate a name with a random style
fn generate_random_style(nsfw: bool) -> String {
    let mut rng = rand::thread_rng();
    let style = *NameStyle::ALL.choose(&mut rng).unwrap();
    generate_name(style, nsfw)
}

fn main() {
    let args = Args::parse();

    // Handle --list-styles
    if args.list_styles {
        println!("Available name styles:\n");
        for style in NameStyle::ALL {
            println!("  {:<10} {} ", style.name().cyan(), style.description().dimmed());
            println!("            {}", format!("e.g., \"{}\"", style.example()).yellow());
            println!();
        }
        return;
    }

    // Parse style if specified
    let style: Option<NameStyle> = if let Some(ref name) = args.style {
        match NameStyle::from_str(name) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    // NSFW is default, --sfw disables it
    let nsfw = !args.sfw;

    // Generate names
    for _ in 0..args.count {
        let name = match style {
            Some(s) => generate_name(s, nsfw),
            None => generate_random_style(nsfw),
        };
        println!("{}", to_snake_case(&name).cyan().bold());
    }
}
