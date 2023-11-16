/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::ToOwned;
use std::collections::HashMap;

use embedder_traits::resources::{self, Resource};
use gen::Prefs;
use lazy_static::lazy_static;
use serde_json::{self, Value};

use crate::pref_util::Preferences;
pub use crate::pref_util::{PrefError, PrefValue};

lazy_static! {
    static ref PREFS: Preferences<'static, Prefs> = {
        let def_prefs: Prefs = serde_json::from_str(&resources::read_string(Resource::Preferences))
            .expect("Failed to initialize config preferences.");
        Preferences::new(def_prefs, &gen::PREF_ACCESSORS)
    };
}

/// A convenience macro for accessing a preference value using its static path.
/// Passing an invalid path is a compile-time error.
#[macro_export]
macro_rules! pref {
    ($($segment: ident).+) => {{
        let values = $crate::prefs::pref_map().values();
        let lock = values.read()
            .map(|prefs| prefs $(.$segment)+.clone());
        lock.unwrap()
    }};
}

/// A convenience macro for updating a preference value using its static path.
/// Passing an invalid path is a compile-time error.
#[macro_export]
macro_rules! set_pref {
    ($($segment: ident).+, $value: expr) => {{
        let values = $crate::prefs::pref_map().values();
        let mut lock = values.write().unwrap();
        lock$ (.$segment)+ = $value;
    }};
}

/// Access preferences using their `String` keys. Note that the key may be different from the
/// static path because legacy keys contain hyphens, or because a preference name has been renamed.
///
/// When retrieving a preference, the value will always be a `PrefValue`. When setting a value, it
/// may be a `PrefValue` or the type that converts into the correct underlying value; one of `bool`,
/// `i64`, `f64` or `String`.
#[inline]
pub fn pref_map() -> &'static Preferences<'static, Prefs> {
    &PREFS
}

pub fn add_user_prefs(prefs: HashMap<String, PrefValue>) {
    if let Err(error) = PREFS.set_all(prefs.into_iter()) {
        panic!("Error setting preference: {:?}", error);
    }
}

pub fn read_prefs_map(txt: &str) -> Result<HashMap<String, PrefValue>, PrefError> {
    let prefs: HashMap<String, Value> =
        serde_json::from_str(txt).map_err(|e| PrefError::JsonParseErr(e))?;
    prefs
        .into_iter()
        .map(|(k, pref_value)| {
            Ok({
                let v = match &pref_value {
                    Value::Bool(b) => PrefValue::Bool(*b),
                    Value::Number(n) if n.is_i64() => PrefValue::Int(n.as_i64().unwrap()),
                    Value::Number(n) if n.is_f64() => PrefValue::Float(n.as_f64().unwrap()),
                    Value::String(s) => PrefValue::Str(s.to_owned()),
                    Value::Array(v) => {
                        let mut array = v.iter().map(|v| PrefValue::from_json_value(v));
                        if array.all(|v| v.is_some()) {
                            PrefValue::Array(array.flatten().collect())
                        } else {
                            return Err(PrefError::InvalidValue(format!(
                                "Invalid value: {}",
                                pref_value
                            )));
                        }
                    },
                    _ => {
                        return Err(PrefError::InvalidValue(format!(
                            "Invalid value: {}",
                            pref_value
                        )));
                    },
                };
                (k.to_owned(), v)
            })
        })
        .collect()
}

mod gen {
    use serde::{Deserialize, Serialize};
    use servo_config_plugins::build_structs;

    // The number of layout threads is calculated if it is not present in `prefs.json`.
    fn default_layout_threads() -> i64 {
        std::cmp::max(num_cpus::get() * 3 / 4, 1) as i64
    }

    fn black() -> i64 {
        0x000000
    }

