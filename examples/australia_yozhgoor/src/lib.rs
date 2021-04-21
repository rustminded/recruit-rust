use candidate::*;
use chrono_tz::Australia;

pub fn candidate() -> &'static Candidate {
    &Candidate {
        slug: "australian",
        name: "Australian",
        pronouns: &["he", "him"],
        birthday_ymd: &(2003, 04, 20),
        timezones: &[Australia::Melbourne],
        bio: "Former ambulance driver, \
            I'm learning programming with Rust",
        email: "yozhgoor@outlook.com",
        asked_techs: &["Rust", "WASM", "PWA"],
        urls: &[
            ("GitHub", "https://github.com/Yozhgoor"),
            ("Twitter", "https://twitter.com/yozhgoor"),
            ("StackOverflow", "https://stackoverflow.com/users/14050514"),
            ("Reddit", "https://www.reddit.com/user/yozhgoor"),
        ],
        jobs: &[Job {
            company: "RustMinded",
            website: "https://github.com/rustminded",
            description: "Learn programming and Rust \
                    as a Junior Consultant Developer",
            techs: &["Rust"],
            period: "2021",
        }],
        contributions: &[
            Contribution {
                project: "Third-I by Big Boy System",
                website: "http://bigboysystems.com",
                techs: &["TypeScript", "React", "HTML", "CSS", "PWA"],
                description: "Third-I, the only camera that replicates your hearing \
                    and your point of view in the first person.",
            },
            Contribution {
                project: "wasm-run",
                website: "https://crates.io/crates/wasm-run",
                techs: &["Rust", "WASM"],
                description: "Build tool that replaces cargo run to build WASM projects.",
            },
            Contribution {
                project: "Yewprint",
                website: "https://yewprint.rm.rs/",
                techs: &["Rust", "WASM", "Yew", "Netlify"],
                description: "Port of blueprintjs to yew",
            },
        ],
        personal_projects: &[
            Contribution {
                project: "Yewprint-playground",
                website: "https://github.com/yozhgoor/yewprint-playground",
                techs: &["Rust", "WASM", "Yew", "CSS"],
                description: "a Yew and yewprint playground using wasm-run",
            },
            Contribution {
                project: "Rusty-snake",
                website: "https://github.com/yozhgoor/rusty-snake",
                techs: &["Rust"],
                description: "Snake, the game in your terminal",
            },
            Contribution {
                project: "Introduction à Rust",
                website: "https://intro-to-rust.netlify.app",
                techs: &["Rust"],
                description: "My vision of the Rust Programming Language, \
                    as an introduction to Rust",
            },
        ],
        contract_type: ContractType::Contractor,
        availability: Availability::PartTime,
        certifications: &[],
    }
}
