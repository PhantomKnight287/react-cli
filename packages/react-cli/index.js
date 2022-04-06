#!/usr/bin/env node
const { familySync: libcName } = require("detect-libc");
const os = require("os");

const architectures = {
  arm: "armv7",
  aarch64: "arm64",
  ia32: "i386",
  x64: "x86_64"
};

const osMap = {
  win32: "pc-windows-msvc",
  darwin: "apple-darwin",
  linux: "unknown-linux"
};

const platform = os.platform();
let rustPlatform = "";

switch (platform) {
  default: {
    rustPlatform = `${architectures[os.arch()]}-${osMap[platform]}`;
    break;
  }
  case "linux": {
    const libc = libcName();
    const libcPlatform = os.arch() === "arm" ? `${libc}eabi` : libc;
    rustPlatform = `${architectures[os.arch()]}-${
      osMap[platform]
    }-${libcPlatform}`;
  }
}
require(`@react-cli/react-${rustPlatform}`);
