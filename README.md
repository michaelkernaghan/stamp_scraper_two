# Stamp Scraper Project

## Overview
This project is a Rust-based web scraper designed for extracting stamp information from online auction pages. It focuses on parsing specific details about stamps, such as country names and stamp numbers, particularly for stamps issued before 1941.

## Features
- Fetch and parse HTML content from stamp auction sites.
- Extract and format stamp details, including country names and stamp numbers.
- Filter data based on specific criteria such as year and excluded keywords.

## Usage
To run the scraper:
1. Ensure Rust is installed on your system.
2. Clone the repository to your local machine.
3. Navigate to the project directory and run `cargo run`.

## Dependencies
- `reqwest` for HTTP requests.
- `scraper` for HTML parsing.
- `regex` for regular expression processing.

## Contributing
Contributions to this project are welcome! Please send pull requests or open an issue if you have suggestions or improvements.

## License
This project is licensed under [MIT License](LICENSE).

## Disclaimer
This tool is intended for personal use and educational purposes. Please adhere to the website's terms of use when scraping data.
