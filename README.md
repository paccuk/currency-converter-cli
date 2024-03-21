# Currency Converter CLI Setup Guide

### Before running the Currency Converter CLI, make sure to follow next steps.

## Prerequisites
1. **Register on Free Currency API:** Sign up on [freecurrencyapi.com](https://freecurrencyapi.com/) to obtain an API key.

2. **Access API Key:** Once registered, navigate to the `Default Key` section on the [dashboard](https://app.freecurrencyapi.com/dashboard) and copy the API key.

3. **Set API Key:** Paste the copied API key into the `API_KEY` variable within the `.env` file of your project.

## Running the CLI
1. **Build Binary:** Execute the following command to build the binary:
    ```bash
    cargo build --release
    ```
2. **Execute Application:** Once the binary is built, run the application using:
    ```bash
    ./target/release/currency_converter_cli
    ```
Alternatively, you can run the application directly without building the binary separately:
```bash
cargo run --release
```

By following these steps, you can efficiently set up and run the Currency Converter CLI for seamless currency conversion.