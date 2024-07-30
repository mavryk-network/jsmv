use boa_engine::Context;
use jsmv_api::js_log::{JsLog, LogData};
use jsmv_core::runtime::with_global_host;
use mavryk_smart_rollup::prelude::debug_msg;

pub(crate) struct PrettyLogger;

impl JsLog for PrettyLogger {
    fn log(&self, log_data: LogData, _context: &mut Context<'_>) {
        let LogData {
            level,
            text,
            groups_len,
        } = log_data;

        let indent = 2 * groups_len;
        let symbol = level.symbol();
        with_global_host(|rt| {
            for line in text.lines() {
                debug_msg!(rt, "[{symbol}] {:>indent$}{line}\n", "");
            }
        });
    }
}
