use crate::words::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Adjective,
    Animal,
    Profession,
    Noun,
    Gerund,      // "juggling" - used after "is/was"
    PresentVerb, // "juggles" - used directly after subject
    Object,
    Adverb,
    Article,
    Preposition,
    Connector,
    End,
}

impl Category {
    pub fn words(&self) -> &'static [&'static str] {
        match self {
            Category::Adjective => ADJECTIVES,
            Category::Animal => ANIMALS,
            Category::Profession => PROFESSIONS,
            Category::Noun => NOUNS,
            Category::Gerund => GERUNDS,
            Category::PresentVerb => PRESENT_VERBS,
            Category::Object => OBJECTS,
            Category::Adverb => ADVERBS,
            Category::Article => ARTICLES,
            Category::Preposition => PREPOSITIONS,
            Category::Connector => CONNECTORS,
            Category::End => &[],
        }
    }

    /// Returns true if this category counts toward the minimum word count
    pub fn counts_toward_minimum(&self) -> bool {
        !matches!(self, Category::Article | Category::Preposition | Category::Connector | Category::End)
    }
}

#[derive(Clone, Copy)]
pub struct Transition {
    pub category: Category,
    pub weight: u32,
}

impl Transition {
    pub const fn new(category: Category, weight: u32) -> Self {
        Self { category, weight }
    }
}

// START → always article (ensures proper sentence beginning)
static START_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Article, 100),
];

// article → adjective or directly to subject (animal/profession/noun)
static ARTICLE_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Adjective, 50),
    Transition::new(Category::Animal, 25),
    Transition::new(Category::Profession, 15),
    Transition::new(Category::Noun, 10),
];

// adjective → must complete noun phrase with a subject (no stacking)
static ADJECTIVE_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Animal, 50),
    Transition::new(Category::Profession, 30),
    Transition::new(Category::Noun, 20),
];

// animal → connector or present verb (END for prepositional phrases)
static ANIMAL_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Connector, 35),
    Transition::new(Category::PresentVerb, 55),
    Transition::new(Category::End, 10), // Allows ending in prep phrases
];

// profession → connector or present verb
static PROFESSION_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Connector, 35),
    Transition::new(Category::PresentVerb, 55),
    Transition::new(Category::End, 10),
];

// noun → connector or present verb
static NOUN_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Connector, 35),
    Transition::new(Category::PresentVerb, 55),
    Transition::new(Category::End, 10),
];

// connector ("is/was") → gerund ONLY (never adverb - "is juggling" not "is awkwardly")
static CONNECTOR_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Gerund, 100),
];

// gerund → object required (prepositions come after object, not directly after verb)
static GERUND_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Object, 90),
    Transition::new(Category::End, 10),
];

// present verb → object required
static PRESENT_VERB_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Object, 90),
    Transition::new(Category::End, 10),
];

// object → adverb, preposition to extend, or end
static OBJECT_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Adverb, 25),
    Transition::new(Category::Preposition, 25),
    Transition::new(Category::End, 50),
];

// adverb → end only (adverb completes the verb phrase, no more extensions)
static ADVERB_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::End, 100),
];

// preposition → always article (to start new noun phrase: "with the...")
static PREPOSITION_TRANSITIONS: &[Transition] = &[
    Transition::new(Category::Article, 100),
];

static EMPTY_TRANSITIONS: &[Transition] = &[];

pub fn get_transitions(from: Option<Category>) -> &'static [Transition] {
    match from {
        None => START_TRANSITIONS,
        Some(Category::Article) => ARTICLE_TRANSITIONS,
        Some(Category::Adjective) => ADJECTIVE_TRANSITIONS,
        Some(Category::Animal) => ANIMAL_TRANSITIONS,
        Some(Category::Profession) => PROFESSION_TRANSITIONS,
        Some(Category::Noun) => NOUN_TRANSITIONS,
        Some(Category::Connector) => CONNECTOR_TRANSITIONS,
        Some(Category::Gerund) => GERUND_TRANSITIONS,
        Some(Category::PresentVerb) => PRESENT_VERB_TRANSITIONS,
        Some(Category::Object) => OBJECT_TRANSITIONS,
        Some(Category::Adverb) => ADVERB_TRANSITIONS,
        Some(Category::Preposition) => PREPOSITION_TRANSITIONS,
        Some(Category::End) => EMPTY_TRANSITIONS,
    }
}
