#[derive(Debug)]
pub struct Candidate {
    pub name: &'static str,
    pub bio: &'static str,
    pub email: &'static str,
    pub pronouns: Vec<&'static str>,
    pub asked_tech: Vec<&'static str>,
    pub urls: Vec<(&'static str, &'static str)>,
    pub jobs: Vec<Job>,
    pub contributions: Vec<Contribution>,
    pub personnal_projects: Vec<Contribution>,
    pub contract_type: ContractType,
    pub availability: Availability,
    pub joined_date: std::time::Instant,
    pub certifications: Vec<&'static str>,
}

#[derive(Debug)]
pub struct Job {
    pub company: &'static str,
    pub website: &'static str,
    pub description: &'static str,
    pub tech: Vec<&'static str>,
    pub period: &'static str,
}

#[derive(Debug)]
pub struct Contribution {
    pub project: &'static str,
    pub website: &'static str,
    pub tech: Vec<&'static str>,
    pub description: &'static str,
}

#[derive(Debug)]
pub enum ContractType {
    Contractor,
    Employee,
    Any,
}

#[derive(Debug)]
pub enum Availability {
    FullTime,
    PartTime,
    NotAvailable,
}
