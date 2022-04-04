const { spawnSync } = require("child_process");
const os = require("os");

function execute(dir) {
  spawnSync(
    `${dir}/react${os.platform() == "win32" ? ".exe" : ""}`,
    process.argv.slice(2),
    {
      stdio: "inherit"
    }
  );
}

module.exports = { execute };
