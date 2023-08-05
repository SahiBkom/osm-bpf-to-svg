mod node_index;
mod style;

use anyhow::Result;
use log::debug;
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

    pub fn append_line(&mut self, prio: u32, svg_line: String) {
        if let Some(e) = self.0.get_mut(&prio) {
            e.push(svg_line);
        } else {
            self.0.insert(prio, vec![svg_line]);
        }
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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SelectBox {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl SelectBox {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> SelectBox {
        SelectBox { x, y, w, h }
    }

    pub fn x_min(&self) -> u32 {
        self.x
    }

    pub fn y_min(&self) -> u32 {
        self.y
    }

    pub fn x_max(&self) -> u32 {
        self.x + self.w
    }

    pub fn y_max(&self) -> u32 {
        self.y + self.h
    }

    /// width
    pub fn w(&self) -> u32 {
        self.w
    }

    /// height
    pub fn h(&self) -> u32 {
        self.h
    }

    /// return true if the coordinate is inside the box
    pub fn is_inside(&self, x: u32, y: u32) -> bool {
        x > self.x_min() && x < self.x_max() && y > self.y_min() && y < self.y_max()
    }
}

pub fn doit(
    select_box: SelectBox,
    input_path: String,
    output_path: String,
    _style_path: Option<String>,
) -> Result<()> {
    let reader = ElementReader::from_path(&input_path)?;

    let node_index =
        reader.par_map_reduce(NodeIndex::convert, NodeIndex::new, NodeIndex::combine)?;

    let node_index_select = node_index.filter(&select_box);

    let reader = ElementReader::from_path(&input_path)?;

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
                        debug!(
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

    // grid

    for x in (select_box.x_min()..select_box.x_max()).step_by(1000) {
        svg.append_line(
            1000,
            format!(
                r#"<line x1="{}" y1="-{}" x2="{}" y2="-{}" stroke="black" />"#,
                x,
                select_box.y_min(),
                x,
                select_box.y_max()
            ),
        );
    }

    for y in (select_box.y_min()..select_box.y_max()).step_by(1000) {
        svg.append_line(
            1000,
            format!(
                r#"<line x1="{}" y1="-{}" x2="{}" y2="-{}" stroke="black" />"#,
                select_box.x_min(),
                y,
                select_box.x_max(),
                y
            ),
        );
    }

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

    let mut output = std::fs::File::create(output_path)?;

    write!(
        output,
        "<svg
  width=\"{}\" 
  height=\"{}\" 
  viewBox=\"{} -{} {} {}\" 
  xmlns=\"http://www.w3.org/2000/svg\" 
  xmlns:xlink=\"http://www.w3.org/1999/xlink\" 
>
{}
{} 
</svg>\n",
        select_box.w,
        select_box.h,
        select_box.x_min(),
        select_box.y_max(),
        select_box.w,
        select_box.h,
        style::defs_pattern(),
        svg
    )?;
    Ok(())
}
