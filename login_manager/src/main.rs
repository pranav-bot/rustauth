use authentication_library::{get_users, hash_password, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    ///Lists all users.
    List,
    ///Add a user.
    Add{
        /// Login id
        username: String,
        /// user's password
        password: String,
        ///Optional - mark as admin
        #[arg(long)]
        admin: Option<bool>
    },
    /// Delete a user 
    Delete{
        ///User to delete
        username: String
    },
    /// Change a user's password
    ChangePassword{
        // Username who's password we're changing
        username: String,
        ///Old password for verifiaction
        old_password: String,
        ///New password
        new_password: String
    },
    //Change a user's username
    ChangeUsername{
        ///Old username
        username: String,
        ///Password for verification
        password: String,
        ///new username
        new_username: String
    }
}

fn list_users(){
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_users();
    users
    .iter()
    .for_each(|(_, user)|{
        println!("{:<20}{:20?}", user.username, user.role);
    })
}

fn add_user(username: String, password: String, admin: bool){
    let mut users = get_users();
    let role = if admin{
        LoginRole::Admin
    }
    else{
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String){
    let mut users = get_users();
    if users.contains_key(&username){
        users.remove(&username);
        save_users(users);
    }
    else {
        println!("{username} does not exist");
    }
}

fn change_password(username: String, old_password: String, new_password: String){
    let mut users = get_users();
    if users.contains_key(&username){
        let user = users.get_mut(&username).unwrap();
        if user.password==hash_password(old_password.as_str()){
            user.password = hash_password(new_password.as_str());
            save_users(users);
            println!("Password changed");
        }
        else{
            println!("wrong password");
        }
    }
    else{
        println!("User not found");
    }
}

fn change_username(username: String, password: String, new_username:String){
    let mut users = get_users();
    if users.contains_key(&username){
        let user = users.get_mut(&username).unwrap();
        if user.password == hash_password(password.as_str()){
            user.username = new_username;
            save_users(users);
            println!("username changed");
    }
    else {
        println!("Wrong password");
    }
    }
    else {
        println!("user not found")
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command{
        Some(Commands::List) => {
            println!("List users here");
            list_users();
        }
        Some(Commands::Add { username, password, admin }) =>{
            
            add_user(username, password, admin.unwrap_or(false));
            println!("User Added");
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
            println!("User deleted");
        }
        Some(Commands::ChangePassword { username, old_password, new_password })=> {
            change_password(username, old_password, new_password)
        }
        Some(Commands::ChangeUsername { username, password, new_username }) =>{
            change_username(username, password, new_username)
        }
        None =>{
            println!("Run with -- --help to see instructions")
        }
    }
}
