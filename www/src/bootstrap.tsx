// エントリーポイント。wasm を import するモジュールは非同期で import する必要があるので、
// こうすることで、ソース内のどこでも wasm を import することができるようになる。
import("./index").catch(e => console.error("Error importing `bundle.js`:", e));
