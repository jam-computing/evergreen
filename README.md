<div align="center">

# Evergreen 2

#### Christmas Tree Light Controller

</div>

* [What is Evergreen?](#-What-is-Evergreen)
* [Project Structure](#-Project-Structure)
* [Todo List](#-Todo-List)
* [Protocol Spec](#-Protocol-Spec)


## What is Evergreen?

Evergreen is the backend for the [Tree Lights Project]().

Pairs with [Willow] for gui communication

## Project Structure

- Front End / Animation Player ( React / Typeshit ) ~~ Joshua ?
- Server - Connects with lights and recvs tcp requests from frontend ~~ Jules ?
- Scanner - Standalone python script which scans tree and sends data to server through tcp ~~ Matthew ?

## Todo List

- [x] Structure Packet.
- [x] Create Specification for an Animation.
- [x] Create Front-end media player
- [x] Go + HTMX
- [x] Database here prolly
- [ ] Actually play the animations on the pi with ws281x rust bindings
- [ ] Add feautures to front end
- [ ] Create tree features
- [ ] Add tree to database
- [ ] Define specification for frames
- [ ] Animation should spawn new thread and loop
- [ ] Write a string to tree
- [ ] Fix tree light request message
- [ ] Fix naming issue with handle_*

## Language spec ideas

- [ ] Functions to make common ideas - plane etc
- [ ] Mathmatical support to modify the planes
- [ ] Tick rate support

## Protocol Spec

Name      No. Bytes   Description

- Version   1           The Protocol stinky Version
- ID        2           The unique identifier of the client sending data, typically unimportant apart from setup
- Command   1           Usually signifies the what data will be contained in a packet
- Status    2           The status code ( 200~ for success, 400~ for failure )
- Length    2           How long the data packet is
- Data      Variable    Json data about the packet

## *Command Reference:*

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
- LedCount  11
- ClearTree 12

## Frame List Spec

!TODO




### PACKET SPEC VERSION 2

###### Two packets this time !

* Packet One - Metadata
* Packet Two - Data

#### Packet One

- Version   | 1B
- ID        | 2B
- Command   | 1B
- Status    | 2B
- Length    | 2B

#### Packet Two ( Optional )

- ID | 2B
- Version | 1B
- Data | Variable Length
