## COHLE
Rust CLI that prints Rust Cohle quotes.

![Alt text](image.png)

**Install:**
```console
cargo install --git=https://github.com/ShettySach/cohle
```

**Usage**: 
```console
cohle.exe [OPTIONS] [quote_index] [COMMAND]
```

**Commands**:
*  list<br>
 Lists all the quotes along with their indices [aliases: l]<br>
* quote<br>
 Print only quote without image [aliases: q]
* image<br>
 Print only image without quote [aliases: i]<br>
* help<br>
 Print this message or the help of the given subcommand(s)<br>

**Arguments**:
 * [quote_index]
 <br>Index of the quote (Optional)

**Options**:
*  -c, --col <colour><br>
Use 'cohle -c <colour>' to print quote in colour
*  -b, --bg<br>
Print image with black background [Better for light coloured terminals]
* -h, --help
 <br>Print help
* -V, --version<br>
Print version

**Colours:**
* r | red
* b | blue
* y | yellow
* g | green
* m | magenta
* c | cyan
* dr | darkred
* db | darkblue
* dy | darkyellow
* dg | darkgreen
* dm | darkmagenta
* dc | darkcyan
* bk | black
