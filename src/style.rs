use log::debug;
use std::collections::HashMap;

type PrioStyleLine<'a> = (u32, &'a str);

pub struct Style<'a>(HashMap<&'a str, (HashMap<&'a str, PrioStyleLine<'a>>, PrioStyleLine<'a>)>);

impl Style<'_> {
    pub fn new() -> Style<'static> {
        Style(HashMap::from([
            (
                "highway",
                (
                    HashMap::from([
                        ("path", (100, "stroke:#002a5a;fill:none")),
                        ("residential", (100, "stroke-width:3;stroke:black;fill:none")),
                        ("primary", (100, "stroke-width:6;stroke:black;fill:none")),
                        ("secondary", (100, "stroke-width:4.5;stroke:black;fill:none")),
                        ("tertiary", (100, "stroke-width:3;stroke:black;fill:none")),
                        ("motorway", (100, "stroke-width:9;stroke:red;fill:none")),
                        (
                            "motorway_link",
                            (100, "stroke-width:4.5;stroke:red;fill:none"),
                        ),
                        ("footway", (100, "stroke:#002a5a;fill:none")),
                        ("track", (100, "stroke:#002a5a;fill:none")),
                        ("service", (100, "stroke:#002a5a;fill:none")),
                        ("cycleway", (100, "stroke:#002a5a;fill:none")),
                        ("unclassified", (100, "stroke:#002a5a;fill:none")),
                    ]),
                    (11, "stroke:#030038;fill:none"),
                ),
            ),
            (
                "building",
                (
                    HashMap::from([
                        ("house", (20, "stroke:blue; fill:purple")),
                        ("yes", (20, "stroke:blue; fill:#ffd62e")),
                        ("shed", (20, "stroke:blue; fill:#ffd62e")),
                        ("apartments", (20, "stroke:blue; fill:#ffd62e")),
                        ("church", (20, "stroke:blue; fill:#fb6bff")),
                        ("school", (20, "stroke:blue; fill:#fb6bff")),
                        ("commercial", (20, "stroke:blue; fill:purple")),
                        ("retail", (20, "stroke:blue; fill:purple")),
                        ("construction", (20, "stroke:blue; fill:none")),
                    ]),
                    (21, "stroke:blue; fill:#ffd020"),
                ),
            ),
            (
                "landuse",
                (
                    HashMap::from([
                        ("forest", (5, "stroke:#009e07; fill:#169400")),
                        ("grass", (5, "stroke:#009e07; fill:#6bff88")),
                        ("residential", (5, "stroke:#009e07; fill:#e2ff16")),
                        ("education", (5, "stroke:#009e07; fill:#007f5f")),
                        ("farmland", (5, "stroke:#009e07; fill:#CD853F")),
                    ]),
                    (2, "stroke:#009e07; fill:#007f5f"),
                ),
            ),
            (
                "natural",
                (
                    HashMap::from([
                        ("shrubbery", (50, "stroke:none; fill:green")),
                        ("tree_row", (50, "stroke:green; fill:none")),
                        ("water", (50, "stroke:Aqua; fill:RoyalBlue")),
                    ]),
                    (50, "stroke:#009e07; fill:#007f5f"),
                ),
            ),
            (
                "barrier",
                (
                    HashMap::from([
                        ("fence", (50, "stroke:red; fill:none")),
                        ("wall", (50, "stroke:darkkhaki; fill:none")),
                        ("hedge", (50, "stroke:green; fill:none")),
                    ]),
                    (20, "stroke:red; fill:none"),
                ),
            ),
            (
                "leisure",
                (
                    HashMap::from([
                        (
                            "playground",
                            (50, "stroke:palegoldenrod; fill:palegoldenrod"),
                        ),
                        ("dog_park", (50, "stroke:brown; fill:yellowgreen")),
                        ("garden", (50, "stroke:greenyellow; fill:greenyellow")),
                        ("pitch", (50, "stroke:chocolate; fill:chocolate")),
                        ("swimming_pool", (50, "stroke:blue; fill:dodgerblue")),
                    ]),
                    (20, "stroke:brown; fill:none"),
                ),
            ),
            (
                "railway",
                (
                    HashMap::from([(
                        "narrow_gauge",
                        (50, "stroke:black;fill:none;stroke-width:2;stroke-miterlimit:4;stroke-dasharray:20, 20;stroke-dashoffset:0"),

                    ),
                        ("rail", (50, "stroke:black;fill:none;stroke-width:4;stroke-miterlimit:4;stroke-dasharray:10, 10;stroke-dashoffset:0")),
                        ("platform", (50, "stroke:Gray; fill:DarkGray")),
                    ]),
                    (20, "stroke:brown; fill:none"),
                ),
            ),
            (
                "amenity",
                (
                    HashMap::from([(
                        "parking",
                        (9, "stroke:LightSkyBlue; fill:url(#parking)"),

                    ),
                    ]),
                    (9, "stroke:LightSkyBlue; fill:LightSkyBlue"),
                ),
            ),
        ]))
    }

    pub fn get(&self, key: &str, value: &str) -> Option<(u32, &str)> {
        if let Some((map, default)) = self.0.get(key) {
            if let Some(&s) = map.get(value) {
                Some(s)
            } else {
                debug!("use default for {}:{}", key, value);
                Some(*default)
            }
        } else {
            None
        }
    }
}
