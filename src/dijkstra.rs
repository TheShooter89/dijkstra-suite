/// compute the best possible path in a graph using Dijkstra algorithm
pub fn dijkstra_path(graph: &str, start: &str, end: &str) -> Result<(), String> {
    println!("dijkstra_path function");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = dijkstra_path("graph", "A", "B");

        // assert_eq!(result, Err("blyat".into()));
        assert_eq!(result, Ok(()));
    }
}
