use entry::*;

struct Stack(Vec<Entry>);

impl Stack {
	//reimplement new, len, push, pop, split_off
	pub fn new() -> Stack {
		Stack(Vec::new())
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn push(&mut self, v: Entry) {
		self.0.push(v)
	}

	pub fn pop(&mut self) -> Option<Entry> {
		self.0.pop()
	}

	pub fn pop_slice(&mut self, depth: usize) -> Stack {
		assert!(depth <= self.len(), "`depth` greater than stack height");
		let height = self.len() - depth;
		Stack(self.0.split_off(height))
	}

	pub fn operate(self, op: Operator) -> Stack {
		if self.len() < op.arity{
			panic!(format!("cannot apply: {} has arity {}, stack has {} elements",
						   op.name, op.arity, self.len()));
		} else {
			let args = self.pop_slice(op.arity);
			self.push((op.body)(&args));
		}
		self
	}
}
