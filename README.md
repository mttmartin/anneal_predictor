## anneal_predictor

This is an example of a novel(but naive) DNA search method that does not use the primary DNA sequence. Instead, it takes a small search sequence and genome and searches for energy wells. This approach can be significantly more accurate in predicting DNA annealing than primary sequence alone.


## Dependencies

You will need Rust in order to build this project. You can acquire it with [rustup](https://www.rustup.rs/) like so:

- On Linux and macOS:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

## Building from source

```bash
$ git clone https://github.com/mttmartin/anneal_predictor
$ cd anneal_predictor
$ cargo build --release
```

After building, you will find an executable in the ./target/release directory. 

