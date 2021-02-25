#[derive(Debug)]
pub struct Candidate {
    pub name: String,
    pub bio: String,
    pub email: String,
    pub pronouns: Vec<String>,
    pub asked_tech: Vec<String>,
    pub urls: Vec<(String, String)>,
    pub jobs: Vec<Job>,
    pub contributions: Vec<Contribution>,
    pub personnal_projects: Vec<Contribution>,
    pub contract_type: ContractType,
    pub availability: Availability,
    pub joined_date: std::time::Instant,
    pub certifications: Vec<String>,
}

#[derive(Debug)]
pub struct Job {
    pub company: String,
    pub website: String,
    pub description: String,
    pub tech: Vec<String>,
    pub period: String,
}

#[derive(Debug)]
pub struct Contribution {
    pub project: String,
    pub website: String,
    pub tech: Vec<String>,
    pub description: String,
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
