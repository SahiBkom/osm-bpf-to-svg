use anyhow::Result;

fn main() -> Result<()> {
    osm_bpf_to_svg::doit(170_000, 180_000, 440_000, 450_000)?;

    Ok(())
}
