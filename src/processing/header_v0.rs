use std::convert::TryFrom;
use std::option::Option::Some;

use chrono::format::Numeric::Timestamp;
use pad::PadStr;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderBinaryV0 {
	pub version: [u8; 2],
	pub datatype: [u8; 1],
	pub name: [u8; 32],
	pub created: [u8; 8],
	pub edited: [u8; 8],
}

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum DataType {
	Password = 0,
	File = 1,
}

impl HeaderBinaryV0 {
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut output = Vec::new();
		output.extend_from_slice(&self.version);
		output.extend_from_slice(&self.datatype);
		output.extend_from_slice(&self.name);
		output.extend_from_slice(&self.created);
		output.extend_from_slice(&self.edited);
		output.resize(1024, 0);
		output
	}
	pub fn from_bytes(bytes: &[u8; 1024]) -> Self {
		let version_and_rest = bytes.split_at(2);
		let datatype_and_rest = version_and_rest.1.split_at(1);
		let name_and_rest = datatype_and_rest.1.split_at(32);
		let created_and_rest = name_and_rest.1.split_at(8);
		let edited_and_rest = created_and_rest.1.split_at(8);
		Self {
			version: <[u8; 2]>::try_from(version_and_rest.0).unwrap(),
			datatype: <[u8; 1]>::try_from(datatype_and_rest.0).unwrap(),
			name: <[u8; 32]>::try_from(name_and_rest.0).unwrap(),
			created: <[u8; 8]>::try_from(created_and_rest.0).unwrap(),
			edited: <[u8; 8]>::try_from(edited_and_rest.0).unwrap(),
		}
	}

	pub fn from_parameters(datatype: &DataType, name: &str, create: Option<i64>) -> Self {
		let mut create_date = chrono::Local::now().timestamp();
		if let Some(create) = create {
			create_date = create;
		}

		let datatype_id: u8;
		match datatype {
			DataType::Password => { datatype_id = 0 }
			DataType::File => { datatype_id = 1 }
		}

		let name_padded = name.pad_to_width(32);

		Self {
			version: <[u8; 2]>::from(0_u16.to_be_bytes()), //Increment for new file
			datatype: datatype_id.to_be_bytes(),
			name: <[u8; 32]>::try_from(name_padded.as_bytes()).unwrap(),
			created: <[u8; 8]>::try_from(create_date.to_be_bytes()).unwrap(),
			edited: <[u8; 8]>::try_from(chrono::Local::now().timestamp().to_be_bytes()).unwrap(),
		}
	}
}
//deprecated
// #[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
// pub struct HeaderV0 {
// 	pub version: u16,
// 	pub datatype: DataType,
// }

// impl HeaderV0 {
// 	pub fn pack_header(&self) -> HeaderBinaryV0 {
// 		let datatype_id: u8;
// 		match self.datatype {
// 			DataType::Password => { datatype_id = 0 }
// 			DataType::File => { datatype_id = 1 }
// 		}
// 		let header_binary = HeaderBinaryV0 {
// 			version: <[u8; 2]>::try_from(self.version.to_be_bytes()).unwrap(),
// 			datatype: <[u8; 1]>::try_from(datatype_id.to_be_bytes()).unwrap(),
// 		};
// 		return header_binary;
// 	}
// }

// pub fn deserialize_header(binary: Vec<u8>) -> Self {
// 	let version_and_rest = binary.split_at(2);
// 	let datatype_and_rest = version_and_rest.1.split_at(1);
// 	let salt_and_rest = datatype_and_rest.1.split_at(22);
// 	let nonce_and_rest = salt_and_rest.1.split_at(12);
// 	let header_binary = HeaderBinaryV0 {
// 		version: <[u8; 2]>::try_from(version_and_rest.0).unwrap(),
// 		datatype: <[u8; 1]>::try_from(datatype_and_rest.0).unwrap(),
// 	};
// 	return header_binary;
// }
// 	pub fn unpack_header(&self) -> HeaderV0 {
// 		let datatype: DataType;
// 		match u8::from_be_bytes(self.datatype) {
// 			0 => datatype = DataType::Password,
// 			1 => datatype = DataType::File,
// 			_ => { panic!("Could not detect file type of header") }
// 		}
// 		let header = HeaderV0 {
// 			version: u16::from_be_bytes(self.version),
// 			datatype,
// 		};
// 		return header;
// 	}


