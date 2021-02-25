use candidate::*;

pub fn candidate() -> Candidate {
    Candidate {
        name: "Yohan Boogaert",
        bio: "Former ambulance driver, \
            I'm starting to learn about programming and Rust",
        email: "yozhgoor@outlook.com",
        pronouns: vec!["he", "him"],
        asked_tech: vec!["Rust", "WASM", "PWA"],
        urls: vec![
            ("GitHub", "https://github.com/Yozhgoor"),
            ("Twitter", "https://twitter.com/yozhgoor"),
            ("StackOverflow", "https://stackoverflow.com/users/14050514"),
            ("Reddit", "https://www.reddit.com/user/yozhgoor"),
        ],
        jobs: vec![Job {
            company: "RustMinded",
            website: "https://github.com/rustminded",
            description: "Learn programming and Rust \
                    as a Junior Consultant Developper",
            tech: vec!["Rust"],
            period: "2021",
        }],
        contributions: vec![
            Contribution {
                project: "Third-I by Big Boy System",
                website: "http://bigboysystems.com",
                tech: vec!["TypeScript", "React", "HTML", "CSS", "PWA"],
                description: "Third-I, the only camera that replicates your hearing \
                    and your point of view in the first person.",
            },
            Contribution {
                project: "wasm-run",
                website: "https://crates.io/crates/wasm-run",
                tech: vec!["Rust", "WASM"],
                description: "Build tool that replaces cargo run to build WASM projects.",
            },
            Contribution {
                project: "Yewprint",
                website: "https://yewprint.rm.rs/",
                tech: vec!["Rust", "WASM", "Yew", "Netlify"],
                description: "Port of blueprintjs to yew",
            },
        ],
        personnal_projects: vec![
            Contribution {
                project: "Yewprint-playground",
                website: "https://github.com/yozhgoor/yewprint-playground",
                tech: vec!["Rust", "WASM", "Yew", "CSS"],
                description: "a Yew and yewprint playground using wasm-run",
            },
            Contribution {
                project: "Rusty-snake",
                website: "https://github.com/yozhgoor/rusty-snake",
                tech: vec!["Rust"],
                description: "Snake, the game in your terminal",
            },
            Contribution {
                project: "Introduction à Rust",
                website: "https://intro-to-rust.netlify.app",
                tech: vec!["Rust"],
                description: "My vision of the Rust Programming Language, \
                    as an introduction to Rust",
            },
        ],
        contract_type: ContractType::Contractor,
        availability: Availability::NotAvailable,
        certifications: vec![],
    }
}
