# cal8tor • ***cal***endar P***8*** extrac***tor***
Extractor of the calendar of the IT degree of university Paris 8.

## Build and run
You will need Rust installed to compile the program.
<details><summary>You also need <code>OpenSSL</code> installed.</summary>

- Ubuntu: `sudo apt install libssl-dev`
- Fedora: `dnf install openssl-devel`
</details>

1. Clone the repo and get in
```bash
$ git clone https://git.kennel.ml/Anri/cal8tor.git && cd cal8tor
```
2. Build the app and get in the output folder
```bash
$ cargo build --release && cd target/release/
```
3. Run the app, here the help page will be displayed
```
$ ./cal8tor --help
```

## ***WIP:*** See the calendar in your terminal
For the L2-X, run:
```bash
$ ./cal8tor l2-X
```

## Export the calendar in .ics format
For the L1-A, run:
```bash
$ ./cal8tor L1A --export calendar.ics
```

> The file include the timezone for `Europe/Paris` and is in
compliance with [this validator tool](https://icalendar.org/validator.html).

---
Please open a PR if you want to improve the project!
