pub fn factors(n: u64) -> Vec<u64> {
 	let mut num = n;
 	let mut items = (2..);
 	let mut vecs = Vec::new();
    while num > 1 {

    	let item = items.next().unwrap();

    	while (num % item == 0) {
    		num = num / item;
    		vecs.push(item);
    	}

    }

    vecs
}
