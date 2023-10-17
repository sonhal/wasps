// Setup state to hold implant connection info
// handle implant communication
    // Implant register request (new implant calls home)
    // Communication channel with existing implants (queue based?)


struct WaspConnection {
    name: String
}


struct WaspHive {
    name: String,
    wasps: Vec<WaspConnection>
}


impl  WaspHive {

    fn handle(register: String) {
        todo!()
    }
    
}