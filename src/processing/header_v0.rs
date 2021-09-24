#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderBinaryV0 {
	pub version: [u8; 2],
	pub datatype: [u8; 1],
}

//deprecated
// #[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
// pub struct HeaderV0 {
// 	pub version: u16,
// 	pub datatype: DataType,
// }

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum DataType {
	Password = 0,
	File = 1,
}

//  most likely deprecated
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

impl HeaderBinaryV0 {
	pub fn header_to_bytes(&self) -> Vec<u8> {
		let mut output = Vec::new();
		output.extend_from_slice(&self.version);
		output.extend_from_slice(&self.datatype);
		output.resize(512, 0);
		output
	}
	pub fn pack_header_from_parameters(version: u16, datatype: &DataType) -> Self {
		let datatype_id: u8;
		match datatype {
			DataType::Password => { datatype_id = 0 }
			DataType::File => { datatype_id = 1 }
		}
		Self {
			version: version.to_be_bytes(),
			datatype: datatype_id.to_be_bytes(),
		}
	}

	// most likely deprecated
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
	//deprecated
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
}


