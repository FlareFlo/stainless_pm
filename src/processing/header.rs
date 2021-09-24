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

pub fn pack_header_from_raw(version: u16, length: u16, mut datatype: DataType, salt: [u8; 22], nonce: [u8; 12]) -> HeaderBinary {
	let datatype_id: u8;
	match datatype {
		DataType::Password => {datatype_id = 0}
		DataType::File => {datatype_id = 1}
	}
	let header = HeaderBinary {
		version: <[u8; 2]>::try_from(version.to_be_bytes()).unwrap(),
		length: <[u8; 2]>::try_from(length.to_be_bytes()).unwrap(),
		datatype: <[u8; 1]>::try_from(datatype_id.to_be_bytes()).unwrap(),
		salt,
		nonce,
	};
	return header
}

pub fn pack_header(header: Header) -> HeaderBinary {
	let datatype_id: u8;
	match header.datatype {
		DataType::Password => {datatype_id = 0}
		DataType::File => {datatype_id = 1}
	}
	let header_binary = HeaderBinary {
		version: <[u8; 2]>::try_from(header.version.to_be_bytes()).unwrap(),
		length: <[u8; 2]>::try_from(header.length.to_be_bytes()).unwrap(),
		datatype: <[u8; 1]>::try_from(datatype_id.to_be_bytes()).unwrap(),
		salt: header.salt,
		nonce: header.nonce,
	};
	return header_binary
}

pub fn header_to_bytes(header: HeaderBinary) -> Vec<u8>{
	let mut output = Vec::new();
	output.extend_from_slice(&header.version);
	output.extend_from_slice(&header.length);
	output.extend_from_slice(&header.datatype);
	output.extend_from_slice(&header.salt);
	output.extend_from_slice((&header.nonce));
	let len = u16::from_be_bytes(header.length) as usize;
	if output.len() <= len {
		output.resize(len, 0);
	}else {
		panic!("Header size exceeded")
	}
	return output

}