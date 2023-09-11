use crate::interface::DencodeArgs;
pub async fn decode_urls(dencode_args: DencodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world from wordlist");
    if let Some(filename) = dencode_args.decode {
        println!("filename is {}", filename);
    }

    Ok(())
}
