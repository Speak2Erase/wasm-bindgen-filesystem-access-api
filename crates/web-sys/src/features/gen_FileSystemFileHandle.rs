#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = FileSystemHandle , extends = :: js_sys :: Object , js_name = FileSystemFileHandle , typescript_type = "FileSystemFileHandle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type FileSystemFileHandle;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemFileHandle" , js_name = createSyncAccessHandle)]
    #[doc = "The `createSyncAccessHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createSyncAccessHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_sync_access_handle(
        this: &FileSystemFileHandle,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemFileHandle" , js_name = createWritable)]
    #[doc = "The `createWritable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_writable(this: &FileSystemFileHandle) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemCreateWritableOptions")]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemFileHandle" , js_name = createWritable)]
    #[doc = "The `createWritable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`, `FileSystemFileHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_writable_with_options(
        this: &FileSystemFileHandle,
        options: &FileSystemCreateWritableOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (catch , method , structural , js_class = "FileSystemFileHandle" , js_name = getFile)]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_file(this: &FileSystemFileHandle) -> Result<::js_sys::Promise, JsValue>;
}
