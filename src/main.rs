// extern crate rand;
use std::io; 
use rand::Rng; 

fn send_bitcoin() {
    println!("You're goin to send bitcoins!\n");
    let clients= vec!["Kirti", "Siddhi", "Anshul", "Hemu"]; 
    println!("Who do you want to send bitcoins to?\n"); 
    for client in &clients{
        print!("{} ", client); 
    }
    println!("\n"); 

    let mut recipient= String::new(); 
    io::stdin().read_line(&mut recipient);
    
    
    if clients.contains(&recipient.trim()) {
        println!("How many bitcoins do you want to send?\n"); 
        let mut amount= String::new(); 
        io::stdin().read_line(&mut amount); 
        println!("You sent {} Bitcoins to {}\n", amount, recipient); 
    }
    else{
        println!("{} is not in your contacts!", recipient);
    }

}

fn receive_bitcoin(){
    println!("We're goin to receive bitcoin!\n"); 
    let mut amount= rand::thread_rng(); 
    
    println!("You just received {} Bitcoins\n", amount.gen_range(0..10));
}

fn exit_console(){
    println!("The value you entered is invalid");
}

fn console() {
    println!("Hey there! Let's use bitcoin\n"); 
    println!("Send(s) or receive(r) bitcoins?\n"); 
    let mut command= String::new(); 
    io::stdin().read_line(&mut command); 

    if command.trim().eq("s"){
        send_bitcoin()
    }
    else if command.trim().eq("r"){
        receive_bitcoin()
    }else{
        exit_console() 
    }

}

fn main() {
    console() 
}
