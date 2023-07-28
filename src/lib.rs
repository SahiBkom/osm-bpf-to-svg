use anyhow::Result;
use osmpbf::{Element, ElementReader, WayRefIter};
use rijksdriehoek::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Write;

struct Style<'a>(HashMap<&'a str, (HashMap<&'a str, (u32, &'a str)>, (u32, &'a str))>);

impl Style<'_> {
    pub fn new() -> Style<'static> {
        Style(HashMap::from([
            (
                "highway",
                (
                    HashMap::from([
                        ("path", (10, "stroke:#002a5a;fill:none")),
                        ("residential", (10, "stroke-width:3;stroke:black;fill:none")),
                        ("footway", (10, "stroke:#002a5a;fill:none")),
                        ("track", (10, "stroke:#002a5a;fill:none")),
                        ("service", (10, "stroke:#002a5a;fill:none")),
                        ("cycleway", (10, "stroke:#002a5a;fill:none")),
                        ("unclassified", (10, "stroke:#002a5a;fill:none")),
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
                    ]),
                    (20, "stroke:red; fill:none"),
                ),
            ),
        ]))
    }

    pub fn get(&self, key: &str, value: &str) -> Option<(u32, &str)> {
        if let Some((map, default)) = self.0.get(key) {
            if let Some(&s) = map.get(value) {
                Some(s)
            } else {
                println!("Default for {}:{}", key, value);
                Some(*default)
            }
        } else {
            None
        }
    }
}

struct NodeIndex(HashMap<i64, (u32, u32)>);

impl NodeIndex {
    pub fn new() -> NodeIndex {
        NodeIndex(HashMap::new())
    }

    pub fn one(id: i64, x: u32, y: u32) -> NodeIndex {
        NodeIndex(HashMap::from([(id, (x, y))]))
    }

    pub fn combine(mut self, ni: NodeIndex) -> NodeIndex {
        self.0.extend(ni.0);
        self
    }

    pub fn filter(&self, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> NodeIndex {
        let mut r = NodeIndex::new();

        for (&id, &(x, y)) in &self.0 {
            if x > x_min && x < x_max && y > y_min && y < y_max {
                r.0.insert(id, (x, y));
            }
        }

        r
    }

    /// svg path definition
    pub fn svg_path_d(&self, line: WayRefIter) -> String {
        let mut d = String::new();
        for (i, r) in line.enumerate() {
            let (x, y) = self.0.get(&r).copied().unwrap_or_default();
            if i == 0 {
                d += &format!("M {} -{} ", x, y);
            } else {
                d += &format!("L {} -{} ", x, y);
            }
        }

        d
    }
}

fn convert(e: Element) -> NodeIndex {
    match e {
        Element::DenseNode(n) => {
            let (x, y) = wgs84_to_rijksdriehoek(n.lat(), n.lon());

            NodeIndex::one(n.id, x as u32, y as u32)
        }
        _ => NodeIndex::new(),
    }
}

struct Svg(HashMap<u32, Vec<String>>);

impl Svg {
    pub fn new() -> Svg {
        Svg(HashMap::new())
    }

    pub fn one(prio: u32, svg_line: String) -> Svg {
        Svg(HashMap::from([(prio, vec![svg_line])]))
    }

    pub fn combine(mut self, other: Svg) -> Svg {
        for (prio, svg_lines) in other.0 {
            if let Some(e) = self.0.get_mut(&prio) {
                e.extend(svg_lines);
            } else {
                self.0.insert(prio, svg_lines);
            }
        }
        self
    }
}

impl Display for Svg {
    fn fmt<'a>(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sort: Vec<(&u32, &Vec<String>)> = self.0.iter().collect();

        sort.sort_by_key(|&(&prio, _lines)| prio);

        for (_, lines) in sort {
            for line in lines {
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

pub fn doit() -> Result<()> {
    // tests/test.osm.pbf
    // gelderland-latest.osm.pbf
    // andorra-latest.osm.pbf
    let reader = ElementReader::from_path("gelderland-latest.osm.pbf")?;

    let t = reader.par_map_reduce(convert, NodeIndex::new, |a, b| a.combine(b))?;

    println!("{}", t.0.len());

    // let mut x_min = u32::MAX;
    // let mut x_max = u32::MIN;
    // let mut y_min = u32::MAX;
    // let mut y_max = u32::MIN;
    // for (id, (x, y)) in &t.0 {
    //     x_min = x_min.min(x);
    //     x_max = x_max.max(x);
    //     y_min = y_min.min(y);
    //     y_max = y_max.max(y);
    // }

    // println!("{} {} | {} {}", x_min, x_max, y_min, y_max);

    let vakje = t.filter(175_000, 176_000, 446_000, 447_000);

    let mut x_min = u32::MAX;
    let mut x_max = u32::MIN;
    let mut y_min = u32::MAX;
    let mut y_max = u32::MIN;
    for (_id, &(x, y)) in &vakje.0 {
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    println!(
        "{} {} | {} {} | {}",
        x_min,
        x_max,
        y_min,
        y_max,
        vakje.0.len()
    );

    let reader = ElementReader::from_path("gelderland-latest.osm.pbf")?;

    let style = Style::new();

    // Count the ways
    let svg = reader.par_map_reduce(
        |element| match element {
            Element::Way(w) => {
                if w.refs().any(|id| vakje.0.contains_key(&id)) {
                    let tags: String = w
                        .tags()
                        .into_iter()
                        .map(|(key, value)| format!("{}:{}; ", key, value))
                        .collect();

                    let mut svg = Svg::new();

                    for (key, value) in w.tags() {
                        if let Some((prio, style)) = style.get(key, value) {
                            svg = Svg::one(
                                prio,
                                format!(
                                    "<path d=\"{}\" id=\"{}\" style=\"{}\"><desc>{}</desc></path>",
                                    t.svg_path_d(w.refs()),
                                    w.id(),
                                    style,
                                    tags
                                ),
                            )
                        }
                    }

                    if svg.0.is_empty() {
                        println!("Missing id:{} {} nodes:{}", w.id(), tags, w.refs().len());
                    }

                    svg
                } else {
                    Svg::new()
                }
            }

            Element::Node(_n) => Svg::new(),
            Element::DenseNode(_n) => Svg::new(),

            _ => Svg::new(),
        },
        Svg::new,     // Zero is the identity value for addition
        Svg::combine, // Sum the partial results
    )?;

    let mut output = std::fs::File::create("test.svg")?;
    write!(output, "<svg width=\"1000\" height=\"1000\" viewBox=\"175000 -447000 1000 1000\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" transform=\"matrix(1,0,0,-1,0,0)\">\n{svg}</svg>")?;

    Ok(())
}

// all DenseNode van een weg printen in een vak van 1000 x1000
