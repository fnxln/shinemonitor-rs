mod client;
pub mod response;

pub use crate::client::ShineMonitorClient;

#[cfg(test)]
mod tests {

    use crate::ShineMonitorClient;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    // --------------------------
    // Ping the server
    // --------------------------
    #[test]
    fn ping() {
        let client: ShineMonitorClient = ShineMonitorClient::default();
        let res = aw!(client.ping());
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().err, 8);
        assert_eq!(
            res.as_ref().unwrap().desc,
            String::from("ERR_FORBIDDEN(missing required parameters)")
        );
    }
    // ----------------------------------
    // Test the auth with $SUSER and SPASS
    // ----------------------------------
    #[test]
    fn auth() {
        use dotenv::dotenv;
        use std::env;
        let client: ShineMonitorClient = ShineMonitorClient::default();
        dotenv().ok();
        let (user, pass): (String, String) = (
            env::var("SUSER").expect("Couldnt get user env variable"),
            env::var("SPASS").expect("Couldnt get password env variable"),
        );
        let res = aw!(client.auth(user.as_str(), pass.as_str()));
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().err, 0);
        assert_eq!(res.as_ref().unwrap().desc, String::from("ERR_NONE"))
    }
    #[test]
    fn pid() {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let client: ShineMonitorClient = ShineMonitorClient::default();
        let (user, pass): (String, String) = (
            env::var("SUSER").expect("Couldnt get user env variable"),
            env::var("SPASS").expect("Couldnt get password env variable"),
        );
        let pid = aw!(aw!(client.auth(user.as_str(), pass.as_str()))
            .unwrap()
            .pid());
        assert!(pid.is_ok());
        assert_eq!(pid.unwrap().err, 0)
    }
    #[test]
    fn info() {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let client: ShineMonitorClient = ShineMonitorClient::default();
        let (user, pass): (String, String) = (
            env::var("SUSER").expect("Couldnt get user env variable"),
            env::var("SPASS").expect("Couldnt get password env variable"),
        );
        let pid = aw!(aw!(client.auth(user.as_str(), pass.as_str()))
        .unwrap()
        .pid()).unwrap().dat.info.first().unwrap().clone();
	let info = aw!(aw!(client.auth(user.as_str(),pass.as_str())).unwrap().plant());
	// println!("{:?}",);
    assert!(info.is_ok());
    assert_eq!(info.as_ref().unwrap().dat.pid, pid.pid);   
    assert_eq!(info.as_ref().unwrap().dat.uid, pid.uid)    
    }
}
