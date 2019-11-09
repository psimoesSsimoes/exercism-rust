pub fn verse(n: u32) -> String {
	match n {
		0 => return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, {} bottles of beer on the wall.\n",99),
		1 => return format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.",n,n),
		_ => return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.",n, n, n-1)
	}
}

pub fn sing(start: u32, end: u32) -> String {
	let mut song = String::new();
    for x in start..end {
		song.push_str(&verse(x))
	}

	return song
}
