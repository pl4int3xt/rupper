# Rupper
Introducing "Rupper": Your password's worst nightmare in Rust! Unleash the relentless power of code-cracking with a dash of Rustic humor. Get ready to make those forgotten passwords regret their life choices!💥🔐
# Arguments

```bash
./rupper <algorithm_type> <hashed_password> <password_file_location> <verbose>

algorithm type = ["sha1","sha256","md2","md4","md5"]
verbose = ["true", "false"]
```

# Crack a password

```bash
./rupper sha256 a50bf766b1dd34c060555480fbef2a9f185f56e2044cacc6c0227de3b7878f2a /usr/share/dict/rockyou.txt false
     
 ██████╗  ██╗   ██╗  ██████╗   ██████╗  ███████╗  ██████╗ 
 ██╔══██╗ ██║   ██║  ██╔══██╗  ██╔══██╗ ██╔════╝  ██╔══██╗
 ██████╔╝ ██║   ██║  ██████╔╝  ██████╔╝ █████╗    ██████╔╝
 ██╔══██╗ ██║   ██║  ██╔═══╝   ██╔═══╝  ██╔══╝    ██╔══██╗
 ██║  ██║ ╚██████╔╝  ██║       ██║      ███████╗  ██║  ██║
 ╚═╝  ╚═╝  ╚═════╝   ╚═╝       ╚═╝      ╚══════╝  ╚═╝  ╚═╝

 -------------------
| Author: <Myst3ry> |
 -------------------

Attempting to crack a50bf766b1dd34c060555480fbef2a9f185f56e2044cacc6c0227de3b7878f2a

Password found

-----------------------------------------------------------------------------------
Attempts [98191]

Password [Tigger2]

Hash [a50bf766b1dd34c060555480fbef2a9f185f56e2044cacc6c0227de3b7878f2a]
-----------------------------------------------------------------------------------
```
