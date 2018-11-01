use entry::*;

#[derive(Debug)]
pub struct Stack(Vec<Entry>);

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

	pub fn peek(&self) -> Option<&Entry> {
		self.0.as_slice().last()
	}

	pub fn pop_slice(&mut self, depth: usize) -> Stack {
		assert!(depth <= self.len(), "`depth` greater than stack height");
		let height = self.len() - depth;
		Stack(self.0.split_off(height))
	}
	
	pub fn as_slice(&self) -> &[Entry] {
		self.0.as_slice()
	}

	pub fn operate(&mut self) {
		let r = match self.pop() {
			Some(Entry::Op(op)) =>
				if self.len() < op.arity{
					Entry::Panic(format!("cannot apply: {} has arity {}, stack has {} elements",
								   op.name, op.arity, self.len()))
				} else {
					let args = self.pop_slice(op.arity);
					(op.body)(args.as_slice())
				},
			Some(e) =>
				Entry::Panic(format!("tried to operate with non-operator entry: {:?}",
							   e)),
			None =>
				Entry::Panic("tried to operate with empty stack".to_string()),
		};
		self.push(r);
	}

	pub fn panic(&mut self) {
		println!("Panicked because: {:?}", self.pop());
	}
}
