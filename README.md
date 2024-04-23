# Evergreen 2

* Tree Lights Project.
* New and improved
* Better than ever.
* Rust.
* Bam.

## What is Evergreen?

Advanced software to control lights on a christmas tree.
Spotify-Like UI.

## Project Structure

- Front End / Animation Player ( React / Typeshit ) ~~ Joshua ?
- Server - Connects with lights and recvs tcp requests from frontend ~~ Jules ?
- Scanner - Standalone python script which scans tree and sends data to server through tcp ~~ Matthew ?

## Todo List

[] Structure Packet.
[] Create Specification for an Animation.
[] Create Front-end media player {
    * React - Next.js
    * Database here prolly
    * Send requests to server to play
}
[] Actually play the animations on the pi with ws281x rust bindings

# *Protocol Spec:*

Name      No. Bytes   Description

- Version   1           The Protocol Version
- Command   1           Usually signifies the what data will be contained in a packet
- Status    2           The status code ( 200~ for success, 400~ for failure )
- ID        2           The unique identifier of the client sending data, typically unimportant apart from setup
- Length    2           How long the data packet is
- Data      Variable    Json data about the packet

# *Command Reference:*

Name      Number

- None      0
- Init      1
- Play      2
- Pause     3
- Get       4
- New       5
- On        6
- Off       7
- OnRange   8
- OffRange  9
- TreeData  10
