use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    pub fn parse(name: String) -> SubscriberName {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_to_long = name.graphemes(true).count() > 256;
        let forbidden_charaters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name.chars().any(|g| forbidden_charaters.contains(&g));

        if is_empty_or_whitespace || is_to_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", name)
        } else {
            Self(name)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
