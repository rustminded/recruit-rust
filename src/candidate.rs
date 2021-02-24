pub struct Candidate {
    name: String,
    bio: String,
    email: String,
    pronouns: Vec<String>,
    technologies: Vec<String>,
    urls: Vec<(String, String)>,
    jobs: Vec<Job>,
    contributions: Vec<Contribution>,
    personnal_project: Vec<Contribution>,
    contract_type: ContractType,
    availability: Availability,
    joined_date: std::time::Instant,
    certifications: Vec<String>,
}

pub struct Job {
    company: String,
    website: String,
    description: String,
    technologies: Vec<String>,
    period: String,
}

pub struct Contribution {
    project: String,
    website: String,
    technologies: Vec<String>,
    description: String,
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
