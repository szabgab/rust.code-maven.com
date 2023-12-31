
# Release

```
cargo test
cargo build --release
```

We have a server nicknamed `s7` and in my home directory there is a folder called `rust`

```
scp target/release/phonebook-rocket-surrealdb s7:rust/
```

```
sudo ln -s /home/gabor/rust/phonebook.service  /usr/lib/systemd/system/phonebook.service
```

```
sudo systemctl daemon-reload
```

```
sudo systemctl restart phonebook.service
```

```
curl http://localhost:8000
```

