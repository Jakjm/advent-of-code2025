use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};
use std::collections::BTreeMap;
use std::convert::TryInto;


pub fn dfs(graph: &BTreeMap<[u8; 3], Vec<[u8; 3]>>, source: [u8; 3], target: [u8; 3], path_count_map: &mut BTreeMap<[u8; 3], usize>) -> usize {
    if source == target {
        return 1;
    }
    else if path_count_map.contains_key(&source) {
        return *path_count_map.get(&source).unwrap();
    }

    let mut total = 0;
    if graph.contains_key(&source) {
        for dest in graph.get(&source).unwrap() {
            total += dfs(graph, *dest, target, path_count_map);
        }
    }
    path_count_map.insert(source, total);
    return total;
}

pub fn problem_one(filename: &str) -> usize{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return 0; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    //Build the graph using a map of sources to the number of paths to the source from "you" and the list of destinations.
    let mut graph = BTreeMap::<[u8; 3], Vec<[u8; 3]>>::new();
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(" ").collect();
        let source = tokens[0].as_bytes()[..3].try_into().unwrap();

        let mut destinations = Vec::<[u8; 3]>::new();
        destinations.reserve(tokens.len() - 1);

        for i in 1..tokens.len(){
            let target = tokens[i].as_bytes()[..3].try_into().unwrap();
            destinations.push(target);
        }
        graph.insert(source, destinations);
    }
    //Note that by the description, it is a directed acyclic graph, which makes the structure of the dfs much simpler.
    return dfs(&graph, "you".as_bytes()[..3].try_into().unwrap(), "out".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
}
pub fn problem_two(filename: &str) -> usize{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return 0; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    //Build the graph using a map of sources to the number of paths to the source from "you" and the list of destinations.
    let mut graph = BTreeMap::<[u8; 3], Vec<[u8; 3]>>::new();
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(" ").collect();
        let source = tokens[0].as_bytes()[..3].try_into().unwrap();

        let mut destinations = Vec::<[u8; 3]>::new();
        destinations.reserve(tokens.len() - 1);

        for i in 1..tokens.len(){
            let target = tokens[i].as_bytes()[..3].try_into().unwrap();
            destinations.push(target);
        }
        graph.insert(source, destinations);
    }
    //Note that by the description, it is a directed acyclic graph, which makes the structure of the dfs much simpler.


    let svr_out = dfs(&graph, "svr".as_bytes()[..3].try_into().unwrap(), "out".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());

    let fft_dac = dfs(&graph, "fft".as_bytes()[..3].try_into().unwrap(), "dac".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
    if fft_dac > 0 {
        let svr_fft = dfs(&graph, "svr".as_bytes()[..3].try_into().unwrap(), "fft".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
        let dac_out = dfs(&graph, "dac".as_bytes()[..3].try_into().unwrap(), "out".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
        let result = svr_fft * fft_dac * dac_out;
        assert!(result < svr_out);
        return result;
    }
    else{
        let svr_dac = dfs(&graph, "svr".as_bytes()[..3].try_into().unwrap(), "dac".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
        let dac_fft = dfs(&graph, "dac".as_bytes()[..3].try_into().unwrap(), "fft".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
        let fft_out = dfs(&graph, "fft".as_bytes()[..3].try_into().unwrap(), "out".as_bytes()[..3].try_into().unwrap(), &mut BTreeMap::<[u8; 3], usize>::new());
        let result = svr_dac * dac_fft * fft_out;
        assert!(result < svr_out);
        return result;
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    let result = problem_two("input.txt");
    println!("{}",result);
}
