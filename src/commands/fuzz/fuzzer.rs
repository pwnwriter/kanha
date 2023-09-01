use crate::{
    commands::{
        kanha_helpers::read_lines,
        statuscode::{fetch_and_print_status_codes, ArgsWithTasks},
    },
    interface::FuzzerArgs,
    log::abort,
};

impl ArgsWithTasks for FuzzerArgs {
    fn tasks(&self) -> usize {
        self.tasks
    }
}

pub async fn fuzz_url(fuzzer_args: FuzzerArgs) -> Result<(), Box<dyn std::error::Error>> {
    let url_to_fuzz = &fuzzer_args.url;

    let lines = read_lines(&fuzzer_args.wordlist).await?;

    let mut formatted_urls = Vec::new(); // Create an empty vector

    if !url_to_fuzz.contains("FUZZ") {
        abort("The URL must contain the 'FUZZ' keyword for fuzzing.")
    }

    for line in lines.map_while(Result::ok) {
        let replaced_url = url_to_fuzz.replace("FUZZ", &line);
        formatted_urls.push(replaced_url); // Add the modified URL to the vector
    }

    fetch_and_print_status_codes(formatted_urls, fuzzer_args).await;

    Ok(())
}
