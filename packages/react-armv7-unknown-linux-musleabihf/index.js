function execute() {
  return require("child_process").execSync("react-cli");
}

module.exports = {
  execute
};
