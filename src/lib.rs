use std::{cell::RefCell, collections::{HashMap, VecDeque}};

pub struct Map<'a>(HashMap<&'a str, Module<'a>>);

impl<'a> Map<'a> {
	pub fn new_from_str(content: &'a str) -> Self {
		let modules = content.lines().fold(HashMap::new(), |mut acc, line| {
			let (type_and_name, destination) = line.split_once(" -> ").unwrap_or_else(|| panic!("invalid input format of line: {line}"));
			let (switch_type, name) = match type_and_name.chars().next().unwrap_or_else(|| panic!("error processing line: {line}")) {
				'%' => (Switch::get_default_flipflop(), &type_and_name[1..]),
				'&' => (Switch::get_default_conjugation(), &type_and_name[1..]),
				_ if type_and_name == "broadcaster" => (Switch::get_default_broadcaster(), type_and_name),
				_ => panic!("invalid switch type found: {type_and_name}"),
			};
			acc.insert(name, Module::new_from_values(switch_type, destination.split(", ").collect::<Vec<&str>>()));
			acc
		});

        // modify map
        modules.iter().for_each(|(&k, v)| {
        	v.destination.iter().for_each(|&des| {
        		if let Some(m) = &modules.get(des) {
        			if let Switch::Conjugation(cell) = &m.switch {
        				cell.borrow_mut().push((k, Pulse::Low));
        			}
        		}  
        	});
        });

        Self(modules)
    }

    pub fn get_pulses_for_cycles(&self, cycles: usize) -> usize {
    	let sum = (0..cycles).into_iter().fold((0, 0), |acc, _| {
    		let (lo, hi) = self.apply_one_cycle();
    		(acc.0 + lo, acc.1 + hi)
    	});
    	sum.0 * sum.1
    }

    fn apply_one_cycle(&self) -> (usize, usize) {
    	let mut q = VecDeque::from([ Event::new_from_values("button", &Pulse::Low, "broadcaster") ]);

    	let mut total = (0, 0);
    	while let Some(event) = q.pop_front() {
    		match event.pulse {
    			Pulse::High => total.1 += 1,
    			Pulse::Low => total.0 += 1,
    		};

    		if let Some(next_module) = self.0.get(event.destination) {
    			match &next_module.switch {
    				Switch::FlipFlop(memory) => {
    					if let Pulse::Low = event.pulse {
    						let (new_state, new_pulse) = match *memory.borrow() {
    							State::On => (State::Off, Pulse::Low),
    							State::Off => (State::On, Pulse::High),
    						};
    						memory.replace(new_state);
    						next_module.destination.iter().for_each(|&des| {
    							q.push_back(Event::new_from_values(event.destination, &new_pulse, des));
    						});
    					}
    				},
    				Switch::Conjugation(memory) => {
    					memory.borrow_mut().iter_mut().find(|(name, _)| name == &event.source).unwrap().1 = event.pulse;
    					let new_pulse = match memory.borrow().iter().all(|(_, pulse)| pulse == &Pulse::High) {
    						true => Pulse::Low,
    						false => Pulse::High,
    					};
    					next_module.destination.iter().for_each(|&des| {
    						q.push_back(Event::new_from_values(event.destination, &new_pulse, des));
    					});
    				},
    				Switch::Broadcaster => next_module.destination.iter().for_each(|&des| {
    					q.push_back(Event::new_from_values(event.destination, &event.pulse, des));
    				}),
    			};
    		}
    	}
    	total
    }
}

struct Event<'a> {
	source: &'a str,
	pulse: Pulse,
	destination: &'a str,
}

impl<'a> Event<'a> {
	fn new_from_values(source: &'a str, pulse: &Pulse, destination: &'a str) -> Self {
		Self {
			source,
			destination,
			pulse: pulse.clone(),
		}
	}
}

struct Module<'a> {
	switch: Switch<'a>,
	destination: Vec<&'a str>,
}

impl<'a> Module<'a> {
	fn new_from_values(switch: Switch<'a>, destination: Vec<&'a str>) -> Self {
		Self {
			switch,
			destination,
		}
	}
}

enum Switch<'a> {
	FlipFlop(RefCell<State>),
	Conjugation(RefCell<Vec<(&'a str, Pulse)>>),
	Broadcaster
}

impl<'a> Switch<'a> {
	fn get_default_flipflop() -> Self {
		Self::FlipFlop(RefCell::new(State::Off))
	}

	fn get_default_conjugation() -> Self {
		Self::Conjugation(RefCell::new(Vec::new()))
	}

	fn get_default_broadcaster() -> Self {
		Self::Broadcaster
	}
}

enum State {
	On,
	Off,
}

#[derive(Clone)]
enum Pulse {
	High,
	Low,
}

impl PartialEq for Pulse {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::High, Self::High) => true,
			(Self::Low, Self::Low) => true,
			_ => false,
		}     
	}    
}