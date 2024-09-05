// files contains stage names for all 8 countries
// maybe there is a way better way to match the stagenames
// (add all car names for groups)
pub mod locations {
    #[derive(Debug, Clone)]
    pub enum Finland {}
    pub enum Sardinia {}
    pub enum Japan {}
    pub enum Norway {}
    pub enum Germany {}
    pub enum Kenya {}
    pub enum Indonesia {}
    pub enum Australia {}

    impl Finland {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("noormarku"),
                2 => Some("lamppi"),
                3 => Some("palus"),
                4 => Some("lassila"),
                5 => Some("kairila"),
                6 => Some("haaparjarvi"),
                _ => None,
            }
        }
    }
    impl Sardinia {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("villacidro"),
                2 => Some("san gavino monreale"),
                3 => Some("san benedetto"),
                4 => Some("gennamari"),
                5 => Some("portu maga"),
                6 => Some("montevecchio"),
                _ => None,
            }
        }
    }
    impl Japan {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("nasu Highland"),
                2 => Some("mount Asama"),
                3 => Some("mount Akagi"),
                4 => Some("nikko"),
                5 => Some("tsumagoi"),
                6 => Some("mount Haruna"),
                _ => None,
            }
        }
    }
    impl Norway {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("laupstad"),
                2 => Some("vestpollen"),
                3 => Some("stronstad"),
                4 => Some("kvannkjosen"),
                5 => Some("grunnfor"),
                6 => Some("lake Rostavatn"),
                _ => None,
            }
        }
    }
    impl Germany {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("hockweiler"),
                2 => Some("franzenheim"),
                3 => Some("holzerath"),
                4 => Some("farschweiler"),
                5 => Some("mertesdorf"),
                6 => Some("gonnesweiler"),
                _ => None,
            }
        }
    }
    impl Kenya {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("mount kenya"),
                2 => Some("karura"),
                3 => Some("homa bay"),
                4 => Some("ndere island"),
                5 => Some("lake baringo"),
                6 => Some("lake nakuru"),
                _ => None,
            }
        }
    }
    impl Indonesia {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("mount kawi"),
                2 => Some("semangka island"),
                3 => Some("satonda island"),
                4 => Some("oreng valley"),
                5 => Some("sangeang island"),
                6 => Some("kalabakan island"),
                _ => None,
            }
        }
    }
    impl Australia {
        pub fn from_number(stage_number: u8) -> Option<&'static str> {
            match stage_number {
                1 => Some("gum scrub"),
                2 => Some("toorooka"),
                3 => Some("nulla nulla"),
                4 => Some("comara canyon"),
                5 => Some("lake lucernia"),
                6 => Some("wombamurra"),
                _ => None,
            }
        }
    }

    pub enum Countries {
        Finland,
        Sardinia,
        Japan,
        Norway,
        Germany,
        Kenya,
        Indonesia,
        Australia,
    }

    impl Countries {
        pub fn from_str(location: &str) -> Option<Countries> {
            match location {
                "Finland" => Some(Countries::Finland),
                "Sardinia" => Some(Countries::Sardinia),
                "Japan" => Some(Countries::Japan),
                "Norway" => Some(Countries::Norway),
                "Germany" => Some(Countries::Germany),
                "kenya" => Some(Countries::Kenya),
                "Indonesia" => Some(Countries::Indonesia),
                "Australia" => Some(Countries::Australia),
                _ => None,
            }
        }
    }
}
