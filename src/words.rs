pub const ADJECTIVES: &[&str] = &[
    // Emotional states
    "suspicious", "grumpy", "anxious", "paranoid", "smug", "indignant",
    "baffled", "bewildered", "exasperated", "melancholy", "euphoric",
    "mortified", "scandalized", "apoplectic", "despondent", "elated",
    "flabbergasted", "gobsmacked", "horrified", "incensed", "jubilant",
    "livid", "nostalgic", "overwhelmed", "perturbed", "resentful",
    "traumatized", "unimpressed", "vengeful", "wistful", "zealous",
    // Physical traits
    "wobbly", "lumpy", "crunchy", "squishy", "gangly", "lanky",
    "rotund", "scrawny", "spindly", "bulbous", "wrinkly", "jiggly",
    "prickly", "rubbery", "gelatinous", "bristly", "stubby", "gangrenous",
    "lopsided", "crooked", "bulging", "deflated", "swollen", "shriveled",
    // Attitudes
    "pretentious", "snooty", "earnest", "cynical", "naive", "pompous",
    "haughty", "condescending", "insufferable", "holier-than-thou", "smirking",
    "pedantic", "contrarian", "sanctimonious", "obsequious", "aloof",
    "dismissive", "judgmental", "self-righteous", "melodramatic", "theatrical",
    // States
    "caffeinated", "decaffeinated", "overworked", "haunted", "cursed",
    "hexed", "enchanted", "possessed", "hungover", "sunburned", "frostbitten",
    "electrified", "magnetized", "radioactive", "contaminated", "pickled",
    "marinated", "fossilized", "petrified", "mummified", "carbonated",
    // Unusual/fantastical
    "interdimensional", "sentient", "rogue", "feral", "domesticated",
    "telepathic", "omniscient", "spectral", "ethereal", "eldritch",
    "cosmic", "quantum", "antimatter", "holographic", "pixelated",
    "glitched", "corrupted", "bootleg", "counterfeit", "unlicensed",
    // Funny/absurd
    "unhinged", "chaotic", "legendary", "mythical", "forbidden",
    "unauthorized", "contraband", "classified", "redacted", "dubious",
    "questionable", "sketchy", "shady", "sus", "vintage", "antique",
    "artisanal", "organic", "free-range", "grass-fed", "locally-sourced",
    // Additional variety
    "whimsical", "peculiar", "eccentric", "flamboyant", "dapper",
    "mischievous", "mysterious", "sparkly", "fuzzy", "fluffy",
    "sleepy", "cranky", "majestic", "elegant", "startled",
    "hypnotic", "ridiculous", "philosophical", "turbulent", "sneaky",
    "confused", "enthusiastic", "dramatic", "optimistic", "sarcastic",
    // Mythical/fantasy
    "arcane", "celestial", "infernal", "abyssal", "seraphic",
    "demonic", "draconic", "fey", "sylvan", "vampiric",
    "lycanthropic", "necromantic", "runic", "astral", "primordial",
    "elven", "orcish", "dwarven", "goblin", "chimeric",
    "phantasmal", "wyrd", "otherworldly", "faerie", "basilisk",
    // Sci-fi/futuristic
    "cybernetic", "biomechanical", "nanoscale", "plasma-infused", "gravitonic",
    "hyperspace", "subspace", "transdimensional", "warp-capable", "cryogenic",
    "bionic", "synthetic", "xenomorphic", "chrono-shifted", "neural-linked",
    "fusion-powered", "photonic", "tachyonic", "void-touched", "zero-gravity",
    // Dramatic states
    "doomed", "forsaken", "exiled", "tormented", "damned",
    "fallen", "banished", "shattered", "sundered", "unraveled",
    "condemned", "blighted", "ravaged", "bereft", "forlorn",
    "desolate", "stricken", "afflicted", "beleaguered", "imperiled",
    // Unusual physical properties
    "crystalline", "phosphorescent", "iridescent", "opalescent", "translucent",
    "obsidian", "mercurial", "vitreous", "gossamer", "nebulous",
    "incandescent", "luminescent", "bioluminescent", "prismatic", "chromatic",
    "molten", "viscous", "vaporous", "calcified", "vitrified",
    // Archaic/poetic
    "wretched", "accursed", "hallowed", "benighted", "besmirched",
    "betrothed", "bewitched", "ensorcelled", "beguiled", "bedazzled",
    "erstwhile", "hitherto", "thrice-blessed", "manifold", "veritable",
    "wroth", "vexed", "smitten", "smote", "beholden",
    // Elemental/nature
    "tempest-born", "storm-wrought", "frost-kissed", "flame-touched", "tide-bound",
    "wind-swept", "earth-shaking", "thunder-voiced", "lightning-fast", "glacier-cold",
    "volcano-forged", "ocean-deep", "mountain-old", "forest-born", "desert-baked",
    // Cosmic/vast
    "nebular", "galactic", "stellar", "planetary", "lunar",
    "solar", "eclipsed", "supernova", "quasar-bright", "pulsar",
    "interstellar", "extragalactic", "multiverse", "omnidimensional", "infinite",
    // Dark/sinister
    "malevolent", "nefarious", "insidious", "sinister", "ominous",
    "malignant", "baleful", "tenebrous", "umbral", "shadow-clad",
    "night-born", "void-spawned", "death-touched", "soul-rending", "blood-soaked",
];

