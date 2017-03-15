
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
                                 ChallengeSolution,
                                 RegisterWallet,
                                 Transactions,
                                 CreateTransaction,
                                 SubmitProblem,
                                 CAServerInfo};

pub mod cmd_response;
pub mod error;


//---------------------------------------------------------
// Consts
//---------------------------------------------------------

pub static DEFAULT_URI: &'static str = "wss://cscoins.2017.csgames.org:8989/client";


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
    pub fn connect(server_uri: &'static str) -> Result<CSCoinClient, CSCoinClientError>{

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


    /// ## Register a New Wallet
    ///
    /// Register your Wallet's public key with the Central Authority.
    ///
    /// Command:   "register_wallet"
    /// Arguments: name:      String
    ///            key:       String
    ///            signature: String
    /// Response:  RegisterWallet
    ///
    /// References: https://github.com/csgames/cscoins#register-a-new-wallet
    pub fn register_wallet(&mut self, name: String, key: String, signature: String) -> Result<RegisterWallet, CSCoinClientError> {
        let mut args: Map<String, Value> = Map::new();
        args.insert("name".to_string(),      Value::String(name));
        args.insert("key".to_string(),       Value::String(key));
        args.insert("signature".to_string(), Value::String(signature));
        self.send_command(CommandPayload{
            command: "get_challenge_solution".to_string(),
            args:    Some(args)
        })
    }


    /// ## Get Transactions
    ///
    /// Get transactions history from the Central Authority.
    ///
    /// Command:   "get_transactions"
    /// Arguments: start: u64
    ///            count: u64
    /// Response:  Transactions
    ///
    /// References: https://github.com/csgames/cscoins#get-transactions
    pub fn get_transactions(&mut self, start: u64, count: u64) -> Result<Transactions, CSCoinClientError> {
        let mut args: Map<String, Value> = Map::new();
        args.insert("start".to_string(), Value::Number(Number::from(start)));
        args.insert("count".to_string(), Value::Number(Number::from(count)));
        self.send_command(CommandPayload{
            command: "get_transactions".to_string(),
            args:    Some(args)
        })
    }


    //TODO: f64 error check (Rust has but JSON doesnt have NaN nor Infinity
    // serde_json checks for that)
    /// ## Create a new Transaction (Send coins)
    ///
    /// Create a new Transaction, sending coins to another wallet
    ///
    /// Command:   "create_transaction"
    /// Arguments: source:    String
    ///            recipient: String
    ///            amount:    f64
    ///            signature: String
    /// Response:  CreateTransaction
    ///
    /// References: https://github.com/csgames/cscoins#create-a-new-transaction-send-coins
    pub fn create_transaction(&mut self, source: String, recipient: String, amount: f64, signature: String) -> Result<CreateTransaction, CSCoinClientError> {
        let mut args: Map<String, Value> = Map::new();
        args.insert("source".to_string(),    Value::String(source));
        args.insert("recipient".to_string(), Value::String(recipient));
        args.insert("amount".to_string(),    Value::Number(Number::from_f64(amount).unwrap()));
        args.insert("signature".to_string(), Value::String(signature));
        self.send_command(CommandPayload{
            command: "create_transaction".to_string(),
            args:    Some(args)
        })
    }


    /// ## Submit a problem solution
    ///
    /// Submit a solution for the current challenge, awarding CSCoins to the miner if the solution is valid.
    ///
    /// Command:   "submission"
    /// Arguments: wallet_id: String
    ///            nonce:     String
    /// Response:  SubmitProblem
    ///
    /// References: https://github.com/csgames/cscoins#submit-a-problem-solution
    pub fn submission(&mut self, wallet_id: String, nonce: String) -> Result<SubmitProblem, CSCoinClientError> {
        let mut args: Map<String, Value> = Map::new();
        args.insert("wallet_id".to_string(), Value::String(wallet_id));
        args.insert("nonce".to_string(),     Value::String(nonce));
        self.send_command(CommandPayload{
            command: "submission".to_string(),
            args:    Some(args)
        })
    }

    /// ## Get Central Authority Server Information
    ///
    /// Fetch the current information of the Central Authority server
    ///
    /// Command:   "ca_server_info"
    /// Arguments: none
    /// Response:  CAServerInfo
    ///
    /// References: https://github.com/csgames/cscoins#get-central-authority-server-information
    pub fn ca_server_info(&mut self) -> Result<CAServerInfo, CSCoinClientError> {
        self.send_command(CommandPayload{
            command: "submission".to_string(),
            args:    None
        })
    }

}

