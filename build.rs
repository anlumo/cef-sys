use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct ParseCallbacks();

impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
    fn int_macro(&self, _name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        Some(bindgen::callbacks::IntKind::I32)
    }
}

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");

    match target_os.as_ref().map(|x| &**x) {
        Ok("windows") => {
            println!("cargo:rustc-link-lib=cef_sandbox");
            println!("cargo:rustc-link-lib=libcef");
        },
        Ok("linux") => {
            println!("cargo:rustc-link-lib=cef");
            println!("cargo:rustc-link-lib=EGL");
            println!("cargo:rustc-link-lib=GLESv2");
            println!(r"cargo:rustc-link-search=../cef/Debug");
        }
        _ => {},
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .clang_arg("-I../cef")
        .whitelist_type("cef_life_span_handler_t")
        .whitelist_type("cef_app_t")
        .whitelist_type("cef_command_line_t")
        .whitelist_type("cef_resource_bundle_handler_t")
        .whitelist_type("cef_scheme_registrar_t")
        .whitelist_type("cef_resource_bundle_handler_t")
        .whitelist_type("cef_browser_process_handler_t")
        .whitelist_type("cef_render_process_handler_t")
        .whitelist_type("cef_main_args_t")
        .whitelist_function("cef_execute_process")
        .whitelist_type("cef_settings_t")
        .whitelist_function("cef_initialize")
        .whitelist_type("cef_window_t")
        .whitelist_type("cef_string_t")
        .whitelist_type("cef_string_userfree_utf16_t")
        .whitelist_function("cef_string_utf8_to_utf16")
        .whitelist_function("cef_string_userfree_utf16_alloc")
        .whitelist_function("cef_string_userfree_utf16_free")
        .whitelist_function("cef_string_utf16_to_lower")
        .whitelist_function("cef_string_utf16_to_upper")
        .whitelist_function("cef_string_utf16_set")
        .whitelist_function("cef_string_utf16_clear")
        .whitelist_function("cef_string_utf16_cmp")
        .whitelist_function("cef_string_utf16_to_utf8")
        .whitelist_function("cef_string_list_alloc")
        .whitelist_function("cef_string_list_size")
        .whitelist_function("cef_string_list_value")
        .whitelist_function("cef_string_list_append")
        .whitelist_function("cef_string_list_clear")
        .whitelist_function("cef_string_list_free")
        .whitelist_function("cef_string_list_copy")
        .whitelist_function("cef_string_userfree_utf16_alloc")
        .whitelist_function("cef_string_userfree_utf16_free")
        .whitelist_type("cef_browser_settings_t")
        .whitelist_type("cef_browser_host_t")
        .whitelist_type("cef_client_t")
        .whitelist_type("cef_audio_handler_t")
        .whitelist_type("cef_context_menu_handler_t")
        .whitelist_type("cef_dialog_handler_t")
        .whitelist_type("cef_display_handler_t")
        .whitelist_type("cef_download_handler_t")
        .whitelist_type("cef_drag_handler_t")
        .whitelist_type("cef_drag_data_t")
        .whitelist_type("cef_find_handler_t")
        .whitelist_type("cef_focus_handler_t")
        .whitelist_type("cef_jsdialog_handler_t")
        .whitelist_type("cef_keyboard_handler_t")
        .whitelist_type("cef_life_span_handler_t")
        .whitelist_type("cef_load_handler_t")
        .whitelist_type("cef_render_handler_t")
        .whitelist_type("cef_screen_info_t")
        .whitelist_type("cef_request_handler_t")
        .whitelist_type("cef_browser_t")
        .whitelist_type("cef_frame_t")
        .whitelist_type("cef_process_message_t")
        .whitelist_type("cef_accessibility_handler_t")
        .whitelist_type("cef_cursor_info_t")
        .whitelist_type("cef_v8context_t")
        .whitelist_type("cef_v8handler_t")
        .whitelist_type("cef_v8value_t")
        .whitelist_type("cef_v8accessor_t")
        .whitelist_type("cef_v8interceptor_t")
        .whitelist_type("cef_v8exception_t")
        .whitelist_type("cef_v8array_buffer_release_callback_t")
        .whitelist_type("cef_v8stack_trace_t")
        .whitelist_type("cef_v8stack_frame_t")
        .whitelist_function("cef_browser_host_create_browser")
        .whitelist_function("cef_browser_host_create_browser_sync")
        .whitelist_function("cef_run_message_loop")
        .whitelist_function("cef_quit_message_loop")
        .whitelist_function("cef_do_message_loop_work")
        .whitelist_function("cef_set_osmodal_loop")
        .whitelist_function("cef_shutdown")
        .whitelist_function("cef_enable_highdpi_support")
        .whitelist_function("cef_v8value_create_undefined")
        .whitelist_function("cef_v8value_create_null")
        .whitelist_function("cef_v8value_create_bool")
        .whitelist_function("cef_v8value_create_int")
        .whitelist_function("cef_v8value_create_uint")
        .whitelist_function("cef_v8value_create_double")
        .whitelist_function("cef_v8value_create_date")
        .whitelist_function("cef_v8value_create_string")
        .whitelist_function("cef_v8value_create_object")
        .whitelist_function("cef_v8value_create_array")
        .whitelist_function("cef_v8value_create_array_buffer")
        .whitelist_function("cef_v8value_create_function")
        .whitelist_function("cef_v8stack_trace_get_current")
        .whitelist_function("cef_register_extension")
        .whitelist_function("cef_command_line_create")
        .whitelist_function("cef_command_line_get_global")
        .whitelist_function("cef_string_map_alloc")
        .whitelist_function("cef_string_map_size")
        .whitelist_function("cef_string_map_find")
        .whitelist_function("cef_string_map_key")
        .whitelist_function("cef_string_map_value")
        .whitelist_function("cef_string_map_append")
        .whitelist_function("cef_string_map_clear")
        .whitelist_function("cef_string_map_free")
        .whitelist_function("cef_resource_bundle_get_global")
        .whitelist_type("cef_point_t")
        .whitelist_type("cef_rect_t")
        .whitelist_type("cef_size_t")
        .whitelist_type("cef_range_t")
        .whitelist_type("cef_insets_t")
        .whitelist_type("cef_scheme_options_t")
        .whitelist_type("cef_dictionary_value_t")
        .whitelist_function("cef_dictionary_value_create")
        .whitelist_type("cef_list_value_t")
        .whitelist_function("cef_list_value_create")
        .whitelist_type("cef_binary_value_t")
        .whitelist_function("cef_binary_value_create")
        .whitelist_type("cef_value_t")
        .whitelist_function("cef_value_create")
        .whitelist_type("cef_print_handler_t")
        .whitelist_type("cef_domnode_t")
        .whitelist_type("cef_domvisitor_t")
        .whitelist_type("cef_domdocument_t")
        .whitelist_type("cef_process_message_t")
        .whitelist_type("cef_string_visitor_t")
        .whitelist_type("cef_request_t")
        .whitelist_function("cef_request_create")
        .whitelist_type("cef_response_t")
        .whitelist_function("cef_response_create")
        .whitelist_type("cef_post_data_element_t")
        .whitelist_function("cef_post_data_element_create")
        .whitelist_type("cef_post_data_t")
        .whitelist_function("cef_post_data_create")
        .whitelist_type("cef_urlrequest_t")
        .whitelist_type("cef_urlrequest_client_t")
        .whitelist_function("cef_urlrequest_create")
        .whitelist_type("cef_request_context_t")
        .whitelist_function("cef_request_context_get_global_context")
        .whitelist_function("cef_request_context_create_context")
        .whitelist_function("cef_create_context_shared")
        .whitelist_type("cef_auth_callback_t")
        .whitelist_type("cef_v8context_t")
        .whitelist_type("cef_v8exception_t")
        .whitelist_type("cef_v8stack_trace_t")
        .whitelist_function("cef_string_multimap_alloc")
        .whitelist_function("cef_string_multimap_size")
        .whitelist_function("cef_string_multimap_find_count")
        .whitelist_function("cef_string_multimap_enumerate")
        .whitelist_function("cef_string_multimap_key")
        .whitelist_function("cef_string_multimap_value")
        .whitelist_function("cef_string_multimap_append")
        .whitelist_function("cef_string_multimap_clear")
        .whitelist_function("cef_string_multimap_free")
        .whitelist_type("cef_web_plugin_info_t")
        .whitelist_type("cef_web_plugin_info_visitor_t")
        .whitelist_type("cef_web_plugin_unstable_callback_t")
        .whitelist_type("cef_register_cdm_callback_t")
        .whitelist_function("cef_refresh_web_plugins")
        .whitelist_function("cef_unregister_internal_web_plugin")
        .whitelist_function("cef_register_web_plugin_crash")
        .whitelist_function("cef_is_web_plugin_unstable")
        .whitelist_function("cef_register_widevine_cdm")
        .whitelist_type("cef_cookie_t")
        .whitelist_type("cef_callback_t")
        .whitelist_type("cef_resource_skip_callback_t")
        .whitelist_type("cef_resource_read_callback_t")
        .whitelist_type("cef_request_callback_t")
        .whitelist_type("cef_request_context_handler_t")
        .whitelist_type("cef_request_context_settings_t")
        .whitelist_type("cef_resource_request_handler_t")
        .whitelist_type("cef_cookie_access_filter_t")
        .whitelist_type("cef_resource_handler_t")
        .whitelist_type("cef_response_filter_t")
        .whitelist_type("cef_time_t")
        .whitelist_type("cef_key_event_t")
        .whitelist_function("cef_time_to_doublet")
        .whitelist_function("cef_time_from_doublet")
        .whitelist_function("cef_time_now")
        .whitelist_function("cef_time_delta")
        .whitelist_function("cef_sandbox_info_create")
        .whitelist_function("cef_sandbox_info_destroy")
        .whitelist_type("cef_run_file_dialog_callback_t")
        .whitelist_type("cef_image_t")
        .whitelist_type("cef_navigation_entry_t")
        .whitelist_function("cef_drag_data_create")
        .whitelist_var("IDR_.*")
        .whitelist_var(".*_JS")
        .whitelist_var(".*_JS_2")
        .whitelist_var(".*_HTML")
        .whitelist_var(".*_PNG")
        .whitelist_var(".*_SVG")
        .whitelist_var(".*_JSON")
        .whitelist_var("IDS_.*")
        .whitelist_var("CONTENT_INVALID_.*")
        .blacklist_type("HINSTANCE")
        .blacklist_type("HINSTANCE__")
        .bitfield_enum("cef_event_flags_t")
        .bitfield_enum("cef_file_dialog_mode_t")
        .bitfield_enum("cef_drag_operations_mask_t")
        .parse_callbacks(Box::new(ParseCallbacks()))
        .derive_copy(false)
        .derive_debug(false)
        .derive_hash(false)
        .derive_default(false)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}