pub const ANIMALS: &[&str] = &[
    // Unusual/weird animals
    "axolotl", "tardigrade", "blobfish", "pangolin", "aardvark",
    "hagfish", "nudibranch", "tarsier", "ayeaye", "hoatzin",
    "kakapo", "proboscis", "glassfrog", "geoduck", "mudskipper",
    "sunfish", "oarfish", "lamprey", "lumpfish", "frogfish",
    // Common but funny
    "goose", "pigeon", "seagull", "squirrel", "chipmunk",
    "possum", "skunk", "badger", "beaver", "groundhog",
    "porcupine", "pelican", "penguin", "puffin", "toucan",
    "raccoon", "opossum", "woodchuck", "gopher", "marmot",
    // Exotic favorites
    "capybara", "quokka", "wombat", "platypus", "koala",
    "kangaroo", "wallaby", "lemur", "sloth", "tapir",
    "okapi", "binturong", "fossa", "kinkajou", "coati",
    "tenrec", "solenodon", "dhole", "babirusa", "gerenuk",
    // Mythical-sounding real ones
    "narwhal", "shoebill", "cassowary", "echidna", "gharial",
    "tuatara", "saiga", "markhor", "dikdik", "numbat",
    "bilby", "quoll", "colugo", "olinguito", "uakari",
    "muntjac", "chevrotain", "bushbaby", "loris", "potoo",
    // Sea creatures
    "octopus", "squid", "cuttlefish", "nautilus", "jellyfish",
    "seahorse", "pufferfish", "anglerfish", "lobster", "mantisshrimp",
    "manatee", "dugong", "walrus", "orca", "dolphin",
    "porpoise", "seal", "otter", "crab", "shrimp",
    "starfish", "urchin", "clam", "oyster", "abalone",
    // Birds
    "flamingo", "peacock", "ostrich", "emu", "kiwi",
    "owl", "raven", "crow", "magpie", "parrot",
    "cockatoo", "macaw", "falcon", "hawk", "eagle",
    "vulture", "condor", "stork", "heron", "ibis",
    "albatross", "kingfisher", "woodpecker", "hummingbird", "nightjar",
    // Mammals - large
    "moose", "elk", "caribou", "bison", "yak",
    "llama", "alpaca", "camel", "hippo", "rhino",
    "elephant", "giraffe", "zebra", "hyena", "jackal",
    "dingo", "coyote", "wolf", "fox", "wolverine",
    // Mammals - small/medium
    "meerkat", "mongoose", "weasel", "ferret", "stoat",
    "hedgehog", "mole", "shrew", "vole", "dormouse",
    "chinchilla", "hamster", "gerbil", "panda", "bear",
    "anteater", "armadillo", "corgi", "dachshund", "pug",
    // Reptiles and amphibians
    "iguana", "gecko", "chameleon", "komodo", "crocodile",
    "alligator", "tortoise", "python", "cobra", "viper",
    "salamander", "newt", "toad", "treefrog", "caecilian",
    // Insects and arachnids
    "mantis", "beetle", "weevil", "cicada", "katydid",
    "firefly", "dragonfly", "butterfly", "moth", "bumblebee",
    "scorpion", "tarantula", "centipede", "millipede", "cricket",
    // Primates
    "gorilla", "chimp", "orangutan", "gibbon", "mandrill",
    "baboon", "macaque", "tamarin", "marmoset", "capuchin",
    // Mythical and legendary creatures
    "dragon", "phoenix", "griffin", "basilisk", "hydra",
    "kraken", "chimera", "minotaur", "centaur", "unicorn",
    "pegasus", "wyvern", "wyrm", "leviathan", "behemoth",
    "cerberus", "manticore", "sphinx", "gorgon", "harpy",
    "siren", "mermaid", "cyclops", "banshee", "wraith",
    "specter", "golem", "gargoyle", "djinn", "ifrit",
    // Cryptids and mysterious creatures
    "bigfoot", "sasquatch", "mothman", "chupacabra", "yeti",
    "nessie", "jackalope", "bunyip", "mokele", "skunk-ape",
    "jersey-devil", "wendigo", "skinwalker", "dogman", "fresno",
    // Fantasy races as creatures
    "goblin", "troll", "ogre", "imp", "gremlin",
    "kobold", "gnoll", "orc", "hobgoblin", "bugbear",
    "pixie", "sprite", "brownie", "leprechaun", "dryad",
    "nymph", "satyr", "faun", "changeling", "redcap",
    // Legendary beasts from world cultures
    "kitsune", "thunderbird", "roc", "simurgh", "ziz",
    "baku", "kappa", "tengu", "tanuki", "jorogumo",
    "naga", "garuda", "qilin", "fenghuang", "pixiu",
    "barghest", "selkie", "kelpie", "puca", "cait-sith",
    // Unusual real animals (not already listed)
    "okapi", "pangolin", "quokka", "fossa", "binturong",
    "zorilla", "caracal", "serval", "ocelot", "margay",
    "quetzal", "hoopoe", "secretary", "marabou", "adjutant",
    "jerboa", "pika", "desman", "solenodon", "hutia",
];

pub const PROFESSIONS: &[&str] = &[
    // Serious jobs (funny in context)
    "accountant", "auditor", "bureaucrat", "actuary", "analyst",
    "consultant", "manager", "executive", "administrator", "coordinator",
    "inspector", "regulator", "compliance", "treasurer", "secretary",
    "receptionist", "clerk", "paralegal", "notary", "registrar",
    // Historical professions
    "blacksmith", "alchemist", "scribe", "herald", "apothecary",
    "cooper", "chandler", "tanner", "fletcher", "wheelwright",
    "farrier", "cobbler", "milliner", "haberdasher", "cordwainer",
    "ostler", "lamplighter", "crier", "jester", "minstrel",
    // Unusual modern jobs
    "cryptographer", "sommelier", "taxidermist", "actuary", "epidemiologist",
    "entomologist", "etymologist", "oenologist", "ornithologist", "mycologist",
    "archivist", "curator", "conservator", "docent", "genealogist",
    "futurist", "ethicist", "ombudsman", "mediator", "arbitrator",
    // Fantasy-adjacent
    "wizard", "bard", "ranger", "paladin", "druid",
    "warlock", "sorcerer", "necromancer", "enchanter", "conjurer",
    "alchemist", "artificer", "illusionist", "diviner", "shaman",
    "oracle", "seer", "prophet", "sage", "mystic",
    // Blue collar
    "plumber", "electrician", "mechanic", "welder", "carpenter",
    "mason", "roofer", "glazier", "plasterer", "tiler",
    "locksmith", "machinist", "millwright", "rigger", "ironworker",
    "longshoreman", "stevedore", "trucker", "forklift", "crane",
    // White collar
    "lawyer", "doctor", "engineer", "architect", "professor",
    "scientist", "researcher", "economist", "statistician", "mathematician",
    "physicist", "chemist", "biologist", "geologist", "meteorologist",
    "astronomer", "linguist", "anthropologist", "sociologist", "psychologist",
    // Artistic
    "sculptor", "painter", "composer", "playwright", "novelist",
    "poet", "illustrator", "animator", "choreographer", "conductor",
    "puppeteer", "ventriloquist", "mime", "acrobat", "juggler",
    "calligrapher", "ceramicist", "glassblower", "woodcarver", "weaver",
    // Service and hospitality
    "barista", "bartender", "sommelier", "concierge", "butler",
    "valet", "chauffeur", "doorman", "bellhop", "maitre",
    "chef", "baker", "butcher", "fishmonger", "cheesemonger",
    "florist", "jeweler", "watchmaker", "optician", "tailor",
    // Adventure and danger
    "pirate", "ninja", "viking", "gladiator", "mercenary",
    "bounty", "smuggler", "spy", "saboteur", "assassin",
    "explorer", "mountaineer", "diver", "astronaut", "stuntman",
    "detective", "investigator", "warden", "ranger", "marshal",
    // Miscellaneous interesting
    "librarian", "archivist", "cartographer", "navigator", "surveyor",
    "beekeeper", "zookeeper", "falconer", "gamekeeper", "forester",
    "gravedigger", "undertaker", "embalmer", "coroner", "mortician",
    "hypnotist", "exorcist", "medium", "clairvoyant", "mentalist",
    // Religious/mystical
    "prophet", "martyr", "heretic", "inquisitor", "crusader",
    "pilgrim", "monk", "nun", "cardinal", "pope",
    "archbishop", "abbot", "abbess", "deacon", "acolyte",
    "confessor", "zealot", "apostle", "disciple", "ascetic",
    // Space/sci-fi
    "astronaut", "cosmonaut", "xenobiologist", "starship-captain", "space-pirate",
    "asteroid-miner", "terraformer", "starchart-navigator", "exoplanetologist", "cyborg",
    "android", "replicant", "time-traveler", "dimension-hopper", "warp-engineer",
    "alien-diplomat", "galactic-smuggler", "clone-wrangler", "nanomancer", "biotech-surgeon",
    // Medieval/fantasy
    "squire", "page", "herald", "executioner", "torturer",
    "dungeon-master", "drawbridge-operator", "moat-keeper", "castle-jester", "dragon-tamer",
    "gargoyle-whisperer", "siege-engineer", "court-poisoner", "royal-taster", "jousting-champion",
    "trebuchet-operator", "chainmail-polisher", "quest-giver", "tavern-wench", "stable-master",
    // Unusual historical
    "plague-doctor", "food-taster", "sin-eater", "chimney-sweep", "resurrectionist",
    "ratcatcher", "leech-collector", "gong-farmer", "knocker-upper", "mudlark",
    "tosher", "whipping-boy", "fuller", "groom-of-the-stool", "town-crier",
    // Modern unusual
    "influencer", "podcaster", "cryptobro", "doomscroller", "content-creator",
    "social-media-manager", "meme-lord", "e-sports-athlete", "twitch-streamer", "tiktok-star",
    "blockchain-evangelist", "growth-hacker", "vibe-curator", "wellness-guru", "life-coach",
    "asmr-artist", "virtual-landlord", "nft-dealer", "metaverse-architect", "algorithm-whisperer",
];

