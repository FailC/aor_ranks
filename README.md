### Small command line tool to combine multiple art of rally players into one leaderboard.

work in progress..

Players receive points depending on their performance for all individual stages.

(Without daily/weekly challenges or bonus groups)

#### features:
- one single leaderboard with all combined points
- leaderboards for every single stage
- average points/stage player stats

uses the Leaderboards.txt game file:
```plaintext
~/.config/unity3d/Funselektor\ Labs/art\ of\ rally/cloud/Leaderboards.txt
```
```plaintext
"C:\Users\USERNAME\AppData\LocalLow\Funselektor Labs\art of rally\cloud\Leaderboards.txt"
```
generated leaderboards look the best when the player files are renamed to username.txt

#### project structure

create a user_files directory, and move player.txt files into it (directory can be any name)

```plaintext
aor_ranks/
├── src/
    ├── main.rs
    ├── lib.rs
    ├── game.rs
├── user_files/
    ├── name1.txt
    ├── name2.txt
    ├── name3.txt
```

run:
```bash
cargo run -- user_files
```
Leaderboard directory gets created:
```plaintext
aor_ranks/
├── src/
    ├── main.rs
    ├── ...
├── Leaderboards/
    ├── ranks
    ├── all_stages/
```

premade "testfiles/" directory included, run cargo with
```bash
cargo run -- testfiles
```

