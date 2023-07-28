use anyhow::Result;

fn main() -> Result<()> {
    osm_bpf_to_svg::doit()?;

    Ok(())
}