pub const NOUNS: &[&str] = &[
    // Abstract concepts
    "entropy", "chaos", "bureaucracy", "democracy", "capitalism",
    "socialism", "anarchy", "hierarchy", "monarchy", "tyranny",
    "liberty", "equality", "justice", "karma", "fate",
    "destiny", "irony", "sarcasm", "hypocrisy", "empathy",
    // Feelings/states
    "nostalgia", "existentialism", "paranoia", "anxiety", "euphoria",
    "melancholy", "ennui", "apathy", "angst", "serenity",
    "despair", "bliss", "dread", "confusion", "bewilderment",
    "frustration", "contentment", "regret", "hope", "anticipation",
    // Systems
    "gravity", "wifi", "algorithms", "physics", "mathematics",
    "economics", "psychology", "sociology", "philosophy", "logic",
    "rhetoric", "grammar", "syntax", "semantics", "pragmatics",
    "thermodynamics", "relativity", "quantum", "chemistry", "biology",
    // Funny abstracts
    "shenanigans", "tomfoolery", "kerfuffle", "hullabaloo", "brouhaha",
    "rigmarole", "bamboozlement", "quandary", "skulduggery", "hijinks",
    "malarkey", "gobbledygook", "flimflam", "balderdash", "poppycock",
    "codswallop", "hokum", "bunkum", "twaddle", "drivel",
    "humbug", "hogwash", "claptrap", "buffoonery", "absurdity",
    "foolishness", "lunacy", "madness", "pandemonium", "mayhem",
    // Science-y
    "paradox", "singularity", "hypothesis", "theorem", "axiom",
    "postulate", "corollary", "conjecture", "theory", "phenomenon",
    "anomaly", "spectrum", "equilibrium", "momentum", "inertia",
    "velocity", "acceleration", "amplitude", "frequency", "wavelength",
    "magnitude", "dimension", "continuum", "infinity", "recursion",
    // Misc abstract
    "conspiracy", "prophecy", "legacy", "mystery", "history",
    "memory", "ambiguity", "complexity", "simplicity", "symmetry",
    "asymmetry", "harmony", "discord", "synergy", "redundancy",
    "efficiency", "latency", "bandwidth", "protocol", "algorithm",
    "metaphor", "allegory", "satire", "parody", "hubris",
    // Cosmic/existential
    "oblivion", "eternity", "void", "abyss", "cosmos",
    "nebula", "vortex", "maelstrom", "cataclysm", "apocalypse",
    "armageddon", "ragnarok", "eschaton", "terminus", "genesis",
    "primordium", "transcendence", "omniscience", "omnipotence", "limbo",
    // Mystical concepts
    "omen", "curse", "blessing", "ritual", "incantation",
    "enchantment", "hex", "sigil", "rune", "talisman",
    "divination", "augury", "portent", "specter", "apparition",
    "phantasm", "wraith", "revenant", "banishment", "exorcism",
    // Dramatic abstracts
    "vengeance", "treachery", "betrayal", "redemption", "salvation",
    "damnation", "perdition", "absolution", "retribution", "reckoning",
    "penance", "atonement", "heresy", "blasphemy", "sacrilege",
    "martyrdom", "tribulation", "lamentation", "wrath", "fury",
    // Philosophical
    "nihilism", "hedonism", "stoicism", "fatalism", "determinism",
    "solipsism", "dualism", "monism", "empiricism", "rationalism",
    "skepticism", "absurdism", "utilitarianism", "relativism", "asceticism",
    // Unusual/archaic
    "pestilence", "plague", "famine", "blight", "scourge",
    "miasma", "contagion", "malaise", "torpor", "languor",
    "decadence", "decrepitude", "desolation", "ruination", "obsequy",
];