    fn white() -> i64 {
        0xFFFFFF
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Pref {
        browser: Browser,
        css: Css,
        devtools: Devtools,
        dom: Dom,
        gfx: Gfx,
        js: Js,
        layout: Layout,
        media: Media,
        network: Network,
        session_history: SessionHistory,
        shell: Shell,
        webgl: Webgl,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Browser {
        display: Display,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Display {
        #[serde(default = "white")]
        background_color: i64,
        #[serde(default = "black")]
        foreground_color: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Css {
        animations: Animations,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Animations {
        testing: Testing,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Testing {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Devtools {
        server: Server,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Server {
        enabled: bool,
        port: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Dom {
        webgpu: Webgpu,
        bluetooth: Bluetooth,
        canvas_capture: CanvasCapture,
        canvas_text: CanvasText,
        composition_event: CompositionEvent,
        custom_elements: CustomElements,
        document: Document,
        forcetouch: Forcetouch,
        fullscreen: Fullscreen,
        gamepad: Gamepad,
        imagebitmap: Imagebitmap,
        microdata: Microdata,
        mouse_event: MouseEvent,
        mutation_observer: MutationObserver,
        offscreen_canvas: OffscreenCanvas,
        permissions: Permissions,
        script: Script,
        serviceworker: Serviceworker,
        servo_helpers: ServoHelpers,
        servoparser: Servoparser,
        shadowdom: Shadowdom,
        svg: Svg,
        testable_crash: TestableCrash,
        testbinding: Testbinding,
        testing: TestingDom,
        testperf: Testperf,
        webgl2: Webgl2,
        webrtc: Webrtc,
        webvtt: Webvtt,
        webxr: Webxr,
        worklet: Worklet,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Webgpu {
        /// Enable WebGPU APIs.
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Bluetooth {
        enabled: bool,
        testing: BluetoothTesting,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct BluetoothTesting {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CanvasCapture {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CanvasText {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CompositionEvent {
        #[serde(rename = "dom.compositionevent.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CustomElements {
        #[serde(rename = "dom.customelements.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Document {
        dblclick_timeout: i64,
        dblclick_dist: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Forcetouch {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Fullscreen {
        test: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Gamepad {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Imagebitmap {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Microdata {
        testing: TestingMicrodata,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestingMicrodata {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct MouseEvent {
        which: Which,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Which {
        #[serde(rename = "dom.mouseevent.which.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct MutationObserver {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct OffscreenCanvas {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Permissions {
        enabled: bool,
        testing: TestingPermissions,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestingPermissions {
        allowed_in_nonsecure_contexts: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Script {
        asynch: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Serviceworker {
        enabled: bool,
        timeout_seconds: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct ServoHelpers {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Servoparser {
        async_html_tokenizer: AsyncHtmlTokenizer,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct AsyncHtmlTokenizer {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Shadowdom {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Svg {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestableCrash {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Testbinding {
        enabled: bool,
        prefcontrolled: Prefcontrolled,
        prefcontrolled2: Prefcontrolled2,
        preference_value: PreferenceValue,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Prefcontrolled {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Prefcontrolled2 {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct PreferenceValue {
        #[serde(default)]
        falsy: bool,
        #[serde(default)]
        quote_string_test: String,
        #[serde(default)]
        space_string_test: String,
        #[serde(default)]
        string_empty: String,
        #[serde(default)]
        string_test: String,
        #[serde(default)]
        truthy: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestingDom {
        element: Element,
        html_input_element: HtmlInputElement,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Element {
        activation: Activation,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Activation {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct HtmlInputElement {
        select_files: SelectFiles,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct SelectFiles {
        #[serde(rename = "dom.testing.htmlinputelement.select_files.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Testperf {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Webgl2 {
        /// Enable WebGL2 APIs.
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Webrtc {
        transceiver: Transceiver,
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Transceiver {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Webvtt {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Webxr {
        #[serde(default)]
        enabled: bool,
        #[serde(default)]
        test: bool,
        first_person_observer_view: bool,
        glwindow: Glwindow,
        hands: Hands,
        layers: Layers,
        sessionavailable: bool,
        #[serde(rename = "dom.webxr.unsafe-assume-user-intent")]
        unsafe_assume_user_intent: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Glwindow {
        #[serde(default)]
        enabled: bool,
        #[serde(rename = "dom.webxr.glwindow.left-right")]
        left_right: bool,
        #[serde(rename = "dom.webxr.glwindow.red-cyan")]
        red_cyan: bool,
        spherical: bool,
        cubemap: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Hands {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Layers {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Worklet {
        blockingsleep: Blockingsleep,
        #[serde(default)]
        enabled: bool,
        testing: TestingWorklet,
        timeout_ms: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Blockingsleep {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestingWorklet {
        #[serde(default)]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Gfx {
        subpixel_text_antialiasing: SubpixelTextAntialiasing,
        texture_swizzling: TextureSwizzling,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct SubpixelTextAntialiasing {
        #[serde(rename = "gfx.subpixel-text-antialiasing.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TextureSwizzling {
        #[serde(rename = "gfx.texture-swizzling.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Js {
        asmjs: Asmjs,
        asyncstack: Asyncstack,
        baseline: Baseline,
        discard_system_source: DiscardSystemSource,
        dump_stack_on_debuggee_would_run: DumpStackOnDebuggeeWouldRun,
        ion: Ion,
        mem: Mem,
        native_regex: NativeRegex,
        offthread_compilation: OffthreadCompilation,
        parallel_parsing: ParallelParsing,
        shared_memory: SharedMemory,
        strict: Strict,
        throw_on_asmjs_validation_failure: ThrowOnAsmjsValidationFailure,
        throw_on_debuggee_would_run: ThrowOnDebuggeeWouldRun,
        timers: Timers,
        wasm: Wasm,
        werror: Werror,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Asmjs {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Asyncstack {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Baseline {
        enabled: bool,
        unsafe_eager_compilation: UnsafeEagerCompilation,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct UnsafeEagerCompilation {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct DiscardSystemSource {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct DumpStackOnDebuggeeWouldRun {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Ion {
        enabled: bool,
        offthread_compilation: OffthreadCompilationIon,
        unsafe_eager_compilation: UnsafeEagerCompilationIon,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct OffthreadCompilationIon {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct UnsafeEagerCompilationIon {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Mem {
        gc: Gc,
        max: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Gc {
        allocation_threshold_mb: i64,
        allocation_threshold_factor: i64,
        allocation_threshold_avoid_interrupt_factor: i64,
        compacting: Compacting,
        decommit_threshold_mb: i64,
        dynamic_heap_growth: DynamicHeapGrowth,
        dynamic_mark_slice: DynamicMarkSlice,
        empty_chunk_count_max: i64,
        empty_chunk_count_min: i64,
        high_frequency_heap_growth_max: i64,
        high_frequency_heap_growth_min: i64,
        high_frequency_high_limit_mb: i64,
        high_frequency_low_limit_mb: i64,
        high_frequency_time_limit_ms: i64,
        incremental: Incremental,
        low_frequency_heap_growth: i64,
        per_zone: PerZone,
        zeal: Zeal,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Compacting {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct DynamicHeapGrowth {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct DynamicMarkSlice {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Incremental {
        enabled: bool,
        slice_ms: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct PerZone {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Zeal {
        frequency: i64,
        level: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct NativeRegex {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct OffthreadCompilation {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct ParallelParsing {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct SharedMemory {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Strict {
        debug: Debug,
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Debug {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct ThrowOnAsmjsValidationFailure {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct ThrowOnDebuggeeWouldRun {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Timers {
        minimum_duration: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Wasm {
        baseline: BaselineWasm,
        enabled: bool,
        ion: IonWasm,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct BaselineWasm {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct IonWasm {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Werror {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Layout {
        animations: AnimationsLayout,
        columns: Columns,
        flexbox: Flexbox,
        legacy_layout: bool,
        #[serde(default = "default_layout_threads")]
        threads: i64,
        writing_mode: WritingMode,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct AnimationsLayout {
        test: Test,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Test {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Columns {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Flexbox {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct WritingMode {
        #[serde(rename = "layout.writing-mode.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Media {
        glvideo: Glvideo,
        testing: TestingMedia,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Glvideo {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestingMedia {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Network {
        enforce_tls: EnforceTls,
        http_cache: HttpCache,
        mime: Mime,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct EnforceTls {
        enabled: bool,
        localhost: bool,
        onion: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct HttpCache {
        #[serde(rename = "network.http-cache.disabled")]
        disabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Mime {
        sniff: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct SessionHistory {
        #[serde(rename = "session-history.max-length")]
        max_length: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Shell {
        background_color: BackgroundColor,
        crash_reporter: CrashReporter,
        homepage: String,
        keep_screen_on: KeepScreenOn,
        #[serde(rename = "shell.native-orientation")]
        native_orientation: String,
        native_titlebar: NativeTitlebar,
        /// URL string of the search engine page (for example <https://google.com> or and <https://duckduckgo.com>.
        searchpage: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct BackgroundColor {
        /// The background color of shell's viewport. This will be used by OpenGL's `glClearColor`.
        #[serde(rename = "shell.background-color.rgba")]
        rgba: [f64; 4],
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CrashReporter {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct KeepScreenOn {
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct NativeTitlebar {
        /// Enable native window's titlebar and decorations.
        #[serde(rename = "shell.native-titlebar.enabled")]
        enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Webgl {
        testing: TestingWebgl,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TestingWebgl {
        context_creation_error: bool,
    }
}
