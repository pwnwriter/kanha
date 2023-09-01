use crate::interface::TakeoverArgs;

pub async fn subdomain_takeover(takeover_args: TakeoverArgs) -> Result<(), Box<dyn std::error::Error>> {

    println!("{}", takeover_args.filename);

    Ok(())
}

