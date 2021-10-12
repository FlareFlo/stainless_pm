use std::collections::BinaryHeap;

use slpm_file::header_binary_v0::HeaderBinaryV0;
use slpm_file::header_v0::HeaderV0;
use slpm_file::payload::Entry;

use crate::menu_options::init;

mod menu_options;
mod manager;
mod table_printer;

fn main() {
	let mut entries: Vec<Entry> = vec![];
	init(entries);
}