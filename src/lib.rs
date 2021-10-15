use rand::prelude::SliceRandom;
use rand::thread_rng;

mod adjectives;
mod cats;

use crate::adjectives::ADJECTIVES;
use crate::cats::CATS;

/// Return a randomly-selected cat name.
pub fn name() -> String {
    let mut rng = thread_rng();
    let adjective = ADJECTIVES.choose(&mut rng).unwrap_or(&"sainted");
    let cat = CATS.choose(&mut rng).unwrap_or(&"bishonen");
    format!("{} {}", adjective, cat)
}

#[cfg(test)]
mod tests {

    #[test]
    fn random_name() {
        let catname = super::name();
        assert!(!catname.is_empty());
        assert!(catname.contains(' '));
    }
}
