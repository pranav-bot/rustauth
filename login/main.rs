use authentication_library::{login, read_line, LoginAction, LoginRole};

fn main() {
    let mut tries = 0;
    loop {
        println!("Enter username");
        let username = read_line();
        println!("enter Password");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role))=>{
                match role {
                    LoginRole::Admin => {println!("Admin")},
                    LoginRole::User => {println!("User")}
                }
                break;
            }         
            Some(LoginAction::Denied) =>{
                println!("Denied");
                tries +=1;
            }
            None => {
                println!("New User")
            }   
        }
        if tries>3{
            println!("Tries exceeded");
            break;
        }
    }
}
