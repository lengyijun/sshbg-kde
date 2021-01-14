```
$ cat ~/.ssh/config
PermitLocalCommand yes
Host *
  LocalCommand ~/sshbg-kde/target/release/sshbg-kde "%n"
```

```
# test your terminal
echo -e "\033]11;#007F00\a"
```
