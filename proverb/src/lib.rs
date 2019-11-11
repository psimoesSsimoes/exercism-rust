pub fn build_proverb(list: &[&str]) -> String {
	let mut proverb = String::new();

	for slice in list.windows(2){
		proverb.push_str(
			&format!(
				"For want of a {} the {} was lost.\n",
				slice[0],
				slice[1]
			)
		);
	}

	if list.len()>=1{
		proverb.push_str(
			&format!(
				"And all for the want of a {}.",
				list.first().unwrap()
				)
			)
	}


	return proverb;
}
