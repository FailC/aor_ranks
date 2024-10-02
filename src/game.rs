// files contains stage names for all 8 countries
// maybe there is a way better way to match the stagenames
// (add all car names for groups)

pub mod locations {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    pub static LOCATIONS: Lazy<HashMap<&'static str, [&'static str; 6]>> =
        Lazy::new(|| get_locations());

    pub fn get_name<'a>(
        map: &'a HashMap<&'static str, [&'static str; 6]>,
        key: &str,
        index: usize,
    ) -> Option<&'static str> {
        let index = index - 1;
        map.get(key).and_then(|vec| vec.get(index)).map(|&s| s)
    }

    pub fn get_locations() -> HashMap<&'static str, [&'static str; 6]> {
        let mut map = HashMap::new();
        map.insert(
            "Finland",
            [
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
            [
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
            [
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
            [
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
            [
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
            [
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
            [
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
            [
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
