use rand::prelude::*;
use rand::rng;

pub fn get_random_name() -> String {
    let names = [
        // Popular names
        "Alice", "Bob", "Charlie", "Diana", "Edward", "Fiona", "George", "Hannah",
        "Isaac", "Julia", "Kevin", "Luna", "Michael", "Nina", "Oliver", "Penny",
        "Quinn", "Rachel", "Samuel", "Tina", "Uma", "Victor", "Wendy", "Xavier",
        "Yara", "Zoe",

        // Classic names
        "Alexander", "Catherine", "Benjamin", "Elizabeth", "Christopher", "Margaret",
        "Daniel", "Jennifer", "Frederick", "Patricia", "Gregory", "Rebecca",
        "Jonathan", "Stephanie", "Matthew", "Victoria",

        // International names
        "Akira", "Lucia", "Hassan", "Ingrid", "Diego", "Priya", "Erik", "Sakura",
        "Marco", "Aaliyah", "Nikolai", "Esperanza", "Olaf", "Fatima", "Pierre",
        "Svetlana", "Tariq", "Yuki", "Amadou", "Zara",

        // Modern names
        "Aiden", "Madison", "Logan", "Ava", "Mason", "Sophia", "Ethan", "Emma",
        "Jackson", "Olivia", "Lucas", "Isabella", "Liam", "Mia", "Noah", "Harper",
        "Sebastian", "Evelyn", "Caleb", "Abigail",
    ];

    let mut rng = rng();
    names.choose(&mut rng).unwrap().to_string()
}
pub fn generate_random_text() -> String {
    // Enhanced components for sentence generation
    let subjects = [
        "I",
        "You",
        "We",
        "They",
        "He",
        "She",
        "It",
        "The cat",
        "The dog",
        "A bird",
        "My friend",
        "The teacher",
        "A stranger",
        "The child",
        "Everyone",
        "Someone",
        "Nobody",
        "The scientist",
        "The artist",
        "My neighbor",
    ];

    let verbs_present = [
        "walk",
        "run",
        "jump",
        "dance",
        "sing",
        "laugh",
        "cry",
        "think",
        "believe",
        "understand",
        "create",
        "discover",
        "explore",
        "imagine",
        "build",
        "write",
        "read",
        "paint",
        "cook",
        "travel",
    ];

    let verbs_past = [
        "walked",
        "ran",
        "jumped",
        "danced",
        "sang",
        "laughed",
        "cried",
        "thought",
        "believed",
        "understood",
        "created",
        "discovered",
        "explored",
        "imagined",
        "built",
        "wrote",
        "read",
        "painted",
        "cooked",
        "traveled",
    ];

    let being_verbs = [
        ("I", "am"),
        ("You", "are"),
        ("We", "are"),
        ("They", "are"),
        ("He", "is"),
        ("She", "is"),
        ("It", "is"),
        ("The cat", "is"),
        ("The dog", "is"),
        ("A bird", "is"),
        ("My friend", "is"),
        ("The teacher", "is"),
        ("A stranger", "is"),
        ("The child", "is"),
        ("Everyone", "is"),
        ("Someone", "is"),
        ("Nobody", "is"),
        ("The scientist", "is"),
        ("The artist", "is"),
        ("My neighbor", "is"),
    ];

    let objects = [
        "a book",
        "the sunset",
        "a rainbow",
        "music",
        "art",
        "technology",
        "nature",
        "the ocean",
        "mountains",
        "flowers",
        "stars",
        "dreams",
        "adventures",
        "mysteries",
        "stories",
        "games",
        "puzzles",
        "treasures",
        "ancient ruins",
        "modern buildings",
        "wild animals",
        "exotic foods",
    ];

    let locations = [
        "in the park",
        "at home",
        "near the lake",
        "in the city",
        "on the mountain",
        "by the ocean",
        "in the forest",
        "under the stars",
        "in a distant land",
        "around the corner",
        "across the street",
        "in the library",
        "at the museum",
        "in the garden",
        "on the beach",
        "in the countryside",
        "downtown",
        "in the wilderness",
    ];

    let adjectives = [
        "beautiful",
        "mysterious",
        "incredible",
        "peaceful",
        "exciting",
        "colorful",
        "ancient",
        "modern",
        "tiny",
        "enormous",
        "brilliant",
        "fascinating",
        "magical",
        "ordinary",
        "extraordinary",
        "delicate",
        "powerful",
        "gentle",
        "fierce",
        "graceful",
        "elegant",
        "rustic",
    ];

    let adverbs = [
        "quietly",
        "loudly",
        "carefully",
        "quickly",
        "slowly",
        "gracefully",
        "mysteriously",
        "suddenly",
        "eventually",
        "frequently",
        "rarely",
        "peacefully",
        "energetically",
        "thoughtfully",
        "creatively",
    ];

    let time_phrases = [
        "yesterday",
        "today",
        "tomorrow",
        "last week",
        "every morning",
        "sometimes",
        "often",
        "rarely",
        "always",
        "never",
        "once upon a time",
        "in the future",
        "long ago",
        "recently",
        "right now",
    ];

    let mut rng = rng();

    // Generate different sentence structures
    match rng.random_range(0..8) {
        0 => {
            // Simple present: Subject + verb + object + location
            let subject = subjects.choose(&mut rng).unwrap();
            let verb = verbs_present.choose(&mut rng).unwrap();
            let object = objects.choose(&mut rng).unwrap();
            let location = locations.choose(&mut rng).unwrap();
            format!("{} {} {} {}.", subject, verb, object, location)
        }
        1 => {
            // Past tense: Time + subject + verb + adverb
            let time = time_phrases.choose(&mut rng).unwrap();
            let subject = subjects.choose(&mut rng).unwrap().to_lowercase();
            let verb = verbs_past.choose(&mut rng).unwrap();
            let adverb = adverbs.choose(&mut rng).unwrap();
            format!(
                "{}, {} {} {}.",
                capitalize_first(time),
                subject,
                verb,
                adverb
            )
        }
        2 => {
            // Being verb: Subject + being verb + adjective
            let (subject, being_verb) = being_verbs.choose(&mut rng).unwrap();
            let adjective = adjectives.choose(&mut rng).unwrap();
            let location = locations.choose(&mut rng).unwrap();
            format!("{} {} {} {}.", subject, being_verb, adjective, location)
        }
        3 => {
            // Descriptive: There + being verb + adjective + object + location
            let adjective = adjectives.choose(&mut rng).unwrap();
            let object = objects.choose(&mut rng).unwrap();
            let location = locations.choose(&mut rng).unwrap();
            format!("There is {} {} {}.", adjective, object, location)
        }
        4 => {
            // Exclamatory: Look at + adjective + object!
            let adjective = adjectives.choose(&mut rng).unwrap();
            let object = objects.choose(&mut rng).unwrap();
            match rng.gen_range(0..3) {
                0 => format!("Look at that {} {}!", adjective, object),
                1 => format!("What {} {} that is!", adjective, object),
                _ => format!("How {} {} looks!", adjective, object),
            }
        }
        5 => {
            // Complex: Subject + adverb + verb + adjective + object
            let subject = subjects.choose(&mut rng).unwrap();
            let adverb = adverbs.choose(&mut rng).unwrap();
            let verb = verbs_present.choose(&mut rng).unwrap();
            let adjective = adjectives.choose(&mut rng).unwrap();
            let object = objects.choose(&mut rng).unwrap();
            format!("{} {} {} {} {}.", subject, adverb, verb, adjective, object)
        }
        6 => {
            // Question format
            let subject = subjects.choose(&mut rng).unwrap().to_lowercase();
            let verb = verbs_present.choose(&mut rng).unwrap();
            let object = objects.choose(&mut rng).unwrap();
            let location = locations.choose(&mut rng).unwrap();
            match rng.gen_range(0..3) {
                0 => format!("Did {} {} {} {}?", subject, verb, object, location),
                1 => format!("Can {} {} {} {}?", subject, verb, object, location),
                _ => format!("Will {} {} {} {}?", subject, verb, object, location),
            }
        }
        _ => {
            // Multi-sentence paragraph
            let sentence1 =
                generate_simple_sentence(&mut rng, &subjects, &verbs_present, &objects, &locations);
            let sentence2 =
                generate_descriptive_sentence(&mut rng, &adjectives, &objects, &locations);
            let sentence3 = generate_action_sentence(&mut rng, &subjects, &verbs_past, &adverbs);
            format!("{}. {}. {}.", sentence1, sentence2, sentence3)
        }
    }
}