pub const GERUNDS: &[&str] = &[
    // Action verbs (physical movement and manipulation)
    "juggling", "throwing", "building", "destroying", "launching",
    "catching", "climbing", "crawling", "dancing", "diving",
    "dodging", "dragging", "dropping", "flinging", "grabbing",
    "hammering", "hurling", "jumping", "kicking", "leaping",
    "lifting", "punching", "pushing", "rolling", "running",
    "shaking", "shoving", "sliding", "spinning", "stomping",
    "swinging", "tossing", "wrestling", "yanking", "zooming",
    // Mental verbs (thinking and cognition)
    "contemplating", "questioning", "analyzing", "doubting", "pondering",
    "calculating", "considering", "daydreaming", "imagining", "memorizing",
    "philosophizing", "reasoning", "reflecting", "speculating", "theorizing",
    "overthinking", "wondering", "brooding", "deliberating", "evaluating",
    // Funny verbs (whimsical and absurd)
    "bamboozling", "discombobulating", "flabbergasting", "hornswoggling", "befuddling",
    "baffling", "bewildering", "boggling", "confounding", "dumbfounding",
    "flummoxing", "gobsmacking", "mystifying", "perplexing", "stupefying",
    "waddling", "wobbling", "yodeling", "snorkeling", "gallivanting",
    "canoodling", "lollygagging", "moseying", "skedaddling", "vamoosing",
    // Sneaky verbs (covert and secretive)
    "smuggling", "hoarding", "scheming", "plotting", "lurking",
    "conniving", "conspiring", "eavesdropping", "pilfering", "prowling",
    "snooping", "stalking", "sneaking", "tiptoeing", "whispering",
    "embezzling", "foraging", "scavenging", "sleuthing", "spying",
    // Professional verbs (formal and business-like)
    "auditing", "reviewing", "investigating", "cataloging", "documenting",
    "administrating", "alphabetizing", "annotating", "appraising", "archiving",
    "assessing", "certifying", "classifying", "compiling", "coordinating",
    "delegating", "drafting", "facilitating", "forecasting", "implementing",
    "indexing", "inspecting", "inventorying", "legislating", "notarizing",
    // Communication verbs
    "broadcasting", "negotiating", "debating", "arguing", "announcing",
    "articulating", "chattering", "declaring", "gossiping", "mumbling",
    "narrating", "proclaiming", "rambling", "ranting", "reciting",
    // Creation and organization verbs
    "assembling", "collecting", "organizing", "rearranging", "knitting",
    "crafting", "designing", "fabricating", "forging", "sculpting",
    // Consumption verbs
    "devouring", "guzzling", "munching", "nibbling", "savoring",
    "slurping", "chomping", "gobbling", "feasting", "sipping",
    // Miscellaneous
    "procrastinating", "meandering", "catapulting", "stampeding", "hibernating",
    "demanding", "perfecting", "practicing", "teleporting", "combusting",
];

pub const PRESENT_VERBS: &[&str] = &[
    // Action verbs (physical movement and manipulation)
    "juggles", "throws", "builds", "destroys", "launches",
    "catches", "climbs", "crawls", "dances", "dives",
    "dodges", "drags", "drops", "flings", "grabs",
    "hammers", "hurls", "jumps", "kicks", "leaps",
    "lifts", "punches", "pushes", "rolls", "runs",
    "shakes", "shoves", "slides", "spins", "stomps",
    "swings", "tosses", "wrestles", "yanks", "zooms",
    // Mental verbs (thinking and cognition)
    "contemplates", "questions", "analyzes", "doubts", "ponders",
    "calculates", "considers", "daydreams", "imagines", "memorizes",
    "philosophizes", "reasons", "reflects", "speculates", "theorizes",
    "overthinks", "wonders", "broods", "deliberates", "evaluates",
    // Funny verbs (whimsical and absurd)
    "bamboozles", "discombobulates", "flabbergasts", "hornswoggles", "befuddles",
    "baffles", "bewilders", "boggles", "confounds", "dumbfounds",
    "flummoxes", "gobsmacks", "mystifies", "perplexes", "stupefies",
    "waddles", "wobbles", "yodels", "snorkels", "gallivants",
    "canoodles", "lollygags", "moseys", "skedaddles", "vamooses",
    // Sneaky verbs (covert and secretive)
    "smuggles", "hoards", "schemes", "plots", "lurks",
    "connives", "conspires", "eavesdrops", "pilfers", "prowls",
    "snoops", "stalks", "sneaks", "tiptoes", "whispers",
    "embezzles", "forages", "scavenges", "sleuths", "spies",
    // Professional verbs (formal and business-like)
    "audits", "reviews", "investigates", "catalogs", "documents",
    "administrates", "alphabetizes", "annotates", "appraises", "archives",
    "assesses", "certifies", "classifies", "compiles", "coordinates",
    "delegates", "drafts", "facilitates", "forecasts", "implements",
    "indexes", "inspects", "inventories", "legislates", "notarizes",
    // Communication verbs
    "broadcasts", "negotiates", "debates", "argues", "announces",
    "articulates", "chatters", "declares", "gossips", "mumbles",
    "narrates", "proclaims", "rambles", "rants", "recites",
    // Creation and organization verbs
    "assembles", "collects", "organizes", "rearranges", "knits",
    "crafts", "designs", "fabricates", "forges", "sculpts",
    // Consumption verbs
    "devours", "guzzles", "munches", "nibbles", "savors",
    "slurps", "chomps", "gobbles", "feasts", "sips",
    // Miscellaneous
    "procrastinates", "meanders", "catapults", "stampedes", "hibernates",
    "demands", "perfects", "practices", "teleports", "combusts",
];

