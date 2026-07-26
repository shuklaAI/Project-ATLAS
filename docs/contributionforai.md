When contributing to Atlas:

- Never rewrite modules unless necessary.
- Preserve public APIs whenever possible.
- Use Tokio for networking.
- Keep atlas-sdk platform-independent.
- Desktop and Android should contain almost no networking logic.
- Scanner must remain modular.
- Discovery only finds Atlas nodes.
- Scanner finds all LAN devices.
- Bluetooth is a separate discovery mechanism.
- Avoid placeholder implementations.
- Production-quality Rust only.
- Include unit tests where practical.
- Explain why new dependencies are introduced.