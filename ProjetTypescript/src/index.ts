import init, { call_csharp_code, fetch_data } from "../pkg/interop_csharp_ts";

// https://medium.com/@mtolmacs/a-gentle-introduction-to-webassembly-in-rust-2025-edition-c1b676515c2d

async function run() {
  await init();
  console.log(call_csharp_code());
  console.log("heo");

  fetch_data();
}

run();
