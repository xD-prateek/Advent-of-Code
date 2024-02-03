use std::{cell::RefCell, collections::{HashMap, VecDeque}};

pub struct Map<'a>(HashMap<&'a str, Module<'a>>);

impl<'a> Map<'a> {
	pub fn new_from_str(content: &'a str) -> Self {
		let modules = content.lines().fold(HashMap::new(), |mut acc, line| {
			let (type_and_name, destination) = line.split_once(" -> ").unwrap_or_else(|| panic!("invalid input format of line: {line}"));

			let (switch_type, name) = if let Some(name) = type_and_name.strip_prefix('%') {
				(Switch::get_default_flipflop(), name)
			}
			else if let Some(name) = type_and_name.strip_prefix('&') {
				(Switch::get_default_conjugation(), name)
			}
			else if type_and_name == "broadcaster" {
				(Switch::get_default_broadcaster(), type_and_name)
			}
			else {
				panic!("invalid switch type found: {type_and_name}")
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

    pub fn get_cycles_till_final_machine(&self, final_machine: &str) -> usize {
    	// the module feeding into final machine is a conjunction module
    	let feeding_conjugation_machine = self.0.iter().find_map(|(&k, v)| {
    		match v.destination.contains(&final_machine) {
    			true => Some(k),
    			false => None,
    		}
    	}).unwrap();

    	let mut query_machines = self.0.iter().filter_map(|(&k, v)| {
    		match v.destination.contains(&feeding_conjugation_machine) {
    			true => Some((k, 0)),
    			false => None,
    		}
    	}).collect::<HashMap<&str, usize>>();

    	let mut cycles = 1;

    	while !query_machines.values().all(|&v| v > 0) {
    		self.get_cycle_count_for_low_pulse(&mut query_machines, cycles);
    		cycles += 1;
    	}

    	query_machines.into_values().reduce(|acc, v| acc * v / Self::gcd(acc, v)).unwrap_or_else(|| panic!("error getting LCM."))
    }

    fn gcd(num1: usize, num2: usize) -> usize {
    	match num2 == 0 {
    		true => num1,
    		false => Self::gcd(num2, num1 % num2),
    	}
    }

    fn get_cycle_count_for_low_pulse(&self, query_machine: &mut HashMap<&'a str, usize>, cycles: usize) -> bool {
    	let mut q = VecDeque::from([ Event::new_from_values("button", &Pulse::Low, "broadcaster") ]);

    	while let Some(event) = q.pop_front() {

    		query_machine.entry(event.source).and_modify(|v| {
    			if let Pulse::High = event.pulse {
    				*v = cycles;
    			}
    		});

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
    	false
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
		matches!((self, other), (Self::High, Self::High) | (Self::Low, Self::Low))     
	}    
}
