use heck::KebabCase;
use petname::Petnames;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use rand::distributions::uniform::SampleRange;
use rand::prelude::*;

#[rustfmt::skip]
const TECHS: &[&str] = &[
    "Rust", "JavaScript", "React", "Actix", "Rocket", "Python", "Django", "PostgreSQL", "MySQL",
    "Scala", "PlayFramwork", "Java", "Spring", "C#", "Hibernate", "Express.js", "Hapi.js",
    "aiohttp", "Sinatra", "Scalatra", "AWS", "Docker", "Terraform", "Serverless", "MongoDB",
    "Flask", "TypeScript", "Redux", "ReactiveX", "Angular", "Vue.js", "Ember.js", "Node.js",
    "F#",
];
const PRONOUNS: &[&str] = &["they/them", "he/him", "she/her", "it/it"];
#[rustfmt::skip]
const TIMEZONES: &[&str] = &[
    "Pacific__Midway", // UTC -11
    "Pacific__Rarotonga", //UTC -10
    "Pacific__Fakaofo", // UTC +13
    "Pacific__Kiritimati", // UTC +14
];

#[proc_macro]
pub fn make_mock_candidates(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let rng = &mut rand::thread_rng();
    let petname = &Petnames::default();

    let timezones: Vec<_> = TIMEZONES
        .iter()
        .map(|x| {
            let ident = Ident::new(x, Span::call_site());
            quote! { Tz::#ident }
        })
        .collect();

    let contract_types = &[quote! { ContractType::Any }];

    let availabilities = &[quote! { Availability::PartTime }];

    let candidates: TokenStream = (0..10)
        .map(|_| {
            let candidate = make_one_candidate(
                rng,
                petname,
                &timezones,
                &contract_types[..],
                &availabilities[..],
            );
            quote! {
                #candidate,
            }
        })
        .collect();

    (quote! {
        pub fn mock_candidates() -> &'static [&'static Candidate] {
            use chrono_tz::Tz;
            use std::str::FromStr;

            &[
                #candidates
            ]
        }
    })
    .into()
}

fn make_one_candidate(
    rng: &mut impl Rng,
    petname: &Petnames,
    available_timezones: &[TokenStream],
    contract_types: &[TokenStream],
    availabilities: &[TokenStream],
) -> TokenStream {
    let name = petname.generate(rng, 2, " ");
    let slug = name.to_kebab_case();
    let email = format!(
        "{}@{}.com",
        slug.replace('-', "."),
        petname.generate(rng, 1, ".")
    );

    let pronouns = choose_multiple(rng, 1..=2, PRONOUNS, |x| quote! { #x, });

    let birthday_y: i32 = rng.gen_range(1950..=2003);
    let birthday_m: u32 = rng.gen_range(1..=12);
    let birthday_d: u32 = rng.gen_range(1..=28);

    let timezones = choose_multiple(rng, 1..=2, available_timezones, |x| {
        quote! { #x, }
    });

    let bio = lipsum::lipsum(rng.gen_range(10..=350));

    let asked_techs = choose_multiple(rng, 5..=10, TECHS, |x| quote! { #x, });

    let not_wanted_techs = choose_multiple(rng, 1..=5, TECHS, |x| quote! { #x, });

    let all_urls = vec![
        ("GitHub", format!("https://github.com/{}", slug)),
        ("Twitter", format!("https://twitter.com/{}", slug)),
        (
            "StackOverflow",
            format!(
                "https://stackoverflow.com/users/{}",
                rng.gen_range(10000000..19999999)
            ),
        ),
        ("Reddit", format!("https://www.reddit.com/user/{}", slug)),
        ("WebSite", format!("https://www.{}.com/", slug)),
        ("Blog", format!("https://blog.{}.com/", slug)),
    ];
    let urls = choose_multiple(
        rng,
        4..=all_urls.len(),
        all_urls,
        |(label, url)| quote! { (#label, #url), },
    );

    let jobs: TokenStream = (0..rng.gen_range(1..10))
        .map(|_| {
            let n = rng.gen_range(1..2);
            let company = petname.generate(rng, n, "-");
            let website = format!("https://{}.com", company.to_kebab_case());
            let techs = choose_multiple(rng, 2..=6, TECHS, |x| quote! { #x, });
            let description = lipsum::lipsum(rng.gen_range(10..=350));
            let period = format!(
                "{}-{}",
                rng.gen_range(2000..2021),
                rng.gen_range(2000..2021)
            );

            quote! {
                Job {
                    company: #company,
                    website: #website,
                    techs: &[#techs],
                    description: #description,
                    period: #period,
                },
            }
        })
        .collect();

    let contributions: TokenStream = (0..rng.gen_range(0..25))
        .map(|_| generate_contribution(rng, petname))
        .collect();
    let personal_projects: TokenStream = (0..rng.gen_range(0..25))
        .map(|_| generate_contribution(rng, petname))
        .collect();

    let contract_type = contract_types.choose(rng).unwrap();

    let availability = availabilities.choose(rng).unwrap();

    let certifications: TokenStream = (0..rng.gen_range(0..5))
        .map(|_| {
            let n = rng.gen_range(3..8);
            let name = petname.generate(rng, n, " ");
            quote! { #name, }
        })
        .collect();

    quote! {
        &Candidate {
            slug: #slug,
            name: #name,
            pronouns: &[#pronouns],
            birthday_ymd: (#birthday_y, #birthday_m, #birthday_d),
            timezones: &[#timezones],
            bio: #bio,
            email: #email,
            asked_techs: &[#asked_techs],
            not_wanted_techs: &[#not_wanted_techs],
            urls: &[#urls],
            jobs: &[#jobs],
            contributions: &[#contributions],
            personal_projects: &[#personal_projects],
            contract_type: #contract_type,
            availability: #availability,
            certifications: &[#certifications],
        }
    }
}

fn generate_contribution(rng: &mut impl Rng, petname: &Petnames) -> TokenStream {
    let n = rng.gen_range(1..2);
    let project = petname.generate(rng, n, "-");
    let website = format!("https://{}.com", project.to_kebab_case());
    let techs = choose_multiple(rng, 2..=6, TECHS, |x| quote! { #x, });
    let description = lipsum::lipsum(rng.gen_range(10..=350));

    quote! {
        Contribution {
            project: #project,
            website: #website,
            techs: &[#techs],
            description: #description,
        },
    }
}

fn choose_multiple<T>(
    rng: &mut impl Rng,
    range: impl SampleRange<usize>,
    collection: impl IntoIterator<Item = T>,
    transform: impl Fn(T) -> TokenStream,
) -> TokenStream {
    let n: usize = rng.gen_range(range);
    collection
        .into_iter()
        .choose_multiple(rng, n)
        .into_iter()
        .map(transform)
        .collect()
}
