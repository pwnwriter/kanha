use crate::{commands::kanha_helpers::read_lines, interface::DencodeArgs, log::abort};
use urlencoding::{decode, encode};

pub async fn dencode_urls(dencode_args: DencodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    match (dencode_args.encode.as_ref(), dencode_args.decode.as_ref()) {
        (Some(filename), None) => {
            let urls = read_lines(filename).await?;
            for line_result in urls {
                let url = line_result?;
                let encoded_url = encode(&url);
                println!("{}", encoded_url);
            }
        }
        (None, Some(filename)) => {
            let urls = read_lines(filename).await?;
            for line_result in urls {
                let url = line_result?;
                let decoded_url = decode(&url);
                println!(
                    "{}",
                    decoded_url.unwrap_or_else(|err| {
                        println!("error decoding URL: {:?}", err);
                        abort("something went wrong while decoding URLs");
                    })
                );
            }
        }
        (Some(_), Some(_)) => {
            println!("Both encode and decode options provided. Please provide only one.");
        }
        (None, None) => {
            println!("No encode or decode option provided.");
        }
    }

    Ok(())
}
