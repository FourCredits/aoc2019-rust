const EX_3: &str = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";

fn main() {
    let graph = d18::parse(EX_3, false);
    println!("parsed {:?}", graph);
    let result = graph.best_path_multiple_bots();
    println!("calculated {} as best result", result);
}
