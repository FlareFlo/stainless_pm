use std::convert::TryFrom;

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct HeaderBinary {
	pub version: [u8; 2],
	pub length: [u8; 2],
	pub datatype: [u8; 1],

	pub salt: [u8; 22],
	pub nonce: [u8; 12],
}

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Header {
	pub version: u16,
	pub length: u16,
	pub datatype: DataType,

	pub salt: [u8; 22],
	pub nonce: [u8; 12],
}

#[derive(Clone, Hash, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub enum DataType {
	Password = 0,
	File = 1,
}

impl Header {
	pub fn pack_header(&self) -> HeaderBinary {
		let datatype_id: u8;
		match self.datatype {
			DataType::Password => {datatype_id = 0}
			DataType::File => {datatype_id = 1}
		}
		let header_binary = HeaderBinary {
			version: <[u8; 2]>::try_from(self.version.to_be_bytes()).unwrap(),
			length: <[u8; 2]>::try_from(self.length.to_be_bytes()).unwrap(),
			datatype: <[u8; 1]>::try_from(datatype_id.to_be_bytes()).unwrap(),
			salt: self.salt,
			nonce: self.nonce,
		};
		return header_binary
	}
}

impl HeaderBinary {
	pub fn header_to_bytes(&self) -> Vec<u8>{
		let mut output = Vec::new();
		output.extend_from_slice(&self.version);
		output.extend_from_slice(&self.length);
		output.extend_from_slice(&self.datatype);
		output.extend_from_slice(&self.salt);
		output.extend_from_slice((&self.nonce));
		let len = u16::from_be_bytes(self.length) as usize;
		if output.len() <= len {
			output.resize(len, 0);
		}else {
			panic!("Header size exceeded")
		}
		return output

	}
	pub fn pack_header_from_raw(version: u16, length: u16, datatype: DataType, salt: [u8; 22], nonce: [u8; 12]) -> Self {
		let datatype_id: u8;
		match datatype {
			DataType::Password => {datatype_id = 0}
			DataType::File => {datatype_id = 1}
		}
		let header = Self {
			version: <[u8; 2]>::try_from(version.to_be_bytes()).unwrap(),
			length: <[u8; 2]>::try_from(length.to_be_bytes()).unwrap(),
			datatype: <[u8; 1]>::try_from(datatype_id.to_be_bytes()).unwrap(),
			salt,
			nonce,
		};
		return header
	}
	pub fn deserialize_binary_v0(binary: Vec<u8>) -> Self {
		let version_and_rest = binary.split_at(2);
		let length_and_rest = version_and_rest.1.split_at(2);
		let datatype_and_rest = length_and_rest.1.split_at(1);
		let salt_and_rest = length_and_rest.1.split_at(22);
		let nonce_and_rest = salt_and_rest.1.split_at(12);
		let header_binary = HeaderBinary {
			version: <[u8; 2]>::try_from(version_and_rest.0).unwrap(),
			length: <[u8; 2]>::try_from(length_and_rest.0).unwrap(),
			datatype: <[u8; 1]>::try_from(datatype_and_rest.0).unwrap(),
			salt: <[u8; 22]>::try_from(salt_and_rest.0).unwrap(),
			nonce: <[u8; 12]>::try_from(nonce_and_rest.0).unwrap(),
		};
		return header_binary
	}
}


