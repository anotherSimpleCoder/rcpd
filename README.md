<h1 align="center">rcpd</h1>
<h3 align="center">The receiving end of <a href="https://github.com/anotherSimpleCoder/rcp">rcp</a>.</h3>

This is basically the sshd of rcp. In order to start rcpd, you need the create a config file.

## Configure rcpd

rcpd can be configured with a simple JSON file called ``config.json``:

````json
{
  "port": <port to listen to>,
  "out_path": "<path for storing files>"
}
````