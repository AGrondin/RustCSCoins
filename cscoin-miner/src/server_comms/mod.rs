
//! # Server Comms
//!
//! Modules to communicate with the server, includes
//! server commands and response structs.
//!
//! Reference: https://github.com/csgames/cscoins#communication-with-the-central-authority

use std::mem;
use std::io::Cursor;

//NOTE: Some imports are renamed with a WSC prefix because
//      yes... there are different implementations of the
//      same type and yes we are using both... The WSC
//      prefixed ones are the types used to declare the
//      WebSockClient type args.
use websocket::{Client as WebSockClient, Message, Receiver, Sender};
use websocket::client::request::Url;
use websocket::dataframe::DataFrame as WSCDataFrame; //See note above
use websocket::receiver::Receiver as WSCReceiver; //See note above
use websocket::sender::Sender as WSCSender; //See note above
use websocket::stream::WebSocketStream;
use websocket::ws::dataframe::DataFrame;
use serde;
use serde_json;
use serde_json::map::Map;
use serde_json::Value;
use serde_json::Number;

use server_comms::error::CSCoinClientError;
use server_comms::cmd_response::{CurrentChallenge,
                                 ChallengeSolution};

pub mod cmd_response;
pub mod error;


//---------------------------------------------------------
// Consts
//---------------------------------------------------------

pub static server_uri: &'static str = "wss://cscoins.2017.csgames.org:8989/client";


//---------------------------------------------------------
// Payload Struct
//---------------------------------------------------------

#[derive(Serialize)]
pub struct CommandPayload {
    pub command: String,
    pub args:    Option<Map<String, Value>>
}


//---------------------------------------------------------
// Client
//---------------------------------------------------------
//Holds the client state

pub struct CSCoinClient {
    client: WebSockClient<WSCDataFrame, WSCSender<WebSocketStream>, WSCReceiver<WebSocketStream>>
}

impl CSCoinClient {

    //Use this to connect to the CA Server
    pub fn connect() -> Result<CSCoinClient, CSCoinClientError>{

        // safe to unwrap, if this crashes then we have a
        // typo in our constant.
        let url      = Url::parse(server_uri).unwrap();      // Parse url
        let request  = try!(WebSockClient::connect(url)      // Connect to server
            .map_err(CSCoinClientError::WebSockErr));
        let response = try!(request.send()                   // Send request
            .map_err(CSCoinClientError::WebSockErr));
        try!(response.validate()                             // Validate response
            .map_err(CSCoinClientError::WebSockErr));

        Ok(CSCoinClient {
            client: response.begin()
        })
    }

    //Implementation of close command
    //Reference: https://github.com/csgames/cscoins#close-connection
    pub fn disconnect(&mut self) -> Result<(), CSCoinClientError> {

        //JSONify payload
        let payload = try!(serde_json::to_string(&CommandPayload{
            command: "close".to_string(),
            args:    Option::None
        }).map_err(CSCoinClientError::JSONErr));

        //Send Payload
        try!(self.client.send_message(&Message::text(payload))
            .map_err(CSCoinClientError::WebSockErr));

        //Close client side connection
        try!(self.client.shutdown().map_err(CSCoinClientError::IOErr));

        drop(self);
        return Ok(())
    }

    //Helper command
    pub fn send_command<D: serde::Deserialize>(&mut self, command_payload: CommandPayload) -> Result<D, CSCoinClientError> {

        //JSONify payload
        let payload = try!(serde_json::to_string(&command_payload)
            .map_err(CSCoinClientError::JSONErr));

        //Send Payload
        try!(self.client.send_message(&Message::text(payload))
            .map_err(CSCoinClientError::WebSockErr));

        let mut receiver = self.client.get_mut_receiver();

        //Receive and extract response
        let response: Message = try!(receiver.recv_message() //get response
            .map_err(CSCoinClientError::WebSockErr));
        let mut response_cursor = Cursor::new(Vec::new());   //create essentially what is a buffer
        try!(response.write_payload(&mut response_cursor)    //write payload to buffer
            .map_err(CSCoinClientError::WebSockErr));
        //Turn buffer data to String
        let response_str = try!(String::from_utf8(response_cursor.into_inner())
            .map_err(CSCoinClientError::UTF8Err));

        serde_json::from_str(&response_str[..]).map_err(CSCoinClientError::JSONErr)
    }


    /// ## Get Current Challenge
    ///
    /// Fetch the current problem set from the Central Authority
    ///
    /// Arguments: none
    /// Response:  CurrentChallenge
    ///
    /// References: https://github.com/csgames/cscoins#get-current-challenge
    pub fn get_current_challenge(&mut self) -> Result<CurrentChallenge, CSCoinClientError> {
        self.send_command(CommandPayload{
            command: "get_current_challenge".to_string(),
            args:    Option::None
        })
    }


    /// ## Get Challenge Solution
    ///
    /// Fetch the solution of a challenge
    ///
    /// Command:   "get_challenge_solution"
    /// Arguments: challenge_id: u64
    /// Response:  ChallengeSolution
    ///
    /// References: https://github.com/csgames/cscoins#get-current-challenge
    pub fn get_challenge_solution(&mut self, challenge_id: u64) -> Result<ChallengeSolution, CSCoinClientError> {
        let mut args: Map<String, Value> = Map::new();
        args.insert("challenge_id".to_string(), Value::Number(Number::from(challenge_id)));
        self.send_command(CommandPayload{
            command: "get_challenge_solution".to_string(),
            args:    Some(args)
        })
    }

}


