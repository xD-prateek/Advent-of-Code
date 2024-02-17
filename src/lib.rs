use std::{cell::{Cell, RefCell}, collections::HashMap};

pub struct Graph<'a>{
	graph: HashMap<&'a str, HashMap<&'a str, Cell<usize>>>,
	parent_list: RefCell<HashMap<&'a str, &'a str>>,
}

impl<'a> Graph<'a> {
	pub fn new_from_str(content: &'a str) -> Self {
		Self {
			graph: content.lines().fold(HashMap::new(), |mut acc, line| {
				let (k, v) = line.split_once(": ").unwrap_or_else(|| panic!("error getting list for {line}"));
				v.split_whitespace().for_each(|v| {
					[(k, v), (v, k)].into_iter().for_each(|(a, b)| { 
						acc.entry(a).and_modify(|list| { 
							list.insert(b, Cell::new(1));
						}).or_insert(HashMap::from([(b, Cell::new(1))])); 
					});
				});
				acc
			}),
			parent_list: RefCell::new(HashMap::default()),
		}
	}

	fn bfs(&self, s: &'a str, t: &'a str) -> bool {
		self.reset_parent();
		let mut q = vec!{ s };
		while let Some(n) = q.pop() {
			self.graph.get(n).unwrap_or_else(|| panic!("parent {n} not found.")).iter().for_each(|(c, v)| {
				if v.get() != 0 && !self.parent_list.borrow().contains_key(c) {
					self.parent_list.borrow_mut().insert(c, n);
					q.push(c);
				}
			});
		}

		self.parent_list.borrow().contains_key(t)
	}

	fn min_cut(&self, s: &'a str, t: &'a str) -> usize {
		self.reset_graph();
		let mut max_flow = usize::default();
		while self.bfs(s, t) {
			let mut flow = usize::MAX;
			let mut n = t;
			let parent_list = self.parent_list.borrow();
			while n != s {
				let parent = parent_list.get(n).unwrap_or_else(|| panic!("parent not found for {n} in list s: {s}, t: {t}."));
				flow = flow.min(self.graph.get(parent).unwrap_or_else(|| panic!("error getting node {parent} from graph.")).get(n).unwrap_or_else(|| panic!("child {n} doesn't exist for parent {parent}.")).get());
				n = parent;
			}

			max_flow += flow;

			n = t;
			while n != s {
				let parent = parent_list.get(n).unwrap_or_else(|| panic!("something went wrong. Parent not found."));
				self.add_value(parent, n, -1);
				self.add_value(n, parent, 1);
				n = parent;
			}
		}
		max_flow
	}

	fn add_value(&self, parent: &str, child: &str, value: isize) {
		let val_cell = self.graph.get(parent).unwrap_or_else(|| panic!("eror getting node {parent} from graph.")).get(child).unwrap_or_else(|| panic!("child {child} doesn't exist for parent {parent}."));
		val_cell.set((val_cell.get() as isize + value) as usize);
	}

	fn reset_graph(&self) {
		self.graph.values().for_each(|child| child.values().for_each(|count| count.set(1)));
	}

	fn reset_parent(&self) {
		*self.parent_list.borrow_mut() = HashMap::default();
	}

	pub fn get_groups(&self) -> usize {
		for (i, p) in self.graph.keys().enumerate() {
			for c in self.graph.keys().skip(i + 1) {
				if self.min_cut(p, c) == 3 {
					let len_of_group = self.parent_list.take().len();
					return len_of_group * (self.graph.len() - len_of_group);
				}
			}
		}
		0
	}
}