
//! # Server Comms
//!
//! Modules to communicate with the server, includes
//! server commands and response structs.
//!
//! Reference: https://github.com/csgames/cscoins#communication-with-the-central-authority

pub mod cmd_response;


//---------------------------------------------------------
// Consts
//---------------------------------------------------------

pub static server_uri: &'static str = "wss://cscoins.2017.csgames.org:8989/client";


//---------------------------------------------------------
// Functions
//---------------------------------------------------------
//Functions used to send the commands

//TODO: commands
