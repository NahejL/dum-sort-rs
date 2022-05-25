

use std::{ collections::*, fmt::* };

pub struct Run <'a> {
	pub name: &'a str,
	pub after: &'a [&'a str],
	pub run: &'a dyn Fn() -> (),
	}

impl Debug for Run <'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_struct("Run")
			.field("name", &self.name)
			.field("after", &self.after)
			.finish()
		}
	}

pub fn run <'a> ( runs: &'a [&'a Run] ) {

	let mut has_ran = HashSet::<&str>::new();

	// borrowing, assert all runs have ran
	'run: for run in runs {
		if has_ran.contains( run.name ) { continue }
		// run run and dependencies
		else {
			// dependency stack to traverse tree
			let mut stack : Vec::<(&Run, usize)> = vec![( &run, 0 )];

			'execution: loop {
				// borrow next run and state to run 
				let ( run, index ) = stack.last().expect( "this shouldnt be empty" );
				// assert all dependencies, continuing, are ran
				'validation: for (index, name) in run.after.iter().skip(*index).enumerate() {
					if has_ran.contains( name ) { continue 'validation }
					// move to this dependency
					else {
						// get run by name
						let run = runs.into_iter().find( |run| run.name == *name ).expect( "invalid name" );
						if stack.contains( run ) { panic!("{:?}", "circular dependency" ); }
						// add to stack, skip this dep since it should have ran
						stack.push(( &run , index +1 ));
						continue 'execution;
						}
					}
				// all dependencies ran, run run
				(run.run)();
				has_ran.insert( run.name );
				stack.pop();
				// until no more runs in stack
				if stack.is_empty() { break 'execution }
				}

			}

		}

	}
