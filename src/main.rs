pub mod lang_c;

pub mod scf;
use scf::SCFDomain;
use scf::SCFNode;
use scf::SCFNode::{Statement, Cond};

#[derive(Debug)]
pub struct D;
impl<'x> SCFDomain for D {
	type Declaration = String;
	type Expression = String;
	type Statement = String;
	type Node = Box<SCFNode<D>>;
}

fn main() {
	let s1 = Box::new(Statement::<D>("puts(\"Hello, world!\");".to_string()));
	let s2 = Box::new(Statement::<D>("return 0;".to_string()));
	let c = Box::new(Cond::<D>{
		cond: "argc < 2".to_string(),
		body: s1,
		alt:  s2
	});
	let (rs1, rs2) = match *c { Cond::<D>{ref cond, ref body, ref alt} => (body, alt), _ => panic!() };
	println!("{:?}", rs1);
	println!("{:?}", rs2);
	println!("{:?}", c);
	println!("{}", lang_c::serialize(&c));
}