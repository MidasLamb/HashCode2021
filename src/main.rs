use hash_code_2021::*;
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap};
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

fn parse_problem(input: String) -> Result<Problem, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>().clone();

    let first_line: Vec<u32> = lines[0]
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let amount_of_seconds = first_line[0];
    let number_intersections = first_line[1];
    let number_streets = first_line[2];
    let number_cars = first_line[3];
    let bonus_points = first_line[4];

    let mut streets = HashMap::<&str, Arc<RefCell<Street>>>::new();
    let mut intersections = HashMap::<u32, Intersection>::new();
    let mut cars = HashMap::new();

    for i in 1..(number_streets + 1) as usize {
        let line = lines[i].split(' ').collect::<Vec<_>>();
        let intersection_start: u32 = line[0].parse().unwrap();
        let intersection_end: u32 = line[1].parse().unwrap();
        let street_name = line[2];
        let time_to_travel = line[3].parse().unwrap();

        // store street
        let street = Arc::new(RefCell::new(Street {
            name: street_name.to_owned(),
            cars: Vec::new(),
            time_to_travel: time_to_travel,
        }));
        streets.insert(street_name, street.clone());

        // update intersection
        if let Some(x) = intersections.get_mut(&intersection_start) {
            x.outgoing_streets.push(street.clone());
        } else {
            intersections.insert(
                intersection_start,
                Intersection {
                    id: intersection_start,
                    incoming_streets: Vec::new(),
                    outgoing_streets: vec![street.clone()],
                },
            );
        }

        if let Some(x) = intersections.get_mut(&intersection_end) {
            x.incoming_streets.push(street.clone());
        } else {
            intersections.insert(
                intersection_end,
                Intersection {
                    id: intersection_end,
                    incoming_streets: vec![street.clone()],
                    outgoing_streets: Vec::new(),
                },
            );
        }
    }

    // Parse cars
    for i in (number_streets + 1) as usize..(number_streets + 1 + number_cars) as usize {
        let line = lines[i].split(' ').collect::<Vec<_>>();
        let car_id: u32 = line[0].parse().unwrap();

        let mut route = Vec::new();

        for j in 1..line.len() {
            route.push(streets.get(line[j]).unwrap().clone());
        }

        let car = Arc::new(RefCell::new(Car {
            id: car_id,
            path_to_take: route,
            destination_reached_at: None,
        }));

        cars.insert(car_id, car.clone());
        streets
            .get(line[1])
            .unwrap()
            .clone()
            .borrow_mut()
            .cars
            .push((car.clone(), 0));
    }

    Ok(Problem {
        amount_of_seconds: amount_of_seconds,
        bonus_points: bonus_points,
        cars: cars.values().map(|x| x.clone()).collect(),
        streets: streets.values().map(|x| x.clone()).collect(),
        intersections: intersections.drain().map(|(_, x)| x).collect(),
    })
}

fn solution_to_string(solution: Solution) -> String {
    let mut lines = Vec::<String>::new();
    lines.push(solution.traffic_lights.len().to_string());

    lines.append(
        &mut solution
            .traffic_lights
            .into_iter()
            .map(|(u, v)| {
                let temp = v
                    .iter()
                    .map(|a| format!("{} {}", a.0, a.1))
                    .collect::<Vec<_>>();
                let mut r: Vec<String> = vec![u.to_string(), v.len().to_string()];
                r.extend_from_slice(&temp[..]);
                r
            })
            .flatten()
            .collect::<Vec<_>>(),
    );

    lines.join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_example_statement() {
        let problem = parse_problem_from_file("input/testfile").unwrap();
        //assert_eq!(problem, Problem {});
    }
}
