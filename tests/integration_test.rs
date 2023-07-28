use anyhow::Result;
use osm_bpf_to_svg::*;

#[test]
fn test_10_by_10_km() -> Result<()> {
    osm_bpf_to_svg::doit(170_000, 180_000, 440_000, 450_000)?;

    Ok(())
}
