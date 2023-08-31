use crate::interface::FuzzerArgs;

pub async fn fuzz_url(fuzzer_args: FuzzerArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from fuzer");
    println!("FuzzerArgs filename: {}", fuzzer_args.filename);
    // Add your fuzzing logic here using fuzzer_args.filename
    Ok(())
}
