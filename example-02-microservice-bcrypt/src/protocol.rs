use std::str;
use nom::IResult;
use nom::{newline,digit,alphanumeric,space,not_line_ending};

#[derive(Debug, PartialEq)]
pub enum BaasProtocol {
	SetCost(u32),
	Hash(String),
	Verify(String, String),
}

impl BaasProtocol {
	pub fn parse(input: &[u8]) -> IResult<&[u8], BaasProtocol> {
		parse(input)
	}
}

named!(cost<BaasProtocol>, do_parse!(tag!("cost") >> cost: digit >> newline >> (BaasProtocol::SetCost(str::from_utf8(cost).unwrap().parse().unwrap()))));
named!(hash<BaasProtocol>, do_parse!(tag!("hash") >> s: alphanumeric >> newline >> (BaasProtocol::Hash(String::from_utf8(s.to_vec()).unwrap()))));
named!(verify<BaasProtocol>, do_parse!(tag!("verify") >> hash: alphanumeric >> space >> verify: not_line_ending >> newline >> (BaasProtocol::Verify(String::from_utf8(hash.to_vec()).unwrap(), String::from_utf8(verify.to_vec()).unwrap()))));

named!(parse<BaasProtocol>, alt!(cost | hash | verify));

#[test]
fn test_cost() {
	assert_eq!(parse("cost123\n".as_bytes()).unwrap().1, BaasProtocol::SetCost(123));
	assert!(parse("cost1a3\n".as_bytes()).is_err());
	assert!(parse("lol\n".as_bytes()).is_err());
}

#[test]
fn test_hash() {
	assert_eq!(parse("hashhelloworld\n".as_bytes()).unwrap().1, BaasProtocol::Hash("helloworld".to_owned()));
	assert!(parse("hash\n".as_bytes()).is_err());
	assert!(parse("hash&\n".as_bytes()).is_err());
}

#[test]
fn test_verify() {
	assert_eq!(parse("verifyhello $y2%$ab&$y2%$ab&$y2%$ab&$y2%$ab&\n".as_bytes()).unwrap().1, BaasProtocol::Verify("hello".to_owned(), "$y2%$ab&$y2%$ab&$y2%$ab&$y2%$ab&".to_owned()));
}
