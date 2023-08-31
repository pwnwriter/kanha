use crate::commands::status::statuscode::read_lines;
use crate::interface::FuzzerArgs;

pub async fn fuzz_url(fuzzer_args: FuzzerArgs) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(lines) = read_lines(&fuzzer_args.filename).await {
        let urls: Vec<String> = lines
            .map_while(Result::ok) // Filter out lines with read errors
            .collect();
        println!("{:?}", urls);
    }

    Ok(())
}
