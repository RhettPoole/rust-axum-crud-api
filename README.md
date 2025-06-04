## Overview

**Project Title**: Rust-CRUD-API

**Project Description**: Simple CRUD operations API written in Rust, using the AXUM web framework, tested using Postman. The majority of this code is built using this guide: https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
    - Much of the code was outdated and I had to troubleshoot the code after finishing the guide. As I have never worked with Rust, I felt comfortable using a guide to code and then review what the function means and why we are structuring it. You'll notice in my commit history there are many more comments explaining syntax and built in keywords in Rust and Axum. As I progressed through the guide I started to remember what many of the words and syntax meant, so I stopped adding syntax comments.
    - I fed prompts to my CoPilot during code reviews such as: "Let's go line by line and review this code. I need to know exactly what that line is doing and why it is integral to our program. Assume that I know nothing about servers, API's, or Rust. I'm trying to understand this program as deeply as possible."

**Project Goals**: Learn how to write an API using Rust. See differences in API's in different languages. This will be my first time ever using Rust. I'm assuming the majority of my time will be spent studying and finding what syntax to use as I build the project.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Rust-analyzer extension in VSCode
2. Install windows pre-requisites. https://rust-lang.github.io/rustup/installation/windows-msvc.html
3. Install Rust https://rustup.rs/ 
4. Install VS C++ tools https://visualstudio.microsoft.com/visual-cpp-build-tools/ 
5. Use 'cargo run' to launch local server, then move to Postman for testing.

Instructions for using the software:

1. Use 'cargo run' to launch local server, then move to Postman for testing.
2. Test each Post, Get, Patch, Del function in Postman.
3. Terminate server

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Same steps as 'Instructions for Build and Use'

## Useful Websites to Learn More

I found these websites useful in developing this software:

* https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
* https://github.com/wpcodevo/simple-api-rust-axum 
* https://www.youtube.com/watch?v=qbLc5a9jdXo 
* https://www.geeksforgeeks.org/introduction-to-rust-programming-language/ 
* https://docs.rs/axum/latest/axum/ 

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* There is no front-end to this project, in the future I could add that.