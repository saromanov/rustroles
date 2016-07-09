extern crate redis;
use redis::Commands;


pub #[derive(Debug)]
struct Roles {
	items: HashMap<String, i32>,
	redisClient: &redis::Connection
}

impl Roles {
	fn new(address: String) -> Roles {
		let client = try!(redis::Client::open(address));
		Roles {
			redisClient: client
		}
	}

	pub fn register_role(identity: String, name: String) -> RetType {
		 let _ : () = try!(con.set(identity, name));
	}

	pub fn register_permission(identity: String, roleName: String, action: String, method:String, permission: bool) -> RetType {
		unimplemented!();
	}

	pub fn check_permission(arg: Type) -> bool {
		match expr {
			Some(expr) => true,
			None => false,
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
