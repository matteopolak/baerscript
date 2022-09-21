use runtime::grid::Grid;
use runtime::parser::interpret;

#[test]
fn empty_grid_test() {
	let grid = Grid::try_from("".lines());

	assert!(grid.is_ok(), "empty input should provide an empty grid");
}

#[test]
fn increment_print_test() {
	let mut grid = Grid::try_from(
		r"+v
+v
>v"
		.lines(),
	)
	.unwrap();

	let mut out = vec![0; 1];

	interpret(&mut grid, false, false, &mut std::io::empty(), &mut out);

	assert_eq!(vec![0, 50], out);
}
