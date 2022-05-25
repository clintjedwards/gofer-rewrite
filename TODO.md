- For events we should change our approach. We can provide people the ability to parse them via json
  and possibly have sdk helpers, but creating different types is not scaleable. Instead pass type in as an enum.

* Possibly make the comfy-tables crate respect NO_COLOR
* Clean up the server service error chain. This should return a clean error back to the caller on why
  the server could not start. (start_service, get_tls_config)
  Figure out how to multiplex and enable the use of services so we can add back in reflection.
