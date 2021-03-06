const getFlags = async () => {
  const meow = await import("meow");
  const cli = meow.default(
    `
			Usage
				$ npx @mbecker20/monitor-cli
			Options
				--core, -c  setup monitor core
				--periphery, -p  setup monitor periphery
        --restart, -r  restart monitor
        --mongo-url  the url of mongo used with monitor (used with restart)
        --pull-latest  whether to pull latest monitor core image (used with restart)
        --restart-default  restart monitor with defaults
			Examples
				$ npx @mbecker20/monitor-cli --core
		`,
    {
      importMeta: import.meta,
      flags: {
        core: {
          type: "boolean",
          alias: "-c",
        },
        periphery: {
          type: "boolean",
          alias: "-p",
        },
        restart: {
          type: "boolean",
          alias: "-r",
        },
        mongoUrl: {
          type: "string",
        },
        pullLatest: {
          type: "boolean",
        },
        restartDefault: {
          type: "boolean",
        },
      },
    }
  );
  return cli.flags
};

export default getFlags;
