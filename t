How does this sound for a network
backend structure?


We have a central server, who keeps
records of the messages

Each client will connect to the server
and identify themselves

Whenever a client has a message, it 
sends that message to the server

When the server receives a message, it
"broadcasts" that message to each connected
client



Things to fix:
1. file upload limit disregard 
    -> server negotiates the p2p sharing of a file
