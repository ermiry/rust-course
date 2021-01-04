use std::io;

fn main() {
	let mut input = String::new();

	println!("Enter your weight (kg): ");
	io::stdin().read_line(&mut input).unwrap();

	let input_weight: f32 = sanitize_input(&input);

	let mut mars_weight: f32 = cal_weight_on_mars(input_weight);
	mars_weight *= 1000.0;
	println!("Weight on mars: {}g", mars_weight);

	drop(input);
}

fn sanitize_input(s: &String) -> f32 {
	let s_input = s.trim();
	return s_input.parse().unwrap_or_default();
}

fn cal_weight_on_mars(weight: f32) -> f32 {
	return (weight / 9.81) * 3.711;
}