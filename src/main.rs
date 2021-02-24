use hash_code_2021::{solve, Problem, Solution};
use std::{fs::write, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Hash code 2021 CLI",
    about = "The CLI to interact with the solution for the 2021 HashCode competition."
)]
struct Options {
    /// Whether or not to run on an individual file.
    #[structopt(short, long)]
    individual_file: bool,

    /// Whether or not to print debug of the solution to stdout instead of creating output files.
    #[structopt(short, long)]
    to_stdout: bool,

    #[structopt(parse(from_os_str), required_if("individual_file", "true"))]
    input: Option<PathBuf>,

    /// Output directory to write to.
    #[structopt(parse(from_os_str), default_value = "output")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Options::from_args();
    if opt.individual_file {
        handle_file(opt.input.clone().unwrap(), &opt)?;
    } else {
        if !opt.output.is_dir() {
            Err("When not using individual_file option, the output should be a directory.")?;
        }

        // Get everything in the input folder
        for f in std::fs::read_dir("input")? {
            let entry = f?;
            handle_file(entry.path(), &opt)?;
        }
    }
    Ok(())
}

fn handle_file(p: PathBuf, opt: &Options) -> Result<(), Box<dyn std::error::Error>> {
    let problem = parse_problem_from_file(&p)?;
    let solution = solve(problem);
    let output_file_contents = solution_to_string(solution);

    if opt.to_stdout {
        println!("For file: {}", p.file_name().unwrap().to_str().unwrap());
        println!("{}", output_file_contents);
    } else {
        let mut out_filename = p.file_name().unwrap().to_owned();
        out_filename.push(".out");

        let mut out_path = opt.output.clone();
        out_path.push(out_filename);

        std::fs::write(out_path, output_file_contents).unwrap();
    }
    Ok(())
}

fn parse_problem_from_file<T: AsRef<std::path::Path>>(
    path: T,
) -> Result<Problem, Box<dyn std::error::Error>> {
    parse_problem(std::fs::read_to_string(path)?)
}

fn parse_problem(_input: String) -> Result<Problem, Box<dyn std::error::Error>> {
    todo!();
}

fn solution_to_string(_solution: Solution) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_example_statement() {
        let problem = parse_problem_from_file("input/testfile").unwrap();
        assert_eq!(problem, Problem {});
    }
}
