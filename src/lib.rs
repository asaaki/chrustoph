/// The one and only Chrustoph
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chrustoph;

impl Chrustoph {
    /// Create a new instance of Chrustoph … just kidding, there is only one!
    pub fn new() -> Self {
        Self
    }

    /// Get the name of Chrustoph
    pub fn name(&self) -> &str {
        "Chrustoph"
    }

    /// Get the rusty name of Chrustoph … nice!
    pub fn rusty_name(&self) -> &str {
        "ChRustoph"
    }

    /// Get the real name of Chrustoph … lame!
    pub fn real_name(&self) -> &str {
        "Christoph"
    }

    /// Get the nick name of Chrustoph … cool!
    pub fn nick_name(&self) -> &str {
        "asaaki"
    }
}

#[cfg(test)]
mod tests;
