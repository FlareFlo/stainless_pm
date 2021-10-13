use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Local, TimeZone, Utc};
use pad::PadStr;
use slpm_file::datatype::DataType;
use slpm_file::header_v0::HeaderV0;
use slpm_file::payload::Entry;

pub fn print_table(headers: Vec<HeaderV0>) {
	let mut old_len: u32 = 0;
	let mut lh = HeaderV0 {
		version: 0,
		datatype: DataType::Password,
		name: "".to_string(),
		created: 0,
		edited: 0,
		file_name: "".to_string(),
		buffer_size: 0,
	};

	for i in headers.clone() {
		let len = i.name.len() + i.file_name.len();
		if len > old_len as usize {
			lh = i;
		}
	}

	let mut file_name = lh.file_name.trim().to_owned();
	if file_name.len() == 0 {
		file_name = "".to_owned().pad_to_width(10);
	}

	let lengths = vec![lh.name.trim().len(), file_name.len(), 5, 13, 14, 0];
	let header = format!("Name{}|File name{}|Type{}|Created{}|Edited{}|version{}",
						 "".pad_to_width(lengths[0]),
						 "".pad_to_width(lengths[1] - 9),
						 "".pad_to_width(lengths[2]),
						 "".pad_to_width(lengths[3]),
						 "".pad_to_width(lengths[4]),
						 "".pad_to_width(lengths[5])
	);
	println!("{}", header);
	println!("{}", "".pad_to_width_with_char(header.len(), '─'));
	for header in headers.clone() {
		let system_time_cr = UNIX_EPOCH + Duration::from_secs(header.created);
		let datetime_cr = DateTime::<Local>::from(system_time_cr);
		let timestamp_str_cr = datetime_cr.format("%Y-%m-%d %H:%M:%S").to_string();

		let system_time_ed = UNIX_EPOCH + Duration::from_secs(header.edited);
		let datetime_ed = DateTime::<Local>::from(system_time_ed);
		let timestamp_str_ed = datetime_ed.format("%Y-%m-%d %H:%M:%S").to_string();

		let td = format!("{}|{}|{}|{}|{}|{}",
						 header.name.trim().pad_to_width(lengths[0] + 4),
						 header.file_name.trim().pad_to_width(lengths[1]),
						 header.datatype.to_string().pad_to_width(lengths[2] + 4),
						 timestamp_str_cr.pad_to_width(lengths[3] + 7),
						 timestamp_str_ed.pad_to_width(lengths[4] + 6),
						 header.version.to_string().to_string().pad_to_width(lengths[5])
		);
		println!("{}", td);
	}
}