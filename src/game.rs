// files contains stage names for all 8 countries
// maybe there is a way better way to match the stagenames
// (add all car names for groups)

pub mod locations {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    pub static LOCATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> =
        Lazy::new(|| get_locations());

    pub fn get_name<'a>(
        map: &'a HashMap<&'static str, Vec<&'static str>>,
        key: &str,
        index: usize,
    ) -> Option<&'static str> {
        let index = index - 1;
        map.get(key).and_then(|vec| vec.get(index)).map(|&s| s)
    }

    pub fn get_locations() -> HashMap<&'static str, Vec<&'static str>> {
        let mut map = HashMap::new();
        map.insert(
            "Finland",
            vec![
                "noormarku",
                "lamppi",
                "palus",
                "lassila",
                "kairila",
                "haaparjarvi",
            ],
        );
        map.insert(
            "Sardinia",
            vec![
                "villacidro",
                "san gavino monreale",
                "san benedetto",
                "gennamari",
                "portu maga",
                "montevecchio",
            ],
        );
        map.insert(
            "Japan",
            vec![
                "nasu highland",
                "mount asama",
                "mount akagi",
                "nikko",
                "tsumagoi",
                "mount haruna",
            ],
        );
        map.insert(
            "Norway",
            vec![
                "laupstad",
                "vestpollen",
                "stronstad",
                "kvannkjosen",
                "grunnfor",
                "lake rostavatn",
            ],
        );
        map.insert(
            "Germany",
            vec![
                "hockweiler",
                "franzenheim",
                "holzerath",
                "farschweiler",
                "mertesdorf",
                "gonnesweiler",
            ],
        );

        map.insert(
            "Kenya",
            vec![
                "mount kenya",
                "karura",
                "homa bay",
                "ndere island",
                "lake baringo",
                "lake nakuru",
            ],
        );

        map.insert(
            "Indonesia",
            vec![
                "mount kawi",
                "semangka island",
                "satonda island",
                "oreng valley",
                "sangeang island",
                "kalabakan island",
            ],
        );

        map.insert(
            "Australia",
            vec![
                "gum scrub",
                "toorooka",
                "nulla nulla",
                "comara canyon",
                "lake lucernia",
                "wombamurra",
            ],
        );
        map
    }
}
