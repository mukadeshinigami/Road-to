def connet ( 
    host: str,
    port: int,
    *tags: str,
    timeout: float = 5.0,
    ssl: bool = False,
    **options: object,
) -> None:
    """Connect to a host on a port."""
    print(f"Connecting to {host} on port {port}")
    print(f"Tags: {tags}")
    print(f"Timeout: {timeout}")
    print(f"SSL: {ssl}")
    print(f"Options: {options}")
connet("localhost", 5432, "read-replica", timeout=5.0, ssl=True, retries=3  )
