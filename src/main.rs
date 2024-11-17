use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    process::ExitCode,
};

fn main() {
    let user_input = parse_input();
    if let Some((year, day)) = user_input {
        generate_files(year, day);
        add_bin_configuration(year, day);
    }
}
fn add_bin_configuration(year: usize, day: usize) {
    let file_path = "Cargo.toml";
    let yr = year - 2000;
    let directory = &format!("./{}/{}/", yr, day);
    let bin1 = format!(
        "\n[[bin]]\nname=\"{}_{}_1\"\npath=\"{}part1.rs\"\n",
        yr, day, directory
    );
    let bin2 = format!(
        "\n[[bin]]\nname=\"{}_{}_2\"\npath=\"{}part2.rs\"\n",
        yr, day, directory
    );
    let result = format!("{}\n{}", bin1, bin2);
    let mut file = match OpenOptions::new().append(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening Cargo.toml: {}", e);
            ExitCode::from(1);
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", result) {
        eprintln!("Error editing Cargo.toml: {}", e);
        ExitCode::from(1);
    } else {
        println!("Cargo.toml edited.");
    }
}

fn create_dir(path: &str) -> Result<(), io::Error> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}

fn create_file(path: &str, contents: &str) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
fn generate_files(year: usize, day: usize) {
    let rust_file_template = include_str!("day.rs.template");
    let yr = year - 2000;
    let directory = &format!("./{}/{}/", yr, day);
    if let Err(res) = create_dir(directory) {
        eprintln!(
            "Could not create directory: {}\n{}",
            directory,
            res.to_string()
        );
        ExitCode::from(1);
    }
    println!("Created directory: {}", directory);
    let part1_path = &format!("{}part1.rs", directory);
    if let Err(e) = create_file(part1_path, rust_file_template) {
        eprintln!("Could not create {}\n{}", part1_path, e.to_string());
        ExitCode::from(1);
    }
    println!("Generated part1.rs");
    let part2_path = &format!("{}part2.rs", directory);

    if let Err(e) = create_file(part2_path, rust_file_template) {
        eprintln!("Could not create {}\n{}", part2_path, e.to_string());
        ExitCode::from(1);
    }
    println!("Generated part2.rs");

    let sample_path = &format!("{}sample.txt", directory);

    if let Err(e) = create_file(sample_path, "") {
        eprintln!("Could not create {}\n{}", sample_path, e.to_string());
        ExitCode::from(1);
    }
    println!("Created empty sample file (Please fill in the corresponding sample text");

    let input_path = &format!("{}input.txt", directory);
    if let Err(e) = create_file(input_path, "") {
        eprintln!("Could not create {}\n{}", input_path, e.to_string());
        ExitCode::from(1);
    }
    println!("Created empty input file (Please fill in the corresponding input text");
    println!("\nAll files succesfully created");
}

fn parse_input() -> Option<(usize, usize)> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Argument error, usage: `./bin {{year}}.{{day}}");
        return None;
    }
    if args[1] == "-h" || args[1] == "help" {
        println!("Usage: ./bin {{year}}.{{day}}\nSupposed to be called from the root directory of a cargo project.");
        return None;
    }
    let input: Vec<&str> = args[1].trim().split('.').collect();
    if input.len() != 2 {
        eprintln!("Argument error, usage: `./bin {{year}}.{{day}}");
        return None;
    }
    let year_opt = input[0].parse::<usize>();
    let day_opt = input[1].parse::<usize>();
    if year_opt.is_err() {
        eprintln!("Provided year: {} is not a positive integer", input[0]);
        return None;
    }
    if day_opt.is_err() {
        eprintln!("Provided day: {} is not a positive integer", input[1]);
        return None;
    }
    let mut year = year_opt.unwrap();
    if year < 100 {
        year += 2000;
    }
    Some((year, day_opt.unwrap()))
}
