use crate::interface::FuzzerArgs;

pub async fn fuzz_url(fuzzer_args: FuzzerArgs) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(lines) = crate::commands::kanha_helpers::read_lines(&fuzzer_args.filename).await {
        let urls: Vec<String> = lines
            .map_while(Result::ok) // Filter out lines with read errors
            .collect();
        println!("{:?}", urls);

    }

    Ok(())
}
