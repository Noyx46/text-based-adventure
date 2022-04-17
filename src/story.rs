use serde::Deserialize;
use std::collections::HashMap;

/// A type containing all the pages of the story.
///
/// Links each page id to the page it corresponds with.
pub type Pages = HashMap<usize, Page>;

/// A type containing flags and their associated values.
/// Each flag is a `char`;
pub type Flags = HashMap<char, i32>;

/// Represents one page.
///
/// Holds the id of the page, the content, and the links it will have
/// to further the story.
#[derive(Debug, Deserialize)]
pub struct Page {
    pub id: usize,
    pub content: String,
    pub actions: Option<Vec<FlagAction>>,
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct FlagAction {
    pub flag: char,
    pub effect: FlagEffect,
    pub modifier: i32,
}

#[derive(Debug, Deserialize)]
pub enum FlagEffect {
    Add,
    Set,
}

/// A link to a page that comes after the current page, specified by the id.
/// `cond` is a list of conditions that must be met for the link to be followed.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Link {
    /// A link to a different page with id `id`. A condition on a flag can be specified.
    Page { page: PageLink },
    /// A link providing an action, capable of setting or changing a flag from a link.
    /// If the link should only be able to be used once, set `repeatable` to `false`
    Action { action: ActionLink },
    /// A group of actions, which, once one of which is selected, will be unable to be used again
    Choice { choice: ChoiceLink },
}

#[derive(Debug, Deserialize)]
pub struct PageLink {
    pub id: usize,
    pub text: String,
    pub cond: Option<Vec<Cond>>,
}

#[derive(Debug, Deserialize)]
pub struct ActionLink {
    pub text: String,
    pub actions: Vec<FlagAction>,
    /// A set of conditions to required to select this action
    pub cond: Option<Vec<Cond>>,
    /// Specifies the number of times an action can be repeated, if `None`, infinite times.
    pub repeats: Option<usize>,
    /// Not to be specified by the user
    pub used: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ChoiceLink {
    pub caption: Option<String>,
    pub choices: Vec<ActionLink>,
    /// The number of times *choices* can be selected; each choice may have its own repeat limitation
    /// as well. If `None`, the choice can be selected an infinite number of times.
    pub repeats: Option<usize>,
    /// Not to be specified by the user
    pub used: Option<usize>,
}

/// A condition to follow a link.
#[derive(Debug, Deserialize)]
pub struct Cond {
    pub flag: char,
    pub cmp: CondCmp,
    pub num: i32,
}

impl Cond {
    /// Returns an error if the flag does not exist.
    /// Returns `Ok(true)` if the condition evaluates to true and `Ok(false)` otherwise.
    pub fn valid(&self, flags: &HashMap<char, i32>) -> Result<bool, &'static str> {
        match flags.get(&self.flag) {
            Some(val) => Ok(match self.cmp {
                CondCmp::Less => val < &self.num,
                CondCmp::Equal => val == &self.num,
                CondCmp::Greater => val > &self.num,
                CondCmp::AtLeast => val >= &self.num,
                CondCmp::AtMost => val <= &self.num,
            }),
            None => Err("Invalid flag"),
        }
    }
}

/// Possible condition comparisons
#[derive(Debug, Deserialize)]
pub enum CondCmp {
    Less,
    Equal,
    Greater,
    AtLeast,
    AtMost,
}
