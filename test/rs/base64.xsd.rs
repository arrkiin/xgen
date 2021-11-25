// Code generated by xgen. DO NOT EDIT.

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;


// MyType1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType1 {
	#[serde(rename = "myType1")]
	pub my_type1: String,
}


// MyType2 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType2 {
	#[serde(rename = "length")]
	pub length: Option<i32>,
	#[serde(rename = "$value")]
	pub value: String,
}


// MyType3 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType3 {
	#[serde(rename = "length")]
	pub length: Option<i32>,
	#[serde(rename = "$value")]
	pub value: u8,
}


// MyType4 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType4 {
	#[serde(rename = "title")]
	pub title: String,
	#[serde(rename = "blob")]
	pub blob: String,
	#[serde(rename = "timestamp")]
	pub timestamp: u8,
}


// MyType5 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType5 {
	#[serde(rename = "myType5")]
	pub my_type5: String,
}


// MyType6 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType6 {
	#[serde(rename = "code")]
	pub code: Option<String>,
	#[serde(rename = "identifier")]
	pub identifier: Option<i32>,
}


// MyType7 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MyType7 {
	#[serde(rename = "origin")]
	pub origin: String,
	#[serde(rename = "$value")]
	pub value: String,
}


// TopLevel ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TopLevel {
	#[serde(rename = "cost")]
	pub cost: Option<f64>,
	#[serde(rename = "LastUpdated")]
	pub last_updated: Option<u8>,
	#[serde(rename = "nested")]
	pub nested: MyType7,
	#[serde(flatten)]
	pub my_type6: MyType6,
}
