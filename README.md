# Ferris' Static Files

Ferris' Static Files (FSF) is a simple static file server written in Rust.
It is designed to serve static files of Single Page Applications (SPA)
that have a single entry point (e.g. `index.html`).

FSF is a handy tool when you:

- are not on the "cloud"
- are not terminating TLS
- don't need Server Side Rendering (SSR)
- and don't want to write NGINX configuration files for your SPAs.

## Usage

```
Usage: fsf.exe [OPTIONS] [path]                                                                                                                                      
                                                                                                                                                                     
Arguments:                                                                                                                                                           
  [path]  path to serve [default: .]                                                                                                                                 
                                                                                                                                                                     
Options:                                                                                                                                                             
  -b, --bind <addr>    bind address [default: 0.0.0.0:3000]                                                                                                          
  -i, --index <file>   fallback file to serve (relative to path) [default: index.html]                                                                               
      --prefix <path>  prefix to strip from URL path (must start with '/' and not end with '/'). E.g.: `--prefix /app` will serve `./index.html` as `/app/index.html`
  -h, --help           Print help                                                                                                                                    
  -V, --version        Print version
```

## License

This project is licensed under the [MIT License](./LICENSE).
