pub struct Candidate {
    pub name: String,
    pub bio: String,
    pub email: String,
    pub pronouns: Vec<String>,
    pub technologies: Vec<String>,
    pub urls: Vec<(String, String)>,
    pub jobs: Vec<Job>,
    pub contributions: Vec<Contribution>,
    pub personnal_project: Vec<Contribution>,
    pub contract_type: ContractType,
    pub availability: Availability,
    pub joined_date: std::time::Instant,
    pub certifications: Vec<String>,
}

pub struct Job {
    pub company: String,
    pub website: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub period: String,
}

pub struct Contribution {
    pub project: String,
    pub website: String,
    pub technologies: Vec<String>,
    pub description: String,
}

pub enum ContractType {
    Contractor,
    Employee,
    Any,
}

pub enum Availability {
    FullTime,
    PartTime,
    NotAvailable,
}
