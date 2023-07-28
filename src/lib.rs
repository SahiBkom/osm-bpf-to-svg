mod node_index;
mod style;

use anyhow::Result;
use node_index::NodeIndex;
use osmpbf::{Element, ElementReader};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Write;
use style::Style;

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

pub fn doit(x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Result<()> {
    let reader = ElementReader::from_path("gelderland-latest.osm.pbf")?;

    let node_index =
        reader.par_map_reduce(NodeIndex::convert, NodeIndex::new, NodeIndex::combine)?;

    let node_index_select = node_index.filter(x_min, x_max, y_min, y_max);

    let reader = ElementReader::from_path("gelderland-latest.osm.pbf")?;

    let style = Style::new();

    // Count the ways
    let mut svg = reader.par_map_reduce(
        |element| match element {
            Element::Way(way) => {
                if way.refs().any(|id| node_index_select.contains_key(id)) {
                    let mut tags: String = way
                        .tags()
                        .map(|(key, value)| format!("{}:{}; ", key, value))
                        .collect();

                    tags = tags.replace('&', "&amp;");

                    let mut svg = Svg::new();

                    for (key, value) in way.tags() {
                        if let Some((prio, style)) = style.get(key, value) {
                            svg = Svg::one(
                                prio,
                                format!(
                                    "<path d=\"{}\" id=\"{}\" style=\"{}\"><desc>{}</desc></path>",
                                    node_index.svg_path_d(way.refs()),
                                    way.id(),
                                    style,
                                    tags
                                ),
                            )
                        }
                    }

                    if svg.0.is_empty() {
                        println!(
                            "Missing id:{} {} nodes:{}",
                            way.id(),
                            tags,
                            way.refs().len()
                        );
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
        Svg::new,
        Svg::combine,
    )?;

    // consuming to much resources
    //   svg = svg.combine(Svg::one(
    //       1,
    //       r##"<defs>
    //   <pattern id="star" viewBox="0,0,10,10" width="10%" height="10%">
    //     <polygon points="0,0 2,5 0,10 5,8 10,10 8,5 10,0 5,2"/>
    //   </pattern>
    //   <pattern id="parking" patternUnits="userSpaceOnUse" width="10" height="6">
    //     <rect width="10" height="6" fill="white"/>
    //     <rect x="0" y="0" width="9" height="5" fill="LightSkyBlue"/>
    //   </pattern>
    // </defs>"##
    //           .to_string(),
    //   ));

    let mut output = std::fs::File::create("test.svg")?;
    let width = x_max - x_min;
    let height = y_max - y_min;
    write!(
        output,
        "<svg 
  width=\"{width}\" 
  height=\"{height}\" 
  viewBox=\"{x_min} -{y_max} {width} {height}\" 
  xmlns=\"http://www.w3.org/2000/svg\" 
  xmlns:xlink=\"http://www.w3.org/1999/xlink\" 
>
{svg} 
</svg>\n"
    )?;
    Ok(())
}