fn generate_simple_sentence(
    rng: &mut ThreadRng,
    subjects: &[&str],
    verbs: &[&str],
    objects: &[&str],
    locations: &[&str],
) -> String {
    let subject = subjects.choose(rng).unwrap();
    let verb = verbs.choose(rng).unwrap();
    let object = objects.choose(rng).unwrap();
    let location = locations.choose(rng).unwrap();
    format!("{} {} {} {}", subject, verb, object, location)
}

fn generate_descriptive_sentence(
    rng: &mut ThreadRng,
    adjectives: &[&str],
    objects: &[&str],
    locations: &[&str],
) -> String {
    let adjective = adjectives.choose(rng).unwrap();
    let object = objects.choose(rng).unwrap();
    let location = locations.choose(rng).unwrap();
    format!("The {} {} appears {}", adjective, object, location)
}

fn generate_action_sentence(
    rng: &mut ThreadRng,
    subjects: &[&str],
    verbs: &[&str],
    adverbs: &[&str],
) -> String {
    let subject = subjects.choose(rng).unwrap();
    let verb = verbs.choose(rng).unwrap();
    let adverb = adverbs.choose(rng).unwrap();
    format!("{} {} {}", subject, verb, adverb)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// Helper function to generate multiple random sentences
pub fn generate_random_paragraph(sentence_count: usize) -> String {
    (0..sentence_count)
        .map(|_| generate_random_text())
        .collect::<Vec<String>>()
        .join(" ")
}

// Generate random text with specific theme
pub fn generate_themed_text(theme: &str) -> String {
    let mut rng = rng();

    match theme.to_lowercase().as_str() {
        "nature" => {
            let nature_subjects = ["The wind", "A flower", "The tree", "Birds", "The river"];
            let nature_verbs = ["flows", "blooms", "whispers", "dances", "grows"];
            let nature_objects = [
                "through the valley",
                "in the meadow",
                "under moonlight",
                "with the breeze",
                "among the leaves",
            ];

            let subject = nature_subjects.choose(&mut rng).unwrap();
            let verb = nature_verbs.choose(&mut rng).unwrap();
            let object = nature_objects.choose(&mut rng).unwrap();
            format!("{} {} {}.", subject, verb, object)
        }
        "technology" => {
            let tech_subjects = [
                "The computer",
                "Artificial intelligence",
                "Robots",
                "The internet",
                "Smart devices",
            ];
            let tech_verbs = ["processes", "learns", "connects", "analyzes", "transforms"];
            let tech_objects = [
                "vast amounts of data",
                "human behavior",
                "our daily lives",
                "complex problems",
                "the future",
            ];

            let subject = tech_subjects.choose(&mut rng).unwrap();
            let verb = tech_verbs.choose(&mut rng).unwrap();
            let object = tech_objects.choose(&mut rng).unwrap();
            format!("{} {} {}.", subject, verb, object)
        }
        _ => generate_random_text(), // Default to regular random text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_proper_sentences() {
        for _ in 0..100 {
            let text = generate_random_text();
            assert!(!text.is_empty());
            // Check that sentence ends with proper punctuation
            assert!(text.ends_with('.') || text.ends_with('!') || text.ends_with('?'));
            // Check that sentence starts with capital letter
            assert!(text.chars().next().unwrap().is_uppercase());
        }
    }

    #[test]
    fn test_paragraph_generation() {
        let paragraph = generate_random_paragraph(3);
        let sentence_count = paragraph.matches('.').count()
            + paragraph.matches('!').count()
            + paragraph.matches('?').count();
        assert!(sentence_count >= 3);
    }

    #[test]
    fn test_themed_generation() {
        let nature_text = generate_themed_text("nature");
        assert!(!nature_text.is_empty());

        let tech_text = generate_themed_text("technology");
        assert!(!tech_text.is_empty());
    }
}
