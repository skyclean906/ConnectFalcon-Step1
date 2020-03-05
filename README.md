# ConnectFalcon-Step1

#### Development

1. Build
    - On Window:
        - Install OpenSSL via https://slproweb.com/products/Win32OpenSSL.html.

        - Check and Modify OpenSSL install path in build.rs.

            ```
            .include("$INSTALL_PATH/include")
            ```
            ```
            println!("cargo:rustc-link-search=static=$INSTALL_PATH/
            lib");
            ```
            
        - Run "cargo build" in console.

2. Run
    - On Window:
        > Run "cargo run" in console.
