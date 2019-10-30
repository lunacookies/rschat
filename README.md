# rschat

`rschat` is a toy unencrypted public chat room thing I created in Rust to help me learn the language. Use `rschat-server [IPV4 ADDR:PORT]` to start a server at the specified location, and then use `rschat-sender [SAME THING]` to send messages and `rschat-viewer [SAME THING]` to view messages. I split the viewer and sender in two to avoid having to deal with async, which significantly simplifies the implementation. Since this is my fist ‘proper’ project in Rust the code is definitely suboptimal, but I learned a lot in the process.
