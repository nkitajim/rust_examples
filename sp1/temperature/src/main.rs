use clap::Parser;
use temperture::app::cli::Cli;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Unit {
	Celsius,
	Fahrenheit,
	Kelvin,
}

impl std::str::FromStr for Unit {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"celsius" | "c" => Ok(Unit::Celsius),
			"fahrenheit" | "f" => Ok(Unit::Fahrenheit),
			"kelvin" | "k" => Ok(Unit::Kelvin),
			_ => Err(format!("Unknown unit {s}")),
		}
	}
}

#[derive(Debug)]
struct Temperature {
	value: f64,
	unit: Unit,
}

impl Temperature {
    fn new(value: f64, unit: Unit) -> Self {
		Temperature {
			value,
			unit,
		}
	}
	fn as_kelvin(&self) -> f64 {
		match self.unit {
			Unit::Celsius => self.value - 273.15,
			Unit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0 + 273.15,
			Unit::Kelvin => self.value,
		}
	}
	fn from_kelvin(k: f64, unit: Unit) -> Self {
		match unit {
			Unit::Celsius => Temperature::new(k + 273.15 , unit),
			Unit::Fahrenheit => Temperature::new((k - 273.15) * 9.0 / 5.0 + 32.0, unit),
			Unit::Kelvin => Temperature::new(k,unit),
		}

	}
	fn to(&self, unit: Unit) -> Self {
		let k = self.as_kelvin();
		Temperature::from_kelvin(k, unit)
	}
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_str = match self.unit {
            Unit::Celsius => "°C",
            Unit::Fahrenheit => "°F",
            Unit::Kelvin => "K",
        };
        write!(f, "{:.2} {}", self.value, unit_str)
    }
}

fn main() {
	let cli = Cli::parse();

	let input_unit = cli.input_mode.parse::<Unit>().expect("invalid input unit");
    let output_unit = cli.output_mode.parse::<Unit>().expect("invalid output unit");

    let input_temp = Temperature::new(cli.temperture, input_unit);
    let output_temp = input_temp.to(output_unit);

    println!("{output_temp}");
}
