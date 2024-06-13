# dwrench  
A cli tool to do various data related tasks.

Status: Work barely in progress

## Usage
### help
```
A prototype that sometimes come in handy

Usage: dwrench <COMMAND>

Commands:
  csvconvert  
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### csvconvert
```
Usage: dwrench csvconvert [OPTIONS] <TO> <INPUT_PATH> <OUTPUT_PATH>

Arguments:
  <TO>           [possible values: parquet]
  <INPUT_PATH>   
  <OUTPUT_PATH>  

Options:
  -s, --separator <SEPARATOR>                    
  -n, --normalize-colnames <NORMALIZE_COLNAMES>  
  -h, --help                                     Print help
```