pub const OBJECTS: &[&str] = &[
    // Office items
    "staplers", "paperclips", "spreadsheets", "receipts", "invoices",
    "binders", "highlighters", "sticky notes", "whiteboards", "calendars",
    "folders", "envelopes", "tape dispensers", "hole punchers", "scissors",
    "calculators", "notepads", "clipboards", "lanyards", "nameplates",
    // Food
    "pickles", "waffles", "tacos", "spaghetti", "bagels",
    "pretzels", "croissants", "burritos", "dumplings", "pancakes",
    "sandwiches", "nachos", "muffins", "cupcakes", "donuts",
    "biscuits", "churros", "empanadas", "pierogis", "noodles",
    "lasagna", "ravioli", "gnocchi", "risotto", "quesadillas",
    // Household
    "socks", "spatulas", "toasters", "curtains", "pillows",
    "blankets", "towels", "hangers", "coasters", "placemats",
    "candles", "vases", "lampshades", "doorstops", "bookends",
    "umbrellas", "slippers", "bathrobes", "potholders", "trivets",
    "laundry", "dishes", "groceries", "furniture", "appliances",
    // Quirky
    "rubber ducks", "kazoos", "suspenders", "monocles", "bowties",
    "top hats", "capes", "cufflinks", "pocket watches", "magnifying glasses",
    "snow globes", "lava lamps", "bobbleheads", "stress balls", "fidget spinners",
    "whoopee cushions", "fake mustaches", "googly eyes", "pinwheels", "kaleidoscopes",
    "yard gnomes", "flamingo statues", "trophies", "participation awards", "novelty mugs",
    // Technology
    "keyboards", "cables", "dongles", "batteries", "chargers",
    "headphones", "webcams", "routers", "flash drives", "hard drives",
    "monitors", "speakers", "microphones", "adapters", "remotes",
    "tablets", "smartwatches", "earbuds", "power banks", "surge protectors",
    "modems", "printers", "scanners", "fax machines", "server racks",
    // Musical
    "accordions", "tambourines", "bagpipes", "banjos", "ukuleles",
    "harmonicas", "maracas", "triangles", "cowbells", "xylophones",
    "bongos", "tubas", "didgeridoos", "sitars", "theremins",
    // Clothing/accessories
    "sombreros", "lederhosen", "moon boots", "flip flops", "crocs",
    "fanny packs", "visors", "bandanas", "scarves", "mittens",
    "cummerbunds", "ascots", "berets", "fedoras", "tutus",
    // Random concrete objects
    "submarines", "helicopters", "trampolines", "motorcycles", "bicycles",
    "skateboards", "surfboards", "kayaks", "canoes", "parachutes",
    "telescopes", "microscopes", "binoculars", "compasses", "sundials",
    "typewriters", "gramophones", "turntables", "cassettes", "vinyl records",
    // Fantasy items
    "scepters", "chalices", "amulets", "talismans", "grimoires",
    "orbs", "crystals", "relics", "phylacteries", "rune stones",
    "enchanted mirrors", "soul gems", "wizard staffs", "dragon eggs", "phoenix feathers",
    // Weapons
    "swords", "daggers", "crossbows", "flails", "halberds",
    "maces", "scimitars", "battle axes", "war hammers", "morning stars",
    "katanas", "claymores", "rapiers", "tridents", "throwing stars",
    // Space/sci-fi
    "ray guns", "jetpacks", "hoverboards", "plasma rifles", "lightsabers",
    "blasters", "phasers", "teleporters", "cryopods", "force fields",
    "warp drives", "stasis chambers", "ion cannons", "laser swords", "pulse grenades",
    // Unusual household
    "gargoyles", "chandeliers", "taxidermy", "hourglasses", "music boxes",
    "grandfather clocks", "armoires", "chaise lounges", "fainting couches", "crystal balls",
    "candelabras", "ornate mirrors", "antique globes", "spinning wheels", "vintage typewriters",
    // Dramatic items
    "shackles", "tombstones", "guillotines", "gallows", "coffins",
    "sarcophagi", "iron maidens", "torture racks", "dungeon keys", "executioner hoods",
    "funeral urns", "mourning veils", "death masks", "bone thrones", "cursed dolls",
];

pub const ADVERBS: &[&str] = &[
    // Manner - confident/bold
    "aggressively", "boldly", "confidently", "defiantly", "fiercely",
    "triumphantly", "majestically", "pompously", "arrogantly", "smugly",
    // Manner - hesitant/cautious
    "tentatively", "reluctantly", "hesitantly", "nervously", "timidly",
    "sheepishly", "meekly", "gingerly", "cautiously", "warily",
    // Manner - emotional
    "gleefully", "furiously", "mournfully", "bitterly", "joyfully",
    "tearfully", "angrily", "happily", "sadly", "longingly",
    // Manner - mysterious/dramatic
    "ominously", "mysteriously", "dramatically", "theatrically", "cryptically",
    "hauntingly", "eerily", "menacingly", "forebodingly", "darkly",
    // Manner - chaotic/energetic
    "chaotically", "frantically", "wildly", "recklessly", "maniacally",
    "hysterically", "feverishly", "desperately", "madly", "violently",
    // Manner - calm/deliberate
    "calmly", "serenely", "peacefully", "stoically", "methodically",
    "deliberately", "patiently", "gracefully", "elegantly", "smoothly",
    // Manner - comedic/absurd
    "absurdly", "ridiculously", "comically", "hilariously", "ironically",
    "sarcastically", "mockingly", "flamboyantly", "outrageously", "preposterously",
    // Additional variety
    "suspiciously", "awkwardly", "enthusiastically", "professionally", "accidentally",
    "vigorously", "obliviously", "nonchalantly", "melodramatically", "indignantly",
    "precariously", "perpetually", "begrudgingly", "quietly", "politely",
];

pub const ARTICLES: &[&str] = &[
    "the", "a",
];

pub const PREPOSITIONS: &[&str] = &[
    // Prepositions that work well in "[verb] [object] [PREP] [article] [adj] [noun]" pattern
    // Good: "juggles pickles beside the grumpy walrus"
    // Good: "throws tacos toward a sleepy penguin"
    "with", "near", "beside", "under", "above", "behind", "beyond",
    "inside", "beneath", "toward", "against", "among", "around",
    "within", "throughout", "across", "past", "about",
    // Note: "for" removed - creates awkward constructions like "for the grumpy walrus"
    // (works with purpose: "for the mission" but not with "[adj] [noun]" pattern)
];

pub const CONNECTORS: &[&str] = &[
    "is", "was",
];

// Question starters (instead of just "why does")
pub const QUESTION_STARTERS: &[&str] = &[
    "why does", "why would", "how does", "how could",
    "when will", "where did", "what makes", "who let",
    "who authorized", "when did", "where would", "what if",
    "why must", "how dare", "who decided", "what possesses",
    "why can't", "how might", "where should", "when might",
    "who permitted", "what drives", "why won't", "how should",
    "where might", "when could", "who allows", "what compels",
    "why shouldn't", "how would", "where has", "when should",
];

// Imperative starters (instead of just "never trust")
pub const IMPERATIVES: &[&str] = &[
    "never trust", "always fear", "beware of", "never underestimate",
    "always question", "do not provoke", "respect the", "fear the",
    "avoid the", "never ignore", "always suspect", "watch out for",
    "do not underestimate", "never befriend", "always monitor",
    "stay away from", "keep distance from", "never anger",
    "bow before", "surrender to", "obey the", "question the",
    "challenge the", "embrace the", "reject the", "honor the",
    "distrust the", "never mock", "always appease", "worship the",
    "run from", "hide from", "confront the", "welcome the",
];

// Warning starters
pub const WARNINGS: &[&str] = &[
    "do not", "never", "avoid", "beware", "stop",
    "cease", "halt", "prevent", "resist", "refuse to",
    "abandon", "forget", "forsake", "reject", "ignore",
    "shun", "evade", "escape", "flee", "abstain from",
    "renounce", "discontinue", "terminate", "end", "quit",
    "forgo", "relinquish", "surrender", "eliminate", "abolish",
    "banish", "prohibit", "forbid", "disallow", "outlaw",
];

