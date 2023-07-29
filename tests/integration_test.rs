use anyhow::Result;
use osm_bpf_to_svg::*;

#[test]
fn test_10_by_10_km() -> Result<()> {
    let select_box = SelectBox::new(170_000, 180_000, 1_000, 1_000);
    osm_bpf_to_svg::doit(
        select_box,
        "gelderland-latest.osm.pbf".to_string(),
        "out.svg".to_string(),
        None,
    )?;

    Ok(())
}
