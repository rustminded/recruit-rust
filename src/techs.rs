use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Tech {
    pub value: Cow<'static, str>,
    pub is_professional: bool,
    pub is_public: bool,
    pub is_asked: bool,
    pub is_not_wanted: bool,
}

impl Tech {
    pub fn with_asked(mut self) -> Self {
        self.is_asked = true;
        self
    }

    pub fn with_not_wanted(mut self) -> Self {
        self.is_not_wanted = true;
        self
    }

    pub fn with_pro(mut self) -> Self {
        self.is_professional = true;
        self
    }

    pub fn with_pub(mut self) -> Self {
        self.is_public = true;
        self
    }
}

impl Hash for Tech {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_lowercase().hash(state);
    }
}

impl Eq for Tech {}

impl PartialEq for Tech {
    fn eq(&self, other: &Self) -> bool {
        self.value.to_lowercase().eq(&other.value.to_lowercase())
    }
}

impl From<&'static str> for Tech {
    fn from(s: &'static str) -> Self {
        Tech {
            value: Cow::Borrowed(s),
            is_asked: false,
            is_not_wanted: false,
            is_professional: false,
            is_public: false,
        }
    }
}

impl From<String> for Tech {
    fn from(s: String) -> Self {
        Tech {
            value: Cow::Owned(s),
            is_asked: false,
            is_not_wanted: false,
            is_professional: false,
            is_public: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TechSet(HashSet<Tech>);

impl TechSet {
    pub fn new() -> TechSet {
        TechSet(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Tech> {
        self.0.iter()
    }

    pub fn is_disjoint(&self, other: &TechSet) -> bool {
        self.0.is_disjoint(&other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Eq for TechSet {}

impl PartialEq for TechSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Extend<Tech> for TechSet {
    fn extend<I: IntoIterator<Item = Tech>>(&mut self, iter: I) {
        for elem in iter {
            if let Some(mut v) = self.0.take(&elem) {
                v.is_professional |= elem.is_professional;
                v.is_public |= elem.is_public;
                v.is_asked |= elem.is_asked;
                self.0.insert(v);
            } else {
                self.0.insert(elem);
            }
        }
    }
}

impl std::iter::FromIterator<Tech> for TechSet {
    fn from_iter<I: IntoIterator<Item = Tech>>(iter: I) -> Self {
        let mut tech_set = TechSet::new();
        tech_set.extend(iter);
        tech_set
    }
}
