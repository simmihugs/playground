# playground
	Playing around with rust

## First test
	Writing my own struct and implementing a trait.
	
## xml test
	```
	╭[simmi@xubu] ~/Projects/rust/playground
	╰─> cargo run                                          on branch main
	   Compiling playground v0.1.0 (/home/simmi/Projects/rust/playground)
	    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
	     Running `target/debug/playground`
	testcases: {
	  test: {
	     { name: uri,  value: cases/xmlnsquote.xml }
	     { name: type, value: not-wf }
	  },
	  test: {
	     { name: uri,  value: cases/quote.xml }
	     { name: type, value: not-wf }
	  },
	  test: {
	     { name: uri,  value: cases/autosar.xml }
	     { name: type, value: valid }
	  },
	}
	╭[simmi@xubu] ~/Projects/rust/playground
	╰─>                                                    on branch main
	```
