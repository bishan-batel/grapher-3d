# IA Criterion C Part 0

## Directories

The `/server` directory contains a netbeans project for the HTTP server that runs the website, the only thing you need to do to run the server is to have the [Javascript / Rust website built for deployment](#how-to-build-the-website), as well as linking the single Derby.jar library in netbeans (if not already). When you run it, simply head to [localhost:3000](http://localhost:3000)

The `/client` directory contains a NPM react-typescript project that contains everything related to the actualy general dispaly of the website (excluding the graph itself)

The `/lib` directory contains a rust crate (project) that is loaded by the react project as a WASM (web-assembly) package to run in the browser. The rust crate is responsible for parsing graph equations on the website, as well as displaying thtem using the browers's native WebGL2 API (excluded on Safari, Internet Explorer, or any legacy browser)

<div style="page-break-after: always;"></div>

## How to build the Website

### 1. Have Docker installed

Docker is a program for servers to be be able to reproduce enviorments efficiently, in where you have ea single file (the 'Dockerfile') that describes the enviorment the server needs to run (ex. Ubuntu with JDK 17). The java portion of my program does not need Docker, as all you need is Netbeans & JDK 11, but because of the tools I used to build the website (Rust, Typescript, NodeJs, and Web-Sys) it is easier for both you (The Don), as well as the ~~idiots~~ respectful IB moderators ( :] ) to only have to download Docker to get these dependicies to run the program rather than managing downloading these other things.
> <https://docs.docker.com/desktop/mac/install/>

### 2. Be able to run Docker from command line

This process should be automatic after installing Docker, if it isn't though then I have no idea what to do (sorry).

```bash
docker -v
```

### 3. Open a terminal and CD into the downloaded directory

```bash
cd path/to/project/directory
```

### 4. Build the docker image

| :zap:        Not neccessary if you have already ran this step, building multiple times has no effect unless you want to change the Dockerfile for some reason   |
|-----------------------------------------|

```bash
./setup.sh

# OR 
# if permission is denied you may have to give 
# executable permission to the script
chmod +x ./setup.sh
./setup.sh
# (you do not need to run the above 2 commands if it worked the first time)
```

### 5. Build the Website

If you ever change the source code, you will need to run this command again to build it for the server to recognize.

__NOTE__: You do not need to restart the Java server if you rebuild the website source

```bash
./build_js.sh

# OR 
# if permission is denied you may have to give 
# executable permission to the script
chmod +x ./build_js.sh
./build_js.sh
# (you do not need to run the above 2 commands if it worked the first time)
```

------

After these steps, there should be a created folder in `/server/src/` named "js-build"

Once you've confirmed that the folder exists, simply open up the `/server` directory in netbeans and run the project.
