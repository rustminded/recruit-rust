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
    "Africa__Abidjan", "Africa__Accra", "Africa__Addis_Ababa", "Africa__Algiers", "Africa__Asmara",
    "Africa__Asmera", "Africa__Bamako", "Africa__Bangui", "Africa__Banjul", "Africa__Bissau",
    "Africa__Blantyre", "Africa__Brazzaville", "Africa__Bujumbura", "Africa__Cairo",
    "Africa__Casablanca", "Africa__Ceuta", "Africa__Conakry", "Africa__Dakar",
    "Africa__Dar_es_Salaam", "Africa__Djibouti", "Africa__Douala", "Africa__El_Aaiun",
    "Africa__Freetown", "Africa__Gaborone", "Africa__Harare", "Africa__Johannesburg",
    "Africa__Juba", "Africa__Kampala", "Africa__Khartoum", "Africa__Kigali", "Africa__Kinshasa",
    "Africa__Lagos", "Africa__Libreville", "Africa__Lome", "Africa__Luanda", "Africa__Lubumbashi",
    "Africa__Lusaka", "Africa__Malabo", "Africa__Maputo", "Africa__Maseru", "Africa__Mbabane",
    "Africa__Mogadishu", "Africa__Monrovia", "Africa__Nairobi", "Africa__Ndjamena",
    "Africa__Niamey", "Africa__Nouakchott", "Africa__Ouagadougou", "Africa__PortoNovo",
    "Africa__Sao_Tome", "Africa__Timbuktu", "Africa__Tripoli", "Africa__Tunis", "Africa__Windhoek",
    "America__Adak", "America__Anchorage", "America__Anguilla", "America__Antigua",
    "America__Araguaina", "America__Argentina__Buenos_Aires", "America__Argentina__Catamarca",
    "America__Argentina__ComodRivadavia", "America__Argentina__Cordoba",
    "America__Argentina__Jujuy", "America__Argentina__La_Rioja", "America__Argentina__Mendoza",
    "America__Argentina__Rio_Gallegos", "America__Argentina__Salta",
    "America__Argentina__San_Juan", "America__Argentina__San_Luis", "America__Argentina__Tucuman",
    "America__Argentina__Ushuaia", "America__Aruba", "America__Asuncion", "America__Atikokan",
    "America__Atka", "America__Bahia", "America__Bahia_Banderas", "America__Barbados",
    "America__Belem", "America__Belize", "America__BlancSablon", "America__Boa_Vista",
    "America__Bogota", "America__Boise", "America__Buenos_Aires", "America__Cambridge_Bay",
    "America__Campo_Grande", "America__Cancun", "America__Caracas", "America__Catamarca",
    "America__Cayenne", "America__Cayman", "America__Chicago", "America__Chihuahua",
    "America__Coral_Harbour", "America__Cordoba", "America__Costa_Rica", "America__Creston",
    "America__Cuiaba", "America__Curacao", "America__Danmarkshavn", "America__Dawson",
    "America__Dawson_Creek", "America__Denver", "America__Detroit", "America__Dominica",
    "America__Edmonton", "America__Eirunepe", "America__El_Salvador", "America__Ensenada",
    "America__Fort_Nelson", "America__Fort_Wayne", "America__Fortaleza", "America__Glace_Bay",
    "America__Godthab", "America__Goose_Bay", "America__Grand_Turk", "America__Grenada",
    "America__Guadeloupe", "America__Guatemala", "America__Guayaquil", "America__Guyana",
    "America__Halifax", "America__Havana", "America__Hermosillo", "America__Indiana__Indianapolis",
    "America__Indiana__Knox", "America__Indiana__Marengo", "America__Indiana__Petersburg",
    "America__Indiana__Tell_City", "America__Indiana__Vevay", "America__Indiana__Vincennes",
    "America__Indiana__Winamac", "America__Indianapolis", "America__Inuvik", "America__Iqaluit",
    "America__Jamaica", "America__Jujuy", "America__Juneau", "America__Kentucky__Louisville",
    "America__Kentucky__Monticello", "America__Knox_IN", "America__Kralendijk", "America__La_Paz",
    "America__Lima", "America__Los_Angeles", "America__Louisville", "America__Lower_Princes",
    "America__Maceio", "America__Managua", "America__Manaus", "America__Marigot",
    "America__Martinique", "America__Matamoros", "America__Mazatlan", "America__Mendoza",
    "America__Menominee", "America__Merida", "America__Metlakatla", "America__Mexico_City",
    "America__Miquelon", "America__Moncton", "America__Monterrey", "America__Montevideo",
    "America__Montreal", "America__Montserrat", "America__Nassau", "America__New_York",
    "America__Nipigon", "America__Nome", "America__Noronha", "America__North_Dakota__Beulah",
    "America__North_Dakota__Center", "America__North_Dakota__New_Salem", "America__Nuuk",
    "America__Ojinaga", "America__Panama", "America__Pangnirtung", "America__Paramaribo",
    "America__Phoenix", "America__PortauPrince", "America__Port_of_Spain", "America__Porto_Acre",
    "America__Porto_Velho", "America__Puerto_Rico", "America__Punta_Arenas",
    "America__Rainy_River", "America__Rankin_Inlet", "America__Recife", "America__Regina",
    "America__Resolute", "America__Rio_Branco", "America__Rosario", "America__Santa_Isabel",
    "America__Santarem", "America__Santiago", "America__Santo_Domingo", "America__Sao_Paulo",
    "America__Scoresbysund", "America__Shiprock", "America__Sitka", "America__St_Barthelemy",
    "America__St_Johns", "America__St_Kitts", "America__St_Lucia", "America__St_Thomas",
    "America__St_Vincent", "America__Swift_Current", "America__Tegucigalpa", "America__Thule",
    "America__Thunder_Bay", "America__Tijuana", "America__Toronto", "America__Tortola",
    "America__Vancouver", "America__Virgin", "America__Whitehorse", "America__Winnipeg",
    "America__Yakutat", "America__Yellowknife", "Antarctica__Casey", "Antarctica__Davis",
    "Antarctica__DumontDUrville", "Antarctica__Macquarie", "Antarctica__Mawson",
    "Antarctica__McMurdo", "Antarctica__Palmer", "Antarctica__Rothera", "Antarctica__South_Pole",
    "Antarctica__Syowa", "Antarctica__Troll", "Antarctica__Vostok", "Arctic__Longyearbyen",
    "Asia__Aden", "Asia__Almaty", "Asia__Amman", "Asia__Anadyr", "Asia__Aqtau", "Asia__Aqtobe",
    "Asia__Ashgabat", "Asia__Ashkhabad", "Asia__Atyrau", "Asia__Baghdad", "Asia__Bahrain",
    "Asia__Baku", "Asia__Bangkok", "Asia__Barnaul", "Asia__Beirut", "Asia__Bishkek",
    "Asia__Brunei", "Asia__Calcutta", "Asia__Chita", "Asia__Choibalsan", "Asia__Chongqing",
    "Asia__Chungking", "Asia__Colombo", "Asia__Dacca", "Asia__Damascus", "Asia__Dhaka",
    "Asia__Dili", "Asia__Dubai", "Asia__Dushanbe", "Asia__Famagusta", "Asia__Gaza", "Asia__Harbin",
    "Asia__Hebron", "Asia__Ho_Chi_Minh", "Asia__Hong_Kong", "Asia__Hovd", "Asia__Irkutsk",
    "Asia__Istanbul", "Asia__Jakarta", "Asia__Jayapura", "Asia__Jerusalem", "Asia__Kabul",
    "Asia__Kamchatka", "Asia__Karachi", "Asia__Kashgar", "Asia__Kathmandu", "Asia__Katmandu",
    "Asia__Khandyga", "Asia__Kolkata", "Asia__Krasnoyarsk", "Asia__Kuala_Lumpur", "Asia__Kuching",
    "Asia__Kuwait", "Asia__Macao", "Asia__Macau", "Asia__Magadan", "Asia__Makassar",
    "Asia__Manila", "Asia__Muscat", "Asia__Nicosia", "Asia__Novokuznetsk", "Asia__Novosibirsk",
    "Asia__Omsk", "Asia__Oral", "Asia__Phnom_Penh", "Asia__Pontianak", "Asia__Pyongyang",
    "Asia__Qatar", "Asia__Qostanay", "Asia__Qyzylorda", "Asia__Rangoon", "Asia__Riyadh",
    "Asia__Saigon", "Asia__Sakhalin", "Asia__Samarkand", "Asia__Seoul", "Asia__Shanghai",
    "Asia__Singapore", "Asia__Srednekolymsk", "Asia__Taipei", "Asia__Tashkent", "Asia__Tbilisi",
    "Asia__Tehran", "Asia__Tel_Aviv", "Asia__Thimbu", "Asia__Thimphu", "Asia__Tokyo",
    "Asia__Tomsk", "Asia__Ujung_Pandang", "Asia__Ulaanbaatar", "Asia__Ulan_Bator", "Asia__Urumqi",
    "Asia__UstNera", "Asia__Vientiane", "Asia__Vladivostok", "Asia__Yakutsk", "Asia__Yangon",
    "Asia__Yekaterinburg", "Asia__Yerevan", "Atlantic__Azores", "Atlantic__Bermuda",
    "Atlantic__Canary", "Atlantic__Cape_Verde", "Atlantic__Faeroe", "Atlantic__Faroe",
    "Atlantic__Jan_Mayen", "Atlantic__Madeira", "Atlantic__Reykjavik", "Atlantic__South_Georgia",
    "Atlantic__St_Helena", "Atlantic__Stanley", "Australia__ACT", "Australia__Adelaide",
    "Australia__Brisbane", "Australia__Broken_Hill", "Australia__Canberra", "Australia__Currie",
    "Australia__Darwin", "Australia__Eucla", "Australia__Hobart", "Australia__LHI",
    "Australia__Lindeman", "Australia__Lord_Howe", "Australia__Melbourne", "Australia__NSW",
    "Australia__North", "Australia__Perth", "Australia__Queensland", "Australia__South",
    "Australia__Sydney", "Australia__Tasmania", "Australia__Victoria", "Australia__West",
    "Australia__Yancowinna", "Brazil__Acre", "Brazil__DeNoronha", "Brazil__East", "Brazil__West",
    "CET", "CST6CDT", "Canada__Atlantic", "Canada__Central", "Canada__Eastern", "Canada__Mountain",
    "Canada__Newfoundland", "Canada__Pacific", "Canada__Saskatchewan", "Canada__Yukon",
    "Chile__Continental", "Chile__EasterIsland", "Cuba", "EET", "EST", "EST5EDT", "Egypt", "Eire",
    "Etc__GMT", "Etc__GMTPlus0", "Etc__GMTPlus1", "Etc__GMTPlus10", "Etc__GMTPlus11",
    "Etc__GMTPlus12", "Etc__GMTPlus2", "Etc__GMTPlus3", "Etc__GMTPlus4", "Etc__GMTPlus5",
    "Etc__GMTPlus6", "Etc__GMTPlus7", "Etc__GMTPlus8", "Etc__GMTPlus9", "Etc__GMTMinus0",
    "Etc__GMTMinus1", "Etc__GMTMinus10", "Etc__GMTMinus11", "Etc__GMTMinus12", "Etc__GMTMinus13",
    "Etc__GMTMinus14", "Etc__GMTMinus2", "Etc__GMTMinus3", "Etc__GMTMinus4", "Etc__GMTMinus5",
    "Etc__GMTMinus6", "Etc__GMTMinus7", "Etc__GMTMinus8", "Etc__GMTMinus9", "Etc__GMT0",
    "Etc__Greenwich", "Etc__UCT", "Etc__UTC", "Etc__Universal", "Etc__Zulu", "Europe__Amsterdam",
    "Europe__Andorra", "Europe__Astrakhan", "Europe__Athens", "Europe__Belfast",
    "Europe__Belgrade", "Europe__Berlin", "Europe__Bratislava", "Europe__Brussels",
    "Europe__Bucharest", "Europe__Budapest", "Europe__Busingen", "Europe__Chisinau",
    "Europe__Copenhagen", "Europe__Dublin", "Europe__Gibraltar", "Europe__Guernsey",
    "Europe__Helsinki", "Europe__Isle_of_Man", "Europe__Istanbul", "Europe__Jersey",
    "Europe__Kaliningrad", "Europe__Kiev", "Europe__Kirov", "Europe__Lisbon", "Europe__Ljubljana",
    "Europe__London", "Europe__Luxembourg", "Europe__Madrid", "Europe__Malta", "Europe__Mariehamn",
    "Europe__Minsk", "Europe__Monaco", "Europe__Moscow", "Europe__Nicosia", "Europe__Oslo",
    "Europe__Paris", "Europe__Podgorica", "Europe__Prague", "Europe__Riga", "Europe__Rome",
    "Europe__Samara", "Europe__San_Marino", "Europe__Sarajevo", "Europe__Saratov",
    "Europe__Simferopol", "Europe__Skopje", "Europe__Sofia", "Europe__Stockholm",
    "Europe__Tallinn", "Europe__Tirane", "Europe__Tiraspol", "Europe__Ulyanovsk",
    "Europe__Uzhgorod", "Europe__Vaduz", "Europe__Vatican", "Europe__Vienna", "Europe__Vilnius",
    "Europe__Volgograd", "Europe__Warsaw", "Europe__Zagreb", "Europe__Zaporozhye",
    "Europe__Zurich", "GB", "GBEire", "GMT", "GMTPlus0", "GMTMinus0", "GMT0", "Greenwich", "HST",
    "Hongkong", "Iceland", "Indian__Antananarivo", "Indian__Chagos", "Indian__Christmas",
    "Indian__Cocos", "Indian__Comoro", "Indian__Kerguelen", "Indian__Mahe", "Indian__Maldives",
    "Indian__Mauritius", "Indian__Mayotte", "Indian__Reunion", "Iran", "Israel", "Jamaica",
    "Japan", "Kwajalein", "Libya", "MET", "MST", "MST7MDT", "Mexico__BajaNorte", "Mexico__BajaSur",
    "Mexico__General", "NZ", "NZCHAT", "Navajo", "PRC", "PST8PDT", "Pacific__Apia",
    "Pacific__Auckland", "Pacific__Bougainville", "Pacific__Chatham", "Pacific__Chuuk",
    "Pacific__Easter", "Pacific__Efate", "Pacific__Enderbury", "Pacific__Fakaofo", "Pacific__Fiji",
    "Pacific__Funafuti", "Pacific__Galapagos", "Pacific__Gambier", "Pacific__Guadalcanal",
    "Pacific__Guam", "Pacific__Honolulu", "Pacific__Johnston", "Pacific__Kiritimati",
    "Pacific__Kosrae", "Pacific__Kwajalein", "Pacific__Majuro", "Pacific__Marquesas",
    "Pacific__Midway", "Pacific__Nauru", "Pacific__Niue", "Pacific__Norfolk", "Pacific__Noumea",
    "Pacific__Pago_Pago", "Pacific__Palau", "Pacific__Pitcairn", "Pacific__Pohnpei",
    "Pacific__Ponape", "Pacific__Port_Moresby", "Pacific__Rarotonga", "Pacific__Saipan",
    "Pacific__Samoa", "Pacific__Tahiti", "Pacific__Tarawa", "Pacific__Tongatapu", "Pacific__Truk",
    "Pacific__Wake", "Pacific__Wallis", "Pacific__Yap", "Poland", "Portugal", "ROC", "ROK",
    "Singapore", "Turkey", "UCT", "US__Alaska", "US__Aleutian", "US__Arizona", "US__Central",
    "US__EastIndiana", "US__Eastern", "US__Hawaii", "US__IndianaStarke", "US__Michigan",
    "US__Mountain", "US__Pacific", "US__PacificNew", "US__Samoa", "UTC", "Universal", "WSU", "WET",
    "Zulu",
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

    let contract_types = &[quote! { ContractType::Contractor }];

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
