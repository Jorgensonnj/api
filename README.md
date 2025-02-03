## About

A personal project written in rust. API uses the actix web framework and a sensible folder structure to separate front-end and back-end activities.

Although this project is a monorepo its structure is such that teams can asynchronously work in their respective domains and modules without conflicts arising.

## Getting Started

To get the application up and running follow these simple example steps.

### Prerequisites

Install dependencies using the following methods.
* Cargo
* Optional: Postgresql database

### Installation

1. Clone the repo
   ```sh
   git clone git@github.com:Jorgensonnj/api.git
   ```
2. Install cargo
   ```sh
   sudo pacman -S cargo
   ```
   or
   ```sh
   sudo apt-get install cargo
   ```
3. Compile application
   ```sh
   cd /path/to/api && cargo run
   ```

## Usage

Once application is running, test functionality

* Check application's health
  ```sh
  curl "http://localhost:8080/status"
  ```