// Versus/conflict words (instead of just "versus")
pub const CONFLICTS: &[&str] = &[
    "versus", "against", "defeats", "fears", "challenges",
    "opposes", "resists", "confronts", "battles", "outsmarts",
    "humiliates", "destroys", "annihilates", "conquers", "vanquishes",
    "overwhelms", "dominates", "undermines", "sabotages", "thwarts",
    "outmaneuvers", "outwits", "confounds", "baffles", "perplexes",
    "confuses", "bewilders", "mystifies", "flummoxes", "stumps",
    "rivals", "combats", "clashes with", "feuds with", "wrestles",
];

// "Somewhere" variations
pub const LOCATIONS: &[&str] = &[
    "somewhere", "out there", "in the void", "beyond logic",
    "in accounting", "at midnight", "underground", "inevitably",
    "allegedly", "reportedly", "supposedly", "apparently",
    "in the shadows", "behind closed doors", "in the basement",
    "in the attic", "offshore", "overseas", "in the distance",
    "at headquarters", "in the warehouse", "at the border",
    "in stealth mode", "under surveillance", "off the grid",
    "in plain sight", "beneath the surface", "across dimensions",
    "in another timeline", "at the edge of reason", "in the matrix",
    "beyond the horizon", "in the ether", "among the stars",
];

// "Should not" variations
pub const PROHIBITIONS: &[&str] = &[
    "should not", "must not", "cannot", "dare not",
    "will never", "refuse to", "failed to", "forgot to",
    "refuses to", "struggles to", "hesitates to", "declines to",
    "opts not to", "chooses not to", "prefers not to", "avoids",
    "dreads to", "fears to", "loathes to", "hates to",
    "never learned to", "couldn't possibly", "wouldn't dare",
    "shall not", "ought not", "better not", "had better not",
    "may not", "might not", "will not", "would not",
    "never manages to", "always fails to", "perpetually avoids",
];

// "Requires more" variations
pub const DEMANDS: &[&str] = &[
    "requires more", "demands extra", "needs additional", "craves endless",
    "hoards excessive", "stockpiles", "accumulates", "consumes vast",
    "desires infinite", "pursues relentless", "hungers for",
    "thirsts for", "yearns for", "longs for", "aches for",
    "obsesses over", "fixates on", "covets", "envies",
    "requires excessive", "demands infinite", "needs constant",
    "craves more", "hoards all", "stockpiles endless",
    "accumulates vast", "consumes endless", "desires more",
    "pursues endless", "seeks unlimited", "chases eternal",
    "gathers infinite", "collects excessive", "amasses tremendous",
];

// Time markers for "since" template
pub const TIME_MARKERS: &[&str] = &[
    "since forever", "since breakfast", "since the incident",
    "until dawn", "before coffee", "after midnight", "during chaos",
    "throughout history", "beyond reason", "without warning", "against protocol",
    "since the beginning", "until the end", "before sunrise", "after dusk",
    "during the meeting", "throughout eternity", "beyond comprehension",
    "without explanation", "against all odds", "since childhood",
    "until retirement", "before the merger", "after the crash",
    "during the eclipse", "throughout the crisis", "beyond repair",
    "without hesitation", "against advice", "since the awakening",
    "until further notice", "before the deadline", "after the revolution",
    "during negotiations", "throughout the ordeal", "beyond expectations",
];

// Numbers for fragment template
pub const NUMBERS: &[&str] = &[
    "three", "seven", "twelve", "forty-two", "exactly nine",
    "countless", "several", "many", "too many", "precisely six",
    "almost eight", "roughly eleven", "approximately four",
    "exactly thirteen", "nearly twenty", "about fifty", "over ninety",
    "under five", "merely two", "only one", "at least ten",
    "no more than fifteen", "fewer than thirty", "upwards of forty",
    "a dozen", "a hundred", "a thousand", "infinite",
    "zero", "negative five", "pi", "an undisclosed number of",
    "an alarming number of", "a suspicious amount of",
    "an unprecedented number of", "a record-breaking",
];

// Detection/status words for fragment endings
pub const STATUS_WORDS: &[&str] = &[
    "detected", "spotted", "confirmed", "reported", "identified",
    "approaching", "escaping", "plotting", "scheming", "emerging",
    "suspected", "anticipated", "expected", "predicted", "forecast",
    "observed", "witnessed", "documented", "recorded", "logged",
    "advancing", "retreating", "mobilizing", "organizing", "preparing",
    "materializing", "manifesting", "appearing", "surfacing", "arising",
    "congregating", "assembling", "gathering", "forming", "coalescing",
    "infiltrating", "invading", "penetrating", "encroaching", "spreading",
];

// Connector words (replaces fixed "is")
pub const CONNECTORS_ACTIVE: &[&str] = &[
    "is", "was", "keeps", "seems", "appears",
    "remains", "continues", "stays", "began",
    "has been", "will be", "might be", "could be",
    "should be", "must be", "would be", "may be",
    "started", "stopped", "finished", "resumed",
    "persists in", "insists on", "excels at", "fails at",
    "struggles with", "thrives on", "survives by",
    "specializes in", "dabbles in", "experiments with",
    "contemplates", "considers", "ponders", "deliberates",
];

// Legacy compatibility aliases
pub const YEARS: &[&str] = &[
    "1987", "1994", "2003", "2012", "1776",
];

pub const COMPARATIVES: &[&str] = &[
    "versus", "against", "defeats", "fears", "challenges",
    "opposes", "resists", "confronts", "battles", "outsmarts",
];

