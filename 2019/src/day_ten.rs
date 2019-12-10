use std::fs;
use std::f32;
use std::f32::consts;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use crate::util;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AsteroidInfo {
	pub coord: (i32, i32),
	pub d: u32,
}

impl AsteroidInfo {
pub fn new(x: i32, y: i32, d: u32) -> AsteroidInfo {
		AsteroidInfo { coord: (x, y), d }
	}
}

impl Ord for AsteroidInfo {
	fn cmp(&self, other: &AsteroidInfo) -> Ordering {
		other.d.cmp(&self.d)
	}
}

impl PartialOrd for AsteroidInfo {
	fn partial_cmp(&self, other: &AsteroidInfo) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn angle_between_pts(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
	let dx = (x1 - x0) as f32;
	let dy = (y1 - y0) as f32;

	// Looks like I needed at least 1 decimal place of accuracy to
	// get the angles correct
	let angle = f32::atan2(dy, dx) * 180.0 / f32::consts::PI;
	(angle * 10.0).round() as i32
}

// After thinking of a few different approaches, I realized all I
// needed to do was to keep a Set of the relative angles of the
// other asteroids. I don't care about individual asteroids. With a
// Set, then asteroids with the same angle as others are filtered out.
fn count_visible(map: &str, x0: i32, y0: i32, width: i32) -> usize {
	let mut asteroids: HashSet<i32> = HashSet::new();

	let (mut x, mut y) = (0, 0);
	for ch in map.chars() {
		match ch {
			'.' => x = (x + 1) % width,
			'#' => {
				if x != x0 || y != y0 {
					let angle = angle_between_pts(x0, y0, x, y);
					asteroids.insert(angle);
				}
				x = (x + 1) % width;
			},
			_ => {
				x = 0;
				y += 1;
			}
		}
	}

	asteroids.len()
}

// I'm absolutely sure this would be simplified but I'm not sure how mod
// works with algebra and I've been banging my head at the Catersian Plane
// for too long at this point
fn clockwise_angle(deg: i32) -> i32 {
	((3600 - (deg + 3600) % 3600) % 3600 + 900) % 3600
}

// For Q2, I am going to put all the asteroids into a Dictionary whose key is their map,
// stored in a priority queue. Then, to find the 200th asteroid zapped, I'll loop over the keys
// smallest to greatest (which simulates the clockwise rotation of the laser), popping on asteroid
// off each queue until I've counted 200.
pub fn solve_q2() {
	//let mut map_txt = ".#..#\n.....\n#####\n....#\n...##\n";
	let map_txt = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##";

	let mut asteroids: HashMap<i32, BinaryHeap<AsteroidInfo>> = HashMap::new();
	let width = map_txt.find('\n').unwrap() as i32;
	let (x0, y0) = (8, 3);
	let mut x = 0;
	let mut y = 0;
	for ch in map_txt.chars() {
		match ch {
			'.' => x = (x + 1) % width,
			'X' => x = (x + 1) % width,
			'#' => {
				let angle = clockwise_angle(angle_between_pts(0, 0, x - x0, (y - y0) * -1));
				let ai = AsteroidInfo::new(x, y, util::manhattan_d(x0, y0, x, y));
				match asteroids.get_mut(&angle) {
					Some(ast_vec) => ast_vec.push(ai),
					None => {
						let mut heap = BinaryHeap::new();
						heap.push(ai);
						asteroids.insert(angle, heap);
					}
				}
				x = (x + 1) % width;
			},
			'\n' => {
				x = 0;
				y += 1;
			}
			_ => {
				println!("Hmm? '{}'", ch);
				panic!("Unexpected character in data!");
			},
		}
	}

	let heap = asteroids.get_mut(&0).unwrap();
	//println!("{:?}", heap);
	let ai = heap.pop().unwrap();
	println!("{:?}", ai);
	//println!("{:?}", heap.pop());
	// For my own sanity, I think I'll transpose coordinates so that the laser
	// station is at (0, 0)
	let x0 = 50;
	let y0 = -50;
	/*
	println!("N {}", angle_between_pts(x0, y0, 50, 0));
	println!("E {}", angle_between_pts(x0, y0, 80, -50));
	println!("S {}", angle_between_pts(x0, y0, 55, -100));
	println!("W {}", angle_between_pts(x0, y0, 0, -50));
	println!("");
	println!("N   {}", clockwise_angle(angle_between_pts(x0, y0, 50, 50)));
	println!("NNW {}", clockwise_angle(angle_between_pts(x0, y0, 49, 50)));
	println!("NNE {}", clockwise_angle(angle_between_pts(x0, y0, 51, 50)));
	println!("E   {}", clockwise_angle(angle_between_pts(x0, y0, 80, -50)));
	println!("S   {}", clockwise_angle(angle_between_pts(x0, y0, 50, -100)));
	println!("W   {}", clockwise_angle(angle_between_pts(x0, y0, 0, -50)));
	*/
}

pub fn solve_q1() {
	let mut map_txt = fs::read_to_string("./inputs/day10.txt").expect("Missing input file");
	map_txt = map_txt.replace("\r", "").trim().to_string();
	//let mut map_txt = ".#..#\n.....\n#####\n....#\n...##\n";
	//map_txt = map_txt.trim();
	let width = map_txt.find('\n').unwrap();
	let length = map_txt.trim().matches('\n').count() + 1;

	let (mut x, mut y) = (0, 0);
	let mut highest = 0;
	let mut best_loc = (0, 0);
	for ch in map_txt.chars() {
		match ch {
			'.' => x = (x + 1) % width,
			'#' => {
				let count = count_visible(&map_txt, x as i32, y as i32, width as i32);
				if count > highest {
					highest = count;
					best_loc = (x, y);
				}
				x = (x + 1) % width;
			},
			'\n' => {
				x = 0;
				y += 1;
			}

			_ => {
				println!("Hmm? '{}'", ch);
				panic!("Unexpected character in data!");
			},
		}
	}
	println!("Q1: {} at {:?}", highest, best_loc);
}
