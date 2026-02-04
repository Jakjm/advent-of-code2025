use std::fs::File;
use std::path::Path;
use std::io::{BufRead,BufReader};

pub fn problem_two(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();

    let mut points  = Vec::<(i64,i64)>::new();
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(",").collect();
        if tokens.len() == 2 {
            let x : i64 = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let y : i64 = tokens[1].to_string().trim().parse::<i64>().unwrap();
            
            points.push((x, y));
        }
    }
    
    let mut max_area : i64 = -1;
    //Iterate over every possible rectangle
    //Defined by the points (pt_x,pt_y),(o_x,pt_y),(o_x,o_y),(pt_x,o_y)
    for (count, (pt_x,pt_y)) in points.iter().enumerate() {
        for (o_x,o_y) in points.iter().skip(count + 1) {
            let (diff_x, diff_y) = (o_x - pt_x,o_y - pt_y);
            let area = (i64::abs(diff_x) + 1) * (i64::abs(diff_y) + 1);
            if area > max_area {
                //Iterate through every edge (pair of consecutive points), including the last point and the first point.
                let slice = &[points[points.len() - 1], points[0]][..]; //last/first point pair.
                let wrap_around = std::iter::once(slice); 
                let mut collision = false;
                let (left,right) = (i64::min(*pt_x, *o_x), i64::max(*pt_x, *o_x));
                let (bottom,top) = (i64::min(*pt_y, *o_y), i64::max(*pt_y, *o_y));
                for window in points.windows(2).chain(wrap_around) {
                    if let [(s_x, s_y), (e_x, e_y)] = window {
                        //Check if the line collides with the potential rectangle
                        if *s_x == *e_x { //Vertical line. 
                            if *s_x > left && *s_x < right { //Line is between the left and right sides...
                                let (line_top, line_bottom) = (i64::max(*s_y,*e_y), i64::min(*s_y,*e_y));
                                if line_bottom < top && line_top > bottom {  //And overlaps with the top/bottom
                                    collision = true;
                                    break;
                                }
                            }
                        }
                        else { //Horizontal line.
                            if *s_y > bottom && *s_y < top { //Line is between the top and bottom sides...
                                let (line_left, line_right) = (i64::min(*s_x,*e_x), i64::max(*s_x,*e_x));
                                if line_left < right && line_right > left {  //And overlaps with the left/right
                                    collision = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !collision {
                    max_area = area;
                }
            }
        }
    }
    return max_area;
}
pub fn problem_one(filename: &str) -> i64{
    let path = Path::new(&filename);
    let file_result = File::open(&path);
    if file_result.is_err() {
        return -1; 
    }
    let reader = BufReader::new(file_result.unwrap());
    let all_lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();


    let mut other_pts = Vec::<(i64,i64)>::new();
    let mut max_area : i64 = -1;
    for line in all_lines.iter() {
        let tokens:Vec<&str> = line.split(",").collect();
        if tokens.len() == 2 {
            let x = tokens[0].to_string().trim().parse::<i64>().unwrap();
            let y = tokens[1].to_string().trim().parse::<i64>().unwrap();

            for (o_x,o_y) in other_pts.iter() {
                let (diff_x, diff_y) = (o_x - x,o_y - y);
                let area = (i64::abs(diff_x) + 1) * (i64::abs(diff_y) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
            other_pts.push((x,y));
        }
    }
    
    return max_area;
}
fn main() {
    let result = problem_two("input.txt");
    println!("{}",result);
}
