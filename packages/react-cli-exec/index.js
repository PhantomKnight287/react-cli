const { spawnSync } = require("child_process");

function execute(dir) {
  spawnSync(`${dir}/react`, process.argv.slice(2), {
    stdio: "inherit"
  });
}

module.exports = { execute };
