use crate::interface::TakeoverArgs;

pub async fn subdomain_takeover(
    takeover_args: TakeoverArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(filename) = takeover_args.filename {
        println!("Filename is {}", filename);
    }

    Ok(())
}