pub const BASE_VERBS: &[&str] = &[
    // Action verbs (physical movement and manipulation)
    "juggle", "throw", "build", "destroy", "launch",
    "catch", "climb", "crawl", "dance", "dive",
    "dodge", "drag", "drop", "fling", "grab",
    "hammer", "hurl", "jump", "kick", "leap",
    "lift", "punch", "push", "roll", "run",
    "shake", "shove", "slide", "spin", "stomp",
    "swing", "toss", "wrestle", "yank", "zoom",
    // Mental verbs (thinking and cognition)
    "contemplate", "question", "analyze", "doubt", "ponder",
    "calculate", "consider", "daydream", "imagine", "memorize",
    "philosophize", "reason", "reflect", "speculate", "theorize",
    "overthink", "wonder", "brood", "deliberate", "evaluate",
    // Funny verbs (whimsical and absurd)
    "bamboozle", "discombobulate", "flabbergast", "hornswoggle", "befuddle",
    "baffle", "bewilder", "boggle", "confound", "dumbfound",
    "flummox", "gobsmack", "mystify", "perplex", "stupefy",
    "waddle", "wobble", "yodel", "snorkel", "gallivant",
    "canoodle", "lollygag", "mosey", "skedaddle", "vamoose",
    // Sneaky verbs (covert and secretive)
    "smuggle", "hoard", "scheme", "plot", "lurk",
    "connive", "conspire", "eavesdrop", "pilfer", "prowl",
    "snoop", "stalk", "sneak", "tiptoe", "whisper",
    "embezzle", "forage", "scavenge", "sleuth", "spy",
    // Professional verbs (formal and business-like)
    "audit", "review", "investigate", "catalog", "document",
    "administrate", "alphabetize", "annotate", "appraise", "archive",
    "assess", "certify", "classify", "compile", "coordinate",
    "delegate", "draft", "facilitate", "forecast", "implement",
    "index", "inspect", "inventory", "legislate", "notarize",
    // Communication verbs
    "broadcast", "negotiate", "debate", "argue", "announce",
    "articulate", "chatter", "declare", "gossip", "mumble",
    "narrate", "proclaim", "ramble", "rant", "recite",
    // Creation and organization verbs
    "assemble", "collect", "organize", "rearrange", "knit",
    "craft", "design", "fabricate", "forge", "sculpt",
    // Consumption verbs
    "devour", "guzzle", "munch", "nibble", "savor",
    "slurp", "chomp", "gobble", "feast", "sip",
    // Miscellaneous
    "procrastinate", "meander", "catapult", "stampede", "hibernate",
    "demand", "perfect", "practice", "teleport", "combust",
];

/// Intransitive verbs that require prepositions before objects.
/// Maps verb (base/present/gerund forms) to their required preposition.
/// When these verbs are used, a preposition must be inserted before the object.
pub const INTRANSITIVE_VERB_PREPS: &[(&str, &str)] = &[
    // WITH - companionship/instrument
    ("dance", "with"), ("dances", "with"), ("dancing", "with"),
    ("gallivant", "with"), ("gallivants", "with"), ("gallivanting", "with"),
    ("canoodle", "with"), ("canoodles", "with"), ("canoodling", "with"),
    ("connive", "with"), ("connives", "with"), ("conniving", "with"),
    ("conspire", "against"), ("conspires", "against"), ("conspiring", "against"),

    // ABOUT - subject matter
    ("daydream", "about"), ("daydreams", "about"), ("daydreaming", "about"),
    ("philosophize", "about"), ("philosophizes", "about"), ("philosophizing", "about"),
    ("speculate", "about"), ("speculates", "about"), ("speculating", "about"),
    ("theorize", "about"), ("theorizes", "about"), ("theorizing", "about"),
    ("wonder", "about"), ("wonders", "about"), ("wondering", "about"),
    ("brood", "over"), ("broods", "over"), ("brooding", "over"),
    ("deliberate", "on"), ("deliberates", "on"), ("deliberating", "on"),
    ("yodel", "about"), ("yodels", "about"), ("yodeling", "about"),
    ("scheme", "about"), ("schemes", "about"), ("scheming", "about"),
    ("argue", "about"), ("argues", "about"), ("arguing", "about"),
    ("chatter", "about"), ("chatters", "about"), ("chattering", "about"),
    ("gossip", "about"), ("gossips", "about"), ("gossiping", "about"),
    ("ramble", "about"), ("rambles", "about"), ("rambling", "about"),
    ("rant", "about"), ("rants", "about"), ("ranting", "about"),
    ("procrastinate", "on"), ("procrastinates", "on"), ("procrastinating", "on"),

    // ON - surface/target
    ("climb", "on"), ("climbs", "on"), ("climbing", "on"),
    ("stomp", "on"), ("stomps", "on"), ("stomping", "on"),
    ("reflect", "on"), ("reflects", "on"), ("reflecting", "on"),
    ("eavesdrop", "on"), ("eavesdrops", "on"), ("eavesdropping", "on"),
    ("snoop", "on"), ("snoops", "on"), ("snooping", "on"),
    ("spy", "on"), ("spies", "on"), ("spying", "on"),
    ("feast", "on"), ("feasts", "on"), ("feasting", "on"),

    // MOVEMENT - over/past/through
    ("crawl", "over"), ("crawls", "over"), ("crawling", "over"),
    ("jump", "over"), ("jumps", "over"), ("jumping", "over"),
    ("leap", "over"), ("leaps", "over"), ("leaping", "over"),
    ("zoom", "past"), ("zooms", "past"), ("zooming", "past"),
    ("waddle", "toward"), ("waddles", "toward"), ("waddling", "toward"),
    ("wobble", "toward"), ("wobbles", "toward"), ("wobbling", "toward"),
    ("mosey", "toward"), ("moseys", "toward"), ("moseying", "toward"),
    ("meander", "through"), ("meanders", "through"), ("meandering", "through"),
    ("stampede", "through"), ("stampedes", "through"), ("stampeding", "through"),

    // FROM - away from
    ("run", "from"), ("runs", "from"), ("running", "from"),
    ("skedaddle", "from"), ("skedaddles", "from"), ("skedaddling", "from"),
    ("vamoose", "from"), ("vamooses", "from"), ("vamoosing", "from"),

    // INTO/TOWARD - direction
    ("dive", "into"), ("dives", "into"), ("diving", "into"),

    // AROUND/NEAR - proximity
    ("lurk", "near"), ("lurks", "near"), ("lurking", "near"),
    ("prowl", "around"), ("prowls", "around"), ("prowling", "around"),
    ("lollygag", "around"), ("lollygags", "around"), ("lollygagging", "around"),
    ("snorkel", "near"), ("snorkels", "near"), ("snorkeling", "near"),
    ("hibernate", "through"), ("hibernates", "through"), ("hibernating", "through"),
    ("tiptoe", "around"), ("tiptoes", "around"), ("tiptoeing", "around"),
    ("sneak", "past"), ("sneaks", "past"), ("sneaking", "past"),

    // FOR - seeking
    ("forage", "for"), ("forages", "for"), ("foraging", "for"),
    ("scavenge", "for"), ("scavenges", "for"), ("scavenging", "for"),
    ("sleuth", "for"), ("sleuths", "for"), ("sleuthing", "for"),

    // AGAINST - opposition
    ("plot", "against"), ("plots", "against"), ("plotting", "against"),
];

/// Get the required preposition for an intransitive verb, if any.
/// Returns None for transitive verbs that can take direct objects.
pub fn get_verb_preposition(verb: &str) -> Option<&'static str> {
    INTRANSITIVE_VERB_PREPS
        .iter()
        .find(|(v, _)| *v == verb)
        .map(|(_, prep)| *prep)
}

// =============================================================================
// NSFW Word Lists (Cards Against Humanity style)
// =============================================================================

