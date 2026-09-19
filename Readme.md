### hashcheck
built to hash with sha256 and check sha256 hash output, recreating sha256sum behavior with rust

### Usage
You can use the hashing program with 1 file or more
```bash
hashcheck --file <file> [file...]
```
to verify a hash the format of the file should be the same as sha256sum
```
hashcheck --check <file>
```
### Verify
to verify hashing functionality output against sha256sum
```bash
sha256sum -f <FILE_TO_HASH>
```
to verify hash check functionality output against sha256sum
```bash
sha256sum -c <OUTPUT_IN_A_FILE>
```
### Build
You can build the project with cargo
```
cargo build --release
```