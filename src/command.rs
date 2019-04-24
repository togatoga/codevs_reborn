
#[derive(Debug, Copy, Clone, Eq)]
pub enum Command {
    Drop((usize, usize)),
    Spell,
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        match (self, other) {
            (&Command::Drop(ref a), &Command::Drop(ref b)) => a == b,
            (&Command::Spell, &Command::Spell) => true,
            _ => false
        }
    }
}