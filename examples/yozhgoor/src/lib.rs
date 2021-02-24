use candidate::*;
use std::time::Instant;

pub fn candidate() -> Candidate {
    Candidate {
        name: "Yohan Boogaert".to_string(),
        bio: "Former ambulance driver,
            I begin to learn about programming and Rust"
            .to_string(),
        email: "yozhgoor@outlook.com".to_string(),
        pronouns: vec!["he".to_string(), "him".to_string()],
        technologies: vec![
            "Rust".to_string(),
            "Markdown".to_string(),
            "HTML".to_string(),
        ],
        urls: vec![
            (
                "GitHub".to_string(),
                "https://github.com/Yozhgoor".to_string(),
            ),
            (
                "Twitter".to_string(),
                "https://twitter.com/yozhgoor".to_string(),
            ),
            (
                "StackOverflow".to_string(),
                "https://stackoverflow.com/users/14050514".to_string(),
            ),
            (
                "Reddit".to_string(),
                "https://www.reddit.com/user/yozhgoor".to_string(),
            ),
        ],
        jobs: vec![
            Job {
                company: "CJP Ambulances du centre".to_string(),
                website: "".to_string(),
                description: "As an ambulance driver".to_string(),
                technologies: vec!["".to_string()],
                period: "2018".to_string(),
            },
            Job {
                company: "Charly's Company".to_string(),
                website: "https://charlyscompany.be".to_string(),
                description: "as a LMV (Light Medical Vehicle) driver".to_string(),
                technologies: vec!["".to_string()],
                period: "2019".to_string(),
            },
        ],
        contributions: vec![
            Contribution {
                project: "Third-I by Big Boy System".to_string(),
                website: "http://bigboysystems.com".to_string(),
                technologies: vec![
                    "TypeScript".to_string(),
                    "React".to_string(),
                    "HTML".to_string(),
                    "CSS".to_string(),
                    "Embedded".to_string(),
                    "PWA".to_string(),
                ],
                description: "Third-I, the only camera that replicates your hearing
                    and your point of view in the first person."
                    .to_string(),
            },
            Contribution {
                project: "Wasm-run".to_string(),
                website: "https://crates.io/crates/wasm-run".to_string(),
                technologies: vec![
                    "Rust".to_string(),
                    "Wasm".to_string(),
                    "CLI".to_string(),
                    "Library".to_string(),
                    "Script".to_string(),
                ],
                description: "Build tool that replaces cargo run to build WASM projects.
                    Just like webpack, wasm-run offers a great deal of customization."
                    .to_string(),
            },
            Contribution {
                project: "Yewprint".to_string(),
                website: "https://yewprint.rm.rs/".to_string(),
                technologies: vec![
                    "Rust".to_string(),
                    "Wasm".to_string(),
                    "Yew".to_string(),
                    "Library".to_string(),
                    "HTML".to_string(),
                    "CSS".to_string(),
                    "Script".to_string(),
                ],
                description: "Port of blueprintjs to yew".to_string(),
            },
        ],
        personnal_projects: vec![
            Contribution {
                project: "Yewprint-playground".to_string(),
                website: "https://github.com/yozhgoor/yewprint-playground".to_string(),
                technologies: vec![
                    "Rust".to_string(),
                    "Wasm".to_string(),
                    "Yew".to_string(),
                    "HTML".to_string(),
                    "CSS".to_string(),
                    "Netlify".to_string(),
                ],
                description: "a Yew and yewprint playground using wasm-run".to_string(),
            },
            Contribution {
                project: "Rusty-snake".to_string(),
                website: "https://github.com/yozhgoor/rusty-snake".to_string(),
                technologies: vec![
                    "Rust".to_string(),
                    "CLI".to_string(),
                    "Game".to_string(),
                    "Design".to_string(),
                ],
                description: "Snake, the game in your terminal".to_string(),
            },
            Contribution {
                project: "Rusty-snake".to_string(),
                website: "https://github.com/yozhgoor/rusty-snake".to_string(),
                technologies: vec![
                    "Rust".to_string(),
                    "CLI".to_string(),
                    "Game".to_string(),
                    "Design".to_string(),
                ],
                description: "Snake, the game in your terminal".to_string(),
            },
            Contribution {
                project: "Introduction à Rust".to_string(),
                website: "https://intro-to-rust.netlify.app".to_string(),
                technologies: vec![
                    "Rust".to_string(),
                    "MdBook".to_string(),
                    "Tutorial".to_string(),
                    "Netlify".to_string(),
                ],
                description: "My vision of the Rust Programming Language,
                    as an introduction to Rust"
                    .to_string(),
            },
        ],
        contract_type: ContractType::Contractor,
        availability: Availability::NotAvailable,
        joined_date: Instant::now(),
        certifications: None,
    }
}
