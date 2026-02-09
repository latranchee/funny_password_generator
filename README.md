# Funny Password Generator

**Strong passwords you'll actually remember.** Instead of `x7$kQ!9m`, get `The paranoid walrus juggles rubber ducks`.

This is a command-line tool that generates passwords as funny, grammatically correct sentences. It also generates ridiculous character names. Built in Rust, it's fast, secure, and endlessly entertaining.

---

## Why This Exists

Traditional password generators create strings like `aG#8xLp!2q` -- secure, sure, but impossible to remember. You end up writing them on sticky notes, which defeats the purpose.

This generator takes a different approach: **it builds tiny stories.** Each password is a grammatically correct sentence made of random words picked from huge curated lists. The result is a password that:

- Is genuinely strong (the tool calculates actual entropy to prove it)
- Sticks in your head because it paints a vivid, absurd picture
- Is fun to type instead of being a chore

## Great for AI Agents *and* Humans

**For AI agents:** When your automation, bot, or AI agent needs to generate strong credentials on the fly, this tool produces high-entropy passwords in a single command -- no configuration, no dependencies, just pipe-friendly output ready to use.

**For humans:** You'll actually *remember* these. Try forgetting "The suspicious octopus hoards staplers" -- you can't. Your brain loves stories, not random characters.

---

## What It Generates

### Passwords (`mkpass`)

The generator picks from **900+ words** across categories like animals, professions, adjectives, verbs, and objects, then assembles them into sentences using 10 different templates.

Here are real examples straight from the tool:

```
Beware! The caffeinated narwhal overthinks rubber ducks!
Never underestimate a sentient accountant.
Why would the wobbly capybara smuggle pickles?
Somewhere a paranoid wizard is plotting...
Llamas should not negotiate.
Do not provoke the existential penguin!
Desperately hoarding chaos since 1987.
Penguin versus bureaucracy.
```

Each run gives you several passwords to pick from, along with how strong they are.

### Character Names (`mkname`)

Need a username, a game character, or a project codename? The name generator has 6 styles:

| Style | Example |
|-------|---------|
| Classic | `the grumpy walrus` |
| Title | `narwhal the magnificent` |
| Epithet | `vengeful pirate of chaos` |
| Action | `the scheming dragon slayer` |
| Versus | `penguin vs entropy` |
| Bearer | `keeper of the rubber ducks` |

---

## How It Works (No CS Degree Required)

Think of it like magnetic poetry on a fridge. The tool has buckets of words:

- **Animals:** axolotl, capybara, narwhal, platypus, pangolin...
- **Adjectives:** suspicious, wobbly, caffeinated, sentient, paranoid...
- **Jobs:** accountant, wizard, astronaut, pirate, bureaucrat...
- **Verbs:** juggles, hoards, schemes, overthinks, smuggles...
- **Objects:** pickles, staplers, rubber ducks, lightsabers, tacos...
- ...and more (adverbs, nouns, prepositions)

It picks words randomly from these buckets, but follows **grammar rules** so the result always reads like a real (if absurd) sentence. It knows that "the" comes before an adjective, that a verb needs a subject, and that "a" becomes "an" before a vowel.

It even generates 10x more candidates than you asked for and picks the ones with the **highest randomness** (entropy), so you always get the strongest options.

---

## Getting Started

### Build it

You need [Rust](https://rustup.rs/) installed, then:

```bash
cargo build --release
```

### Generate passwords

```bash
# 3 passwords (default)
mkpass

# 5 passwords with 7+ words each
mkpass 7 -n 5

# Show entropy (strength) information
mkpass -e

# Family-friendly mode (no spicy words)
mkpass --sfw

# CamelCase format
mkpass --camel

# Use a custom separator
mkpass -s "_"
```

### Generate names

```bash
# One random name
mkname

# 5 names in "bearer" style
mkname 5 -s bearer

# Snake_case output for usernames
mkname --snake

# See all available styles
mkname --styles
```

---

## Is It Actually Secure?

Yes. Use the `-e` flag and the tool will show you the math:

```
Entropy (pattern attack): 52.4 bits
Entropy (brute force):    78.1 bits
```

A 5-word funny password has comparable entropy to a random 10-character alphanumeric password, but you can actually remember it. Bump it to 7+ words and you're in very strong territory.

The tool assumes the attacker **knows your word lists** and still calculates security based on that worst case. In practice, an attacker who doesn't know you're using this tool faces even more combinations.

---

## Options at a Glance

| Flag | What it does |
|------|-------------|
| `[number]` | Minimum words per password (default: 5) |
| `-n [count]` | How many passwords to generate (default: 3) |
| `-s [char]` | Word separator (default: space) |
| `-t [name]` | Force a specific template style |
| `-e` | Show entropy / strength details |
| `--sfw` | Keep it family-friendly |
| `--camel` | CamelCase output |

---

## NSFW Mode

NSFW words are **on by default** because this is a password generator, not a children's book. Use `--sfw` if you need to keep things clean. The NSFW lists add extra words to the pool, which also increases password strength.

---

*Built with Rust. Powered by capybaras, chaos, and questionable life choices.*
