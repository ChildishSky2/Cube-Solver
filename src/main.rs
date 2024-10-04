
mod rubix_cube;
//use rubix_cube::Iddfs;
use rubix_cube::Iddfs;
fn main() {
    let mut rubix = rubix_cube::RubixCube::create_solved_rubix();

    rubix.make_random_moves(9);
    //rubix.search_astar();
    rubix.thread_search_iddfs(10);
    //rubix.print_cube();

}
    