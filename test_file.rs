#[test]
fn base58_to_wallet() {

println!("Enter your name:");
let stdin = io::stdin();
let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
//gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt
let wallet = bs58::decode(base58).into_vec().unwrap();
println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
let wallet: Vec<u8> =
vec![[124,174,8,101,6,174,104,8,234,184,51,170,227,171,46,94,161,10,217,13,144,154,124,203,110,122,47,206,157,232,140,94,34,105,64,4,157,229,19,225,143,244,1,82,9,235,179,184,117,79,15,223,67,189,244,200,151,77,247,169,243,17,86,87]];
let base58 = bs58::encode(wallet).into_string();
println!("{:?}", base58);
}