pub const ADJECTIVES_NSFW: &[&str] = &[
    "aroused", "post-coital", "freshly-divorced", "inappropriately-moist",
    "emotionally-unavailable", "heavily-medicated", "regrettably-naked",
    "visibly-aroused", "suspiciously-sticky", "chronically-single",
    "lactating", "overcompensating", "sexually-confused",
    "tax-evading", "recently-incarcerated", "questionably-legal",
    "perpetually-horny", "aggressively-single", "notoriously-kinky",
    "uncomfortably-erect",
];

pub const ADVERBS_NSFW: &[&str] = &[
    "seductively", "suggestively", "erotically", "sensually",
    "provocatively", "lewdly", "inappropriately", "nakedly",
    "drunkenly", "shamelessly", "guiltily", "flirtatiously",
    "intimately", "voyeuristically", "orgasmically", "promiscuously",
    "lustfully", "raunchily", "salaciously", "lasciviously",
];

pub const PROFESSIONS_NSFW: &[&str] = &[
    "masseuse", "escort", "dominatrix", "gigolo", "poolboy",
    "proctologist", "fluffer", "divorce-lawyer", "telemarketer",
    "debt-collector", "yoga-instructor", "personal-trainer",
    "stripper", "cam-model", "sugar-daddy",
];

pub const NOUNS_NSFW: &[&str] = &[
    "puberty", "midlife-crisis", "alimony", "libido", "impotence",
    "incontinence", "flatulence", "walk-of-shame", "performance-anxiety",
    "innuendo", "climax", "safe-word", "pillow-talk",
    "one-night-stand", "booty-call",
];

pub const OBJECTS_NSFW: &[&str] = &[
    "handcuffs", "whips", "blindfolds", "riding-crops", "leather-pants",
    "enema-bags", "hemorrhoid-cream", "adult-diapers", "pregnancy-tests",
    "marital-aids", "back-massagers", "cucumbers", "bananas", "melons",
    "edible-underwear", "body-oils", "flavored-lubricants", "massage-candles",
    "feather-dusters", "silk-scarves",
];

pub const BASE_VERBS_NSFW: &[&str] = &[
    "seduce", "fondle", "grope", "arouse",
    "undress", "spank", "caress", "mount",
    "twerk", "probe", "titillate", "straddle",
    "ravish", "tantalize", "motorboat",
];

pub const PRESENT_VERBS_NSFW: &[&str] = &[
    "seduces", "fondles", "gropes", "arouses",
    "undresses", "spanks", "caresses", "mounts",
    "twerks", "probes", "titillates", "straddles",
    "ravishes", "tantalizes", "motorboats",
];

pub const GERUNDS_NSFW: &[&str] = &[
    "seducing", "fondling", "groping", "arousing",
    "undressing", "spanking", "caressing", "mounting",
    "twerking", "probing", "titillating", "straddling",
    "ravishing", "tantalizing", "motorboating",
];

pub const IMPERATIVES_NSFW: &[&str] = &[
    "never sleep with", "stop touching", "quit fondling",
    "seduce the", "do not arouse", "never expose",
    "stop straddling", "quit groping", "never motorboat",
    "avoid seducing",
];

pub const PROHIBITIONS_NSFW: &[&str] = &[
    "refuses to climax", "cannot finish", "struggles to perform",
    "failed to satisfy", "perpetually edges", "never finishes",
    "prematurely concludes", "awkwardly fumbles", "embarrassingly fails to",
    "chronically disappoints",
];

pub const DEMANDS_NSFW: &[&str] = &[
    "craves intense", "demands rougher", "requires deeper",
    "desires longer", "hungers for moist", "thirsts for thick",
    "needs harder", "wants bigger", "yearns for wetter",
    "aches for firmer",
];

// =============================================================================
// Word Getter Functions (for NSFW support)
// =============================================================================

/// Get adjectives, optionally including NSFW words.
pub fn get_adjectives(nsfw: bool) -> Vec<&'static str> {
    let mut words = ADJECTIVES.to_vec();
    if nsfw {
        words.extend(ADJECTIVES_NSFW);
    }
    words
}

/// Get adverbs, optionally including NSFW words.
pub fn get_adverbs(nsfw: bool) -> Vec<&'static str> {
    let mut words = ADVERBS.to_vec();
    if nsfw {
        words.extend(ADVERBS_NSFW);
    }
    words
}

/// Get professions, optionally including NSFW words.
pub fn get_professions(nsfw: bool) -> Vec<&'static str> {
    let mut words = PROFESSIONS.to_vec();
    if nsfw {
        words.extend(PROFESSIONS_NSFW);
    }
    words
}

/// Get nouns, optionally including NSFW words.
pub fn get_nouns(nsfw: bool) -> Vec<&'static str> {
    let mut words = NOUNS.to_vec();
    if nsfw {
        words.extend(NOUNS_NSFW);
    }
    words
}

/// Get objects, optionally including NSFW words.
pub fn get_objects(nsfw: bool) -> Vec<&'static str> {
    let mut words = OBJECTS.to_vec();
    if nsfw {
        words.extend(OBJECTS_NSFW);
    }
    words
}

/// Get base verbs, optionally including NSFW words.
pub fn get_base_verbs(nsfw: bool) -> Vec<&'static str> {
    let mut words = BASE_VERBS.to_vec();
    if nsfw {
        words.extend(BASE_VERBS_NSFW);
    }
    words
}

/// Get present tense verbs, optionally including NSFW words.
pub fn get_present_verbs(nsfw: bool) -> Vec<&'static str> {
    let mut words = PRESENT_VERBS.to_vec();
    if nsfw {
        words.extend(PRESENT_VERBS_NSFW);
    }
    words
}

/// Get gerunds, optionally including NSFW words.
pub fn get_gerunds(nsfw: bool) -> Vec<&'static str> {
    let mut words = GERUNDS.to_vec();
    if nsfw {
        words.extend(GERUNDS_NSFW);
    }
    words
}

/// Get imperatives, optionally including NSFW words.
pub fn get_imperatives(nsfw: bool) -> Vec<&'static str> {
    let mut words = IMPERATIVES.to_vec();
    if nsfw {
        words.extend(IMPERATIVES_NSFW);
    }
    words
}

/// Get prohibitions, optionally including NSFW words.
pub fn get_prohibitions(nsfw: bool) -> Vec<&'static str> {
    let mut words = PROHIBITIONS.to_vec();
    if nsfw {
        words.extend(PROHIBITIONS_NSFW);
    }
    words
}

/// Get demands, optionally including NSFW words.
pub fn get_demands(nsfw: bool) -> Vec<&'static str> {
    let mut words = DEMANDS.to_vec();
    if nsfw {
        words.extend(DEMANDS_NSFW);
    }
    words
}
