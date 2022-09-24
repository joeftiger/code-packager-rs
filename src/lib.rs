use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct Tags<'a> {
    /// Include code within the range of these tags.
    include: HashSet<&'a str>,
    /// Exclude code within the range of these tags, even if they are surrounded by a tag to be
    /// included.
    exclude: HashSet<&'a str>,
    /// The 'end' tag to mark the end of an in-/excluded section.
    end_tag: &'a str,
    /// The prefix for each tag.
    tag_prefix: &'a str,
}

impl<'a> Tags<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag_prefix(mut self, tag_prefix: &'a str) -> Self {
        self.tag_prefix = tag_prefix;
        self
    }

    pub fn end_tag(mut self, end_tag: &'a str) -> Self {
        self.end_tag = end_tag;
        self
    }

    pub fn add_include(mut self, include: &'a str) -> Self {
        self.include.insert(include);
        self
    }

    pub fn include_all(self, include: impl Iterator<Item = &'a str>) -> Self {
        include.into_iter().fold(self, Self::add_include)
    }

    pub fn remove_include(mut self, include: &'a str) -> Self {
        self.include.remove(include);
        self
    }

    pub fn add_exclude(mut self, exclude: &'a str) -> Self {
        self.exclude.insert(exclude);
        self
    }

    pub fn exclude_all(self, exclude: impl Iterator<Item = &'a str>) -> Self {
        exclude.into_iter().fold(self, Self::add_exclude)
    }

    pub fn remove_exclude(mut self, exclude: &'a str) -> Self {
        self.exclude.remove(exclude);
        self
    }

    pub fn package(&self, content: &str) -> Result<String, TagError> {
        let mut out = Vec::new();

        // the stack in which the top most element tells us whether to include the line or not.
        let mut stack = VecDeque::new();
        // shadowed stack which is wrapped inside a `exclude`.
        let mut shadow_stack = VecDeque::new();

        let mut line_num = 0;
        for line in content.lines() {
            line_num += 1; // start at 1 as opposed to `content.lines().enumerate()` at 0

            // choose correct stack to operate on for in-/excludes
            let s = match stack.back() {
                Some(&false) => &mut shadow_stack,
                _ => &mut stack,
            };

            match TagType::from(self, line) {
                TagType::Include => s.push_back(true),
                TagType::Exclude => s.push_back(false),
                TagType::End => {
                    if shadow_stack.pop_back().is_none() && stack.pop_back().is_none() {
                        return Err(TagError::UnexpectedEndTag(line_num));
                    }
                }
                TagType::None => {
                    if stack.is_empty() || stack.back() == Some(&true) {
                        out.push(line)
                    }
                }
            }
        }

        match stack.is_empty() {
            true => {
                // work around str::lines() ignoring the last empty line
                let packaged = match content.ends_with('\n') {
                    true => out.join("\n") + "\n",
                    false => out.join("\n"),
                };
                Ok(packaged)
            }
            false => Err(TagError::ExpectedEndTag(line_num)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum TagType {
    Include,
    Exclude,
    End,
    None,
}

impl TagType {
    fn from(tags: &Tags, line: &str) -> TagType {
        if let Some((_, mut tag)) = line.split_once(tags.tag_prefix) {
            tag = tag.trim();

            if tags.end_tag == tag {
                Self::End
            } else if tags.include.contains(tag) {
                Self::Include
            } else if tags.exclude.contains(tag) {
                Self::Exclude
            } else {
                Self::None
            }
        } else {
            Self::None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TagError {
    UnexpectedEndTag(usize),
    ExpectedEndTag(usize),
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEndTag(line_num) => {
                write!(f, "unexpected `end` tag on line: {}", line_num)
            }
            Self::ExpectedEndTag(line_num) => {
                write!(f, "expected at least one `end` tag on line: {}", line_num)
            }
        }
    }
}

impl Error for TagError {}
