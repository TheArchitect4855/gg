# gg

`gg` is the backend service for the Gravity Gunners video game.

## Installation

1. Download the release binary for your platform.
2. [Install sqlite3 CLI tools](https://sqlite.org/download.html).
3. In the same directory as the `gg` executable, run `sqlite3 database.sqlite < schema.sql`.
4. Add your server address to the `server_addresses` table in the database.
5. Create a file named `start-server.sh`. This should be a shell script that starts the Gravity Gunners game server. Be sure to make it executable.
6. Open port 80 for the `gg` executable.

## Contributing

Please open an issue for any bugs you find or feature requests you have. Pull requests are not accepted from the public at this time.

## License

[MIT](https://choosealicense.com/licenses/mit/)

