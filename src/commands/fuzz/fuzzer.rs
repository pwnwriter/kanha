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

    if !url_to_fuzz.contains("FUZZ") {
        abort("The URL must contain the 'FUZZ' keyword for fuzzing.")
    }

    // https://www.reddit.com/r/learnrust/comments/pcwp31/comment/ham0vpa/?utm_source=share&utm_medium=web2x&context=3
    let formatted_urls: Vec<String> = lines
        .map(Result::ok)
        .filter_map(|line_result| line_result.map(|line| url_to_fuzz.replace("FUZZ", &line)))
        .collect();

    fetch_and_print_status_codes(formatted_urls, fuzzer_args).await;

    Ok(())
}
