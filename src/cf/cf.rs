#![allow(dead_code)]

pub fn if_statememt() {
	let temp = 35;

	if temp > 30 {
		println!("hot");
	} else if temp < 10 {
		println!("cold");
	} else {
		println!("okay");
	}

	let day = if temp > 20 { "Sunny" } else { "cloudy" };
	println!("{}", day);

	println!(
		"it is {}",
		if temp > 30 {
			"hot"
		} else if temp < 10 {
			"cold"
		} else {
			"okay"
		}
	);

	println!(
		"the temp is {}",
		if temp > 20 {
			if temp > 30 {
				"very hot"
			} else {
				"hot"
			}
		} else if temp < 10 {
			"cold"
		} else {
			"okay"
		}
	);

	// LOOPS:
	while_and_loop();

	// MATCH:
	match_st();
}

fn while_and_loop() {
	let mut x = 1;

	// while loop
	while x < 1000 {
		x *= 2;

		// does not print 64
		if x == 64 {
			continue;
		}
		println!("x = {}", x);
	}

	let mut y = 1;

	// simple loop, equivalent of while true
	// a loop that never stops unless you explicitly break out of it
	loop {
		y *= 2;
		if y >= 10 {
			break;
		}
		println!("y = {}", y);
	}

	// for loop
	// values of z will go from 1 to 10
	for z in 1..11 {
		if z == 3 {
			continue;
		}
		if z == 8 {
			break;
		}
		println!("z = {}", z);
	}

	for (pos,val) in (30..41).enumerate() {
		println!("pos = {}, val = {}", pos, val);
	}
}

fn match_st() {

	// MATCH
	// like if statement but can check several cases at the same time
	let code = 44;
	let country = match code {
		44 => "UK",
		46 => "Sweden",
		1..=1000 => "unknown",
		_ => "inavlid"
	};

	println!("country = {}, code = {}", country, code);
}
