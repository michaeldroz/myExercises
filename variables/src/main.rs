fn main() {
    let mut missiles = 8; 
    let ready = 2; 
    println!("Firing {} of {} missiles...", ready, missiles);
    missiles = missiles - ready; 
    println!("{} missiles left", missiles); 
}
