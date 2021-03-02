#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub slug: &'static str,
    pub name: &'static str,
    pub bio: &'static str,
    pub email: &'static str,
    pub pronouns: &'static [&'static str],
    pub asked_tech: &'static [&'static str],
    pub urls: &'static [(&'static str, &'static str)],
    pub jobs: &'static [Job],
    pub contributions: &'static [Contribution],
    pub personnal_projects: &'static [Contribution],
    pub contract_type: ContractType,
    pub availability: Availability,
    pub certifications: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub company: &'static str,
    pub website: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub period: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contribution {
    pub project: &'static str,
    pub website: &'static str,
    pub tech: &'static [&'static str],
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContractType {
    Contractor,
    Employee,
    Any,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Availability {
    FullTime,
    PartTime,
    NotAvailable,
}
