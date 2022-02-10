# Network Outline

* We have a central server, who keeps in-memory records of the messages

* Each client will connect to the server and identify themselves
  * If this is the client's first time connecting to the server, it fails authentication so the server sends it a certificate, then kicks the client. Client will reconnect with the certificate 
* Whenever a client has a message, it sends that message to the server
* When the server receives a message, it "broadcasts" that message to each connected client, who will update their respective GUIs to reflect it

# Data packet design
The packet will be a serialized JSON structure. That way we can very easily convert it into a Rust structure and handle it from there with minimal parsing and overhead. The structure will look as follows:

```json
{
    "user": "username",
    "type": "MESSAGE_TYPE",
    "message": "The message goes here. "
}
```

## `MESSAGE_TYPE` enum
There will be two types of messages: text messages (`TEXT`) and file messages (`FILE`). Text messages will be broadcast to each client connected to the server, while file messages will generate a text message alerting users to the file's presence. Given that the file transfer goes on without the server storing the information, it cannot hold onto the file's content.


# Planned features
1. file upload limit disregard 
    -> server negotiates the p2p sharing of a file



# Todos:
1. Client should be driven by the GUI, not the other